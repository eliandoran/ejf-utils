use freetype::{Library, Face, face::LoadFlag, Bitmap};
use image::{DynamicImage, ImageBuffer, ImageFormat};
use viuer::Config;

const FONT_PATH: &'static str = "./fonts/Roboto/Roboto-Light.ttf";
const FACE_CHAR_WIDTH: isize = 8 * 64;
const FACE_HORIZONTAL_RESOLUTION: u32 = 100;

const OUTPUT_WIDTH: u32 = 32;
const OUTPUT_HEIGHT: u32 = 24;

fn get_pixels(bitmap: Bitmap, x: i32, y: i32) -> DynamicImage {
    let mut figure = ImageBuffer::new(OUTPUT_WIDTH as u32, OUTPUT_HEIGHT as u32);
    let mut p = 0;
    let mut q = 0;
    let width = bitmap.width() as usize;
    let x_max: u32 = (x + width as i32) as u32;
    let y_max: u32 = (y + bitmap.rows() as i32) as u32;

    for i in (x as u32)..x_max {
        for j in (y as u32)..y_max {
            if i < OUTPUT_WIDTH && j < OUTPUT_HEIGHT {
                figure[(i,j)] = image::Luma([bitmap.buffer()[q * width + p]]);
                q += 1;
            }
        }
        q = 0;
        p += 1;
    }

    let mut image = DynamicImage::ImageLuma8(figure);
    image.invert();
    image
}

fn render_single_character(face: &Face, ch: char) {
    // Try to render a single character.
    face.load_char(ch as usize, LoadFlag::RENDER)
        .expect("Unable to load one of the characters for rendering.");

    let glyph = face.glyph();
    let x = glyph.bitmap_left();
    let y = glyph.bitmap_top();
    
    println!("Bitmap position: char='{}', x={}, y={}", ch, x, y);

    // Get the pixels of that single character.
    let img = get_pixels(glyph.bitmap(), x, y);    

    // Save the output to png.
    img.save_with_format("output.png", ImageFormat::Png).unwrap();

    let config = Config {
        absolute_offset: false,
        ..Default::default()
    };

    viuer::print(&img, &config)
        .expect("Image printing failed.");
}

fn main() {
    // Try to open the font.
    let library = Library::init().unwrap();
    let face: Face = library
        .new_face(FONT_PATH, 0)
        .expect("Unable to load the font file to be used for rendering.\nPlease check the path to the font, permissions or that the file format is supported.");

    println!("Font family: {}", face.family_name().unwrap());

    // Set face properties.
    face.set_char_size(FACE_CHAR_WIDTH, 0, FACE_HORIZONTAL_RESOLUTION, 0)
        .expect("Unable to set the character size.");

    for ch in 'a'..'z' {
        render_single_character(&face, ch);
    }    
}

