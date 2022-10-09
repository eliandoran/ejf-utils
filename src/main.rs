use freetype::{Library, Face};

const FONT_PATH: &'static str = "./fonts/Roboto/Roboto-Light.ttf";

const ERROR_UNABLE_TO_LOAD_FONT_FILE: &'static str = "Unable to load the font file to be used for rendering.\nPlease check the path to the font, permissions or that the file format is supported.";

fn main() {
    // Try to open the font.
    let library = Library::init().unwrap();
    let face: Face = library
        .new_face(FONT_PATH, 0)
        .expect(ERROR_UNABLE_TO_LOAD_FONT_FILE);

    println!("Font family: {}", face.family_name().unwrap());
}

