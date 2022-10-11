use binstall_zip::{ZipWriter, write::FileOptions, CompressionMethod};
use freetype::{Library, Face, face::LoadFlag};
use image::ImageFormat;
use indicatif::ProgressBar;
use serde::{Serialize, Deserialize};
use super::char_range;

mod errors;
mod header;
mod renderer;

use std::{fs::File, io::{Write, Cursor}};
pub use crate::ejf::errors::Error;

const DEFAULT_DPI: u32 = 72;
const PRINT_CHARACTERS: bool = false;

#[derive(Debug, Serialize, Deserialize)]
pub struct EjfConfig {
    input: String,
    output: String,
    size: i8,
    char_range: String,
    skip_control_characters: bool,
    dpi: Option<u32>
}

fn determine_max_ascend(face: &Face)  -> Result<u32, Error> {
    let metrics = face.size_metrics().ok_or(Error::MetricsError);
    let y_scale = metrics?.y_scale as f32;
    Ok((face.ascender() as f32 * (y_scale / 65536.0)) as u32 >> 6)
}

fn determine_max_height(face: &Face, chars: &[u8], max_ascent: u32) -> Result<u32, Error> {    
    let mut max_descent: u32 = 0;
    for code in chars {
        face.load_char(*code as usize, LoadFlag::RENDER)
            .expect("Unable to load character.");
        let glyph = face.glyph();
        let height = (glyph.metrics().height >> 6) as i32;
        let top = glyph.bitmap_top();

        if height >= top {
            let cur_descent = (height - glyph.bitmap_top()) as u32;
    
            if cur_descent > max_descent {
                max_descent = cur_descent;
            }
        }
    }

    Ok((max_ascent + max_descent) as u32)
}

pub fn build_ejf(config: EjfConfig) -> Result<(), Error> {
    // Parse the character range from the config.
    let chars = char_range(config.char_range)?;

    // Open the output file
    let zip_file = File::create(config.output)?;
    let mut zip = ZipWriter::new(zip_file);

    // Try to open the font.
    let library = Library::init()?;
    let face: Face = library.new_face(config.input, 0)?;

    // Set face properties.
    let char_width = config.size as isize * 64;
    let dpi = config.dpi.unwrap_or(DEFAULT_DPI);
    face.set_char_size(char_width, 0, dpi, 0)?;
    
    // Determine max height.
    let max_ascent = determine_max_ascend(&face)?;
    let image_height = determine_max_height(&face, &chars, max_ascent)?;

    println!("Font family: {}", face.family_name().unwrap_or_default());
    println!("The characters will have a height of: {}px.", image_height);

    // Render the characters.
    let bar = ProgressBar::new(chars.len() as u64);
    let zip_options = FileOptions::default()
        .compression_method(CompressionMethod::Stored);
    
    for code in &chars {
        let ch = *code as char;

        if config.skip_control_characters && ch.is_control() {
            continue;
        }

        let image = renderer::render_single_character(&face, ch as char, image_height, max_ascent);
        let mut cursor = Cursor::new(Vec::new());

        if PRINT_CHARACTERS {
            renderer::print_character(&image);
        }

        image.to_rgb8().write_to(&mut cursor, ImageFormat::Png)?;
        
        // Write the character to the zip file
        let char_code = format!("0x{:x}", ch as u32);
        let image_data = cursor.into_inner();
        zip.start_file(&char_code, zip_options)?;
        zip.write(&image_data)?;        

        // Also write the "design" character to the zip file.
        zip.start_file(format!("design_{}", &char_code), zip_options)?;
        zip.write(&image_data)?;

        bar.inc(1);
    }    

    // Write the header
    let header = header::write_header(&chars, image_height)?;
    zip.start_file("Header", zip_options)?;
    zip.write(&header)?;
    zip.finish()?;

    Ok(())
}