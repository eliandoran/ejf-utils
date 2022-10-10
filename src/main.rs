mod ejf;
mod char_range;
use char_range::char_range;
use ejf::{EjfConfig, Error, build_ejf};

fn main() {    
    let result = build_ejf(EjfConfig {
        char_range: "0x0, 0x40-0x50,0x60-0x80".to_string(),
        path: "./fonts/Roboto/Roboto-Light.ttf".to_string(),
        size: 25,
        skip_control_characters: false
    });

    let message: String = match result {
        Ok(_) => ".ejf file generated successfully.".to_string(),
        Err(error) => match error {
            Error::FreeTypeError(_) => "Unable to initialize FreeType.".to_string(),
            Error::ImageError(_) => "Unable to generate the image files for one or more characters.".to_string(),
            Error::IoError(_) => "Unable to read or write the .ejf file.".to_string(),
            Error::MetricsError => "Unable to determine the metrics for one or more characters.".to_string(),
            Error::XmlWriterError(_) => "Unable to write the header XML file.".to_string(),
            Error::ZipWriterError(_) => "Unable to file the ZIP file (.ejf).".to_string(),
            Error::RangeParseError(e) => format!("Unable to parse the given character range: {}", e.message)
        }
    };

    println!("{}", message);
}
