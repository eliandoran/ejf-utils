use binstall_zip::{ZipWriter, write::FileOptions, CompressionMethod};
use freetype::{Library, Face, face::LoadFlag, Bitmap};
use image::{DynamicImage, ImageBuffer, ImageFormat, RgbImage};
use indicatif::ProgressBar;
use quick_xml::Writer;
use viuer::Config;
use core::{cmp::max};
use std::{fs::File, io::{Write, Cursor}};

const FONT_PATH: &'static str = "./fonts/Roboto/Roboto-Light.ttf";
const FACE_CHAR_WIDTH: isize = 25 * 64;
const FACE_HORIZONTAL_RESOLUTION: u32 = 100;

const SKIP_CONTROL_CHARACTERS: bool = true;
const PRINT_CHARACTERS: bool = false;

fn get_pixels(bitmap: Bitmap, image_height: u32, offset_y: i32) -> DynamicImage {    
    let width = bitmap.width() as usize;
    let image_width = max(1, width as u32); // 0px width images are not allowed.
    let mut figure = ImageBuffer::new(image_width, image_height);
    
    for cx in 0..width {
        for cy in 0..bitmap.rows() as usize {
            let pos = (cy) * width + (cx);
            let pixel = [ bitmap.buffer()[pos] ];

            let dest_y = cy as i32 + offset_y;
            if dest_y >= 0 && dest_y < image_height as i32 {
                figure[(cx as u32, dest_y as u32)] = image::Luma(pixel);
            }
        }
    }

    let mut image = DynamicImage::ImageLuma8(figure);
    image.invert();
    image 
}

fn render_single_character(face: &Face, ch: char, image_height: u32, max_ascent: u32) -> RgbImage {
    // Try to render a single character.
    face.load_char(ch as usize, LoadFlag::RENDER)
        .expect("Unable to load one of the characters for rendering.");

    let glyph = face.glyph();

    // Get the pixels of that single character.
    let offset_y = max_ascent as i32 - (glyph.bitmap_top() as i32);
    let img = get_pixels(glyph.bitmap(), image_height, offset_y);    

    if PRINT_CHARACTERS {
        let config = Config {
            absolute_offset: false,
            ..Default::default()
        };

        viuer::print(&img, &config)
            .expect("Image printing failed.");
    }

    img.to_rgb8()
}

fn write_header(chars: &[char], height: u32) -> Vec<u8> {
    let mut writer = Writer::new(Vec::new());
    writer.create_element("FontGenerator")
        .write_inner_content(|writer| {
            writer.create_element("Informations")
                .with_attribute(("Vendor", "IS2T"))
                .with_attribute(("Version", "0.8"))
                .write_empty()?;

            writer.create_element("FontProperties")
                .with_attribute(("Baseline", "13"))
                .with_attribute(("Filter", "u"))
                .with_attribute(("Height", height.to_string().as_str()))
                .with_attribute(("Name", "Foo"))
                .with_attribute(("Space", "5"))
                .with_attribute(("Style", "pu"))
                .with_attribute(("Width", "-1"))
                .write_inner_content(|writer| {
                    writer.create_element("Identifier")
                        .with_attribute(("Value", "34"))
                        .write_empty()?;
                    Ok(())
                })?;

            writer.create_element("FontCharacterProperties")
                .write_inner_content(|writer| {
                    for ch in chars.iter() {
                        let index = format!("0x{:x}", (*ch) as u8); 
                        writer.create_element("Character")
                            .with_attribute(("Index", index.as_str()))
                            .with_attribute(("LeftSpace", "0"))
                            .with_attribute(("RightSpace", "0"))
                            .write_empty()?;
                    }

                    Ok(())
                })?;
            Ok(())
        }).unwrap();
    writer.into_inner()
}

fn main() {
    // Open the output file
    let zip_file = File::create("output/font.ejf").unwrap();
    let mut zip = ZipWriter::new(zip_file);

    // Try to open the font.
    let library = Library::init().unwrap();
    let face: Face = library
        .new_face(FONT_PATH, 0)
        .expect("Unable to load the font file to be used for rendering.\nPlease check the path to the font, permissions or that the file format is supported.");    

    // Set face properties.
    face.set_char_size(FACE_CHAR_WIDTH, 0, FACE_HORIZONTAL_RESOLUTION, 0)
        .expect("Unable to set the character size.");

    // Determine ascent, descent, image height
    let y_scale = face.size_metrics().unwrap().y_scale as f32;
    let max_ascent = (face.ascender() as f32 * (y_scale / 65536.0)) as u32 >> 6;
    
    // Determine max height.
    let mut max_descent: i32 = 0;
    for code in (0 as u8)..(255 as u8) {
        face.load_char(code as usize, LoadFlag::RENDER)
            .expect("Unable to load character.");
        let glyph = face.glyph();
        let cur_descent = (glyph.metrics().height >> 6) as i32 - glyph.bitmap_top();

        if cur_descent > max_descent {
            max_descent = cur_descent;
        }
    }

    let image_height = (max_ascent as i32 + max_descent) as u32;

    println!("Font family: {}", face.family_name().unwrap());
    println!("The characters will have a height of: {}px.", image_height);

    // Render the characters.
    let bar = ProgressBar::new(256);
    let mut vec = Vec::<char>::new();
    let zip_options = FileOptions::default()
        .compression_method(CompressionMethod::Stored);
    
    for code in (0 as u8)..(255 as u8) {
        let ch = code as char;

        if SKIP_CONTROL_CHARACTERS && ch.is_control() {
            continue;
        }

        let image = render_single_character(&face, ch as char, image_height, max_ascent);
        let mut cursor = Cursor::new(Vec::new());
        
        image.write_to(&mut cursor, ImageFormat::Png).unwrap();
        
        // Write the character to the zip file
        let char_code = format!("0x{:x}", ch as u32);
        let image_data = cursor.into_inner();
        zip.start_file(&char_code, zip_options).unwrap();
        zip.write(&image_data).unwrap();

        // Also write the "design" character to the zip file.
        zip.start_file(format!("design_{}", &char_code), zip_options).unwrap();
        zip.write(&image_data).unwrap();

        vec.push(ch);
        bar.inc(1);
    }    

    // Write the header
    let header = write_header(&vec, image_height);
    zip.start_file("Header", zip_options).unwrap();
    zip.write(&header).unwrap();
    zip.finish().unwrap();
}
