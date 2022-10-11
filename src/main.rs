use std::{fs, process::exit};
use serde::{Deserialize};

mod ejf;
mod char_range;
use char_range::char_range;
use ejf::{EjfConfig, Error, build_ejf};

#[derive(Debug, Deserialize)]
struct Config {
    font: Vec<EjfConfig>
}

fn main() {    
    let file_data = fs::read_to_string("input.toml").unwrap();
    let config: Result<Config, toml::de::Error> = toml::from_str(&file_data);

    if config.is_err() {
        println!("Unable to parse the configuration file: {}.", config.unwrap_err().to_string());
        exit(1)
    }

    for font in config.unwrap().font {
        let result = build_ejf(font);
    
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
}
