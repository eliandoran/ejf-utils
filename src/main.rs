use std::{fs, process::exit, env::{args, set_current_dir}, path::Path};
use serde::{Deserialize};

mod ejf;
mod char_range;
use char_range::char_range;
use ejf::{EjfConfig, Error, build_ejf};

#[derive(Debug, Deserialize)]
struct Config {
    font: Vec<EjfConfig>
}

fn print_usage() {
    println!("Usage: /path/to/config.toml");
}

fn chdir(path: String) -> bool {
    let root = Path::new(&path).parent();
    if root.is_none() {
        return false;
    }

    set_current_dir(&root.unwrap()).is_err()
}

fn generate_fonts(config_path: String) {
    // Read the configuration file.
    let file_data = fs::read_to_string(&config_path);

    if file_data.is_err() {
        println!("Unable to open the configuration file at '{}'.\nPlease check that the input file exists and is accessible.", &config_path);
        exit(2);
    }

    // Parse the configuration file as TOML.
    let file_data = file_data.unwrap();
    let config: Result<Config, toml::de::Error> = toml::from_str(&file_data);

    if config.is_err() {
        println!("Unable to parse the configuration file: {}.", config.unwrap_err().to_string());
        exit(1)
    }

    // Change the working directory.
    chdir(config_path);

    // Generate each font.
    for font in config.unwrap().font {
        let result = build_ejf(&font);
    
        let message: String = match result {
            Ok(result) => format!("{}: height={}px", result.name, result.height),
            Err(error) => match error {
                Error::FreeTypeError(_) => "Unable to initialize FreeType.".to_string(),
                Error::ImageError(_) => "Unable to generate the image files for one or more characters.".to_string(),
                Error::IoError(e) => format!("Unable to read or write the .ejf file at '{}': {}", &font.output, e.to_string()),
                Error::MetricsError => "Unable to determine the metrics for one or more characters.".to_string(),
                Error::XmlWriterError(_) => "Unable to write the header XML file.".to_string(),
                Error::ZipWriterError(_) => "Unable to file the ZIP file (.ejf).".to_string(),
                Error::RangeParseError(e) => format!("Unable to parse the given character range: {}", e.message),
                Error::NameError => "Unable to determine the name of the resulting font (.ejf) based on the path.".to_string()
            }
        };
    
        println!("{}", message);
    }
}

fn main() {    
    match args().nth(1) {
        Some(config_path) => generate_fonts(config_path),
        None => print_usage()
    }
}
