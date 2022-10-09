use freetype::{Library, Face, face::LoadFlag, Bitmap};

const FONT_PATH: &'static str = "./fonts/Roboto/Roboto-Light.ttf";
const FACE_CHAR_WIDTH: isize = 1000;
const FACE_HORIZONTAL_RESOLUTION: u32 = 100;
const RENDER_CHAR: char = 'A';

const OUTPUT_WIDTH: usize = 32;
const OUTPUT_HEIGHT: usize = 64;

fn get_pixels(bitmap: Bitmap, x: usize, y: usize) -> [[u8; OUTPUT_WIDTH]; OUTPUT_HEIGHT] {
    let mut figure = [[0; OUTPUT_WIDTH]; OUTPUT_HEIGHT];
    let mut p: usize = 0;
    let mut q: usize = 0;
    let width: usize = bitmap.width() as usize;
    let x_max: usize = x + width;
    let y_max: usize = y + bitmap.rows() as usize;

    for i in x..x_max {
        for j in y..y_max {
            if i < OUTPUT_WIDTH && j < OUTPUT_HEIGHT {
                figure[j][i] |= bitmap.buffer()[q * width + p];
                q += 1;
            }
        }
        q = 0;
        p += 1;
    }
    figure
}

fn render_character(pixels: [[u8; OUTPUT_WIDTH]; OUTPUT_HEIGHT]) {
    for i in 0..OUTPUT_HEIGHT {
        for j in 0..OUTPUT_WIDTH {
            print!("{}",
                match pixels[i][j] {
                    p if p == 0 => " ",
                    p if p < 128 => "*",
                    _ => "+"
                }
            );
        }
        println!();
    }
}

fn main() {
    // Try to open the font.
    let library = Library::init().unwrap();
    let face: Face = library
        .new_face(FONT_PATH, 0)
        .expect("Unable to load the font file to be used for rendering.\nPlease check the path to the font, permissions or that the file format is supported.");

    // Set face properties.
    face.set_char_size(FACE_CHAR_WIDTH, 0, FACE_HORIZONTAL_RESOLUTION, 0)
        .expect("Unable to set the character size.");

    // Try to render a single character.
    face.load_char(RENDER_CHAR as usize, LoadFlag::RENDER)
        .expect("Unable to load one of the characters for rendering.");

    let glyph = face.glyph();
    let x = glyph.bitmap_left() as usize;
    let y = glyph.bitmap_top() as usize;
    
    println!("Font family: {}", face.family_name().unwrap());
    println!("Bitmap position: x={}, y={}", x, y);

    // Get the pixels of that single character.
    let pixels = get_pixels(glyph.bitmap(), x, y);
    render_character(pixels);
}

