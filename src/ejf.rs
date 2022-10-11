use binstall_zip::{ZipWriter, write::FileOptions, CompressionMethod};
use freetype::{Library, Face};
use image::ImageFormat;
use indicatif::ProgressBar;
use serde::{Serialize, Deserialize};
use self::{header::HeaderInfo, metrics::determine_metrics_from_font};

use super::char_range;

mod errors;
mod header;
mod renderer;
mod metrics;

use std::{fs::File, io::{Write, Cursor}, path::Path};
pub use crate::ejf::errors::Error;

const DEFAULT_DPI: u32 = 72;
const PRINT_CHARACTERS: bool = false;

#[derive(Debug, Serialize, Deserialize)]
pub struct EjfConfig {
    pub input: String,
    pub output: String,
    pub size: u32,
    pub char_range: String,
    pub skip_control_characters: bool,
    pub dpi: Option<u32>
}

pub struct EjfResult {
    pub height: u32,
    pub name: String
}

/// Determine font name (same as the path, minus extension).
fn get_font_name(output_name: &String) -> Result<String, Error> {
    let path = Path::new(output_name);
    match path.file_stem() {
        Some(path) => Ok(path.to_string_lossy().to_string()),
        None => Err(Error::NameError)
    }
}

pub fn build_ejf(config: EjfConfig) -> Result<EjfResult, Error> {
    let font_name = get_font_name(&config.output)?;

    // Parse the character range from the config.
    let chars = char_range(config.char_range)?;

    // Open the output file
    let zip_file = File::create(&config.output)?;
    let mut zip = ZipWriter::new(zip_file);

    // Try to open the font.
    let library = Library::init()?;
    let face: Face = library.new_face(config.input, 0)?;

    // Set face properties.
    let char_width = config.size as isize * 64;
    let dpi = config.dpi.unwrap_or(DEFAULT_DPI);
    face.set_char_size(char_width, 0, dpi, 0)?;
    
    // Determine max height.
    let metrics = determine_metrics_from_font(&face)?;
    let image_height = metrics.height;

    // Render the characters.
    let bar = ProgressBar::new(chars.len() as u64);
    let zip_options = FileOptions::default()
        .compression_method(CompressionMethod::Stored);
    
    for code in &chars {
        let ch = char::from_u32(*code);

        // Skip unsupported characters.
        if ch.is_none() {
            continue;
        }

        let ch = ch.unwrap();

        if ch == ' ' {
            continue;
        }

        if config.skip_control_characters && ch.is_control() {
            continue;
        }

        let image = renderer::render_single_character(&face, ch as char, image_height, metrics.ascent);
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
    let header = header::write_header(HeaderInfo {
        chars: chars,
        height: image_height,
        name: font_name.to_string()
    })?;
    zip.start_file("Header", zip_options)?;
    zip.write(&header)?;
    zip.finish()?;

    Ok(EjfResult {
        height: image_height,
        name: font_name.to_string()
    })
}