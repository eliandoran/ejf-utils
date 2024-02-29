use zip::{ZipWriter, write::FileOptions, CompressionMethod};
use freetype::{Library, Face};
use image::ImageFormat;
use serde::{Serialize, Deserialize};
use self::{header::HeaderInfo, metrics::determine_metrics_from_font, renderer::RenderConfig};

use super::char_range;

mod errors;
mod header;
mod renderer;
mod metrics;

use std::{fs::File, io::{Write, Cursor}, path::Path};
pub use crate::ejf::errors::Error;

const DEFAULT_DPI: u32 = 72;
const DEFAULT_LEFT_SPACING: u8 = 0;
const DEFAULT_RIGHT_SPACING: u8 = 1;
const PRINT_CHARACTERS: bool = false;

#[derive(Debug, Serialize, Deserialize)]
pub struct EjfConfig {
    pub input: String,
    pub output: String,
    pub size: u32,
    pub char_range: String,
    pub skip_control_characters: bool,
    pub add_null_character: Option<bool>,
    pub dpi: Option<u32>,
    pub left_spacing: Option<u8>,
    pub right_spacing: Option<u8>
}

pub struct EjfResult {
    pub height: u32,
    pub name: String
}

/// Determine font name (same as the path, minus extension).
pub fn get_font_name(output_name: &String) -> Result<String, Error> {
    let path = Path::new(output_name);
    match path.file_stem() {
        Some(path) => Ok(path.to_string_lossy().to_string()),
        None => Err(Error::NameError)
    }
}

pub fn build_ejf<F>(config: &EjfConfig, progress_callback: F) -> Result<EjfResult, Error>
    where F: Fn((i32, i32))
{
    let font_name = get_font_name(&config.output)?;

    // Parse the character range from the config.
    let chars = char_range(&config.char_range, config.skip_control_characters, config.add_null_character)?;

    // Open the output file
    let zip_file = File::create(&config.output)?;
    let mut zip = ZipWriter::new(zip_file);

    // Try to open the font.
    let library = Library::init()?;
    let face: Face = library.new_face(&config.input, 0)?;

    // Set face properties.
    let char_width = config.size as isize * 64;
    let dpi = config.dpi.unwrap_or(DEFAULT_DPI);
    face.set_char_size(char_width, 0, dpi, 0)?;
    
    // Determine max height.
    let metrics = determine_metrics_from_font(&face)?;
    let image_height = metrics.height;

    // Render the characters.
    let zip_options = FileOptions::default()
        .compression_method(CompressionMethod::Stored);
    
    let mut num_processed = 0;
    for ch in &chars {
        let image = renderer::render_single_character(&face, *ch, RenderConfig {
            left_spacing: config.left_spacing.unwrap_or(DEFAULT_LEFT_SPACING),
            right_spacing: config.right_spacing.unwrap_or(DEFAULT_RIGHT_SPACING),
            max_ascent: metrics.ascent,
            total_height: image_height
        });
        let mut cursor = Cursor::new(Vec::new());

        if PRINT_CHARACTERS {
            renderer::print_character(&image);
        }

        image.to_rgb8().write_to(&mut cursor, ImageFormat::Png)?;
        
        // Write the character to the zip file
        let char_code = format!("0x{:x}", *ch as u32);
        let image_data = cursor.into_inner();
        zip.start_file(&char_code, zip_options)?;
        zip.write(&image_data)?;        

        // Also write the "design" character to the zip file.
        zip.start_file(format!("design_{}", &char_code), zip_options)?;
        zip.write(&image_data)?;

        num_processed += 1;
        progress_callback((num_processed, chars.len() as i32));
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