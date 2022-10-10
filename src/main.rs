use binstall_zip::{ZipWriter, write::FileOptions, CompressionMethod};
use freetype::{Library, Face, face::LoadFlag};
use image::ImageFormat;
use indicatif::ProgressBar;

use std::{fs::File, io::{Write, Cursor}};

mod header;
mod renderer;

const FONT_PATH: &'static str = "./fonts/Roboto/Roboto-Light.ttf";
const FACE_CHAR_WIDTH: isize = 25 * 64;
const FACE_HORIZONTAL_RESOLUTION: u32 = 100;

const SKIP_CONTROL_CHARACTERS: bool = true;
const PRINT_CHARACTERS: bool = false;

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

        let image = renderer::render_single_character(&face, ch as char, image_height, max_ascent);
        let mut cursor = Cursor::new(Vec::new());

        if PRINT_CHARACTERS {
            renderer::print_character(&image);
        }

        image.to_rgb8().write_to(&mut cursor, ImageFormat::Png).unwrap();
        
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
    let header = header::write_header(&vec, image_height);
    zip.start_file("Header", zip_options).unwrap();
    zip.write(&header).unwrap();
    zip.finish().unwrap();
}
