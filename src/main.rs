use std::{fs, process::exit, env::{args, set_current_dir}, path::Path, thread::{self, JoinHandle}};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::{Deserialize};

mod ejf;
mod char_range;
use char_range::char_range;
use ejf::{EjfConfig, Error, build_ejf, EjfResult, get_font_name};

#[derive(Debug, Deserialize)]
struct Config {
    font: Vec<EjfConfig>
}

struct ThreadData {
    input: String,
    output: String,
    result: Result<EjfResult, Error>
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

    let spinner_style_progress = ProgressStyle::with_template("{prefix:32.bold.dim} {wide_bar:.cyan/blue} {pos:>5}/{len}").unwrap().tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    let spinner_style_done = ProgressStyle::with_template("{prefix:32.bold.dim} {msg}").unwrap();

    let fonts = config.unwrap().font;
    let num_fonts = fonts.len();
    let progress = MultiProgress::new();
    let mut threads: Vec<JoinHandle<ThreadData>> = Vec::new();
    let mut i = 0;
    for font in fonts {
        i += 1;
        let name = get_font_name(&font.output).unwrap_or_default();
        let pb = progress.add(ProgressBar::new(0));
        let style_done = spinner_style_done.clone();
        pb.set_style(spinner_style_progress.clone());
        pb.set_prefix(format!("[{}/{}] {}", i, num_fonts, name.to_string()));

        threads.push(thread::spawn(move || {            
            let result = build_ejf(&font,  | progress | {
                pb.set_length(progress.1 as u64);
                pb.set_position(progress.0 as u64);                
            });

            let status = match &result {
                Ok(result) => format!("Done, height: {}px.", &result.height),
                Err(_) => "Failed.".to_string()
            };

            pb.set_style(style_done);
            pb.finish_with_message(format!("{}", status));

            ThreadData {
                input: font.input,
                output: font.output,
                result
            }
        }));
    }

    for thread in threads {
        let data = thread.join().unwrap();

        let message: Option<String> = match data.result {
            Ok(_) => None,
            Err(error) => Some(match error {
                Error::FreeTypeError(_) => "Unable to initialize FreeType.".to_string(),
                Error::ImageError(_) => "Unable to generate the image files for one or more characters.".to_string(),
                Error::IoError(e) => format!("Unable to read or write the .ejf file at '{}': {}", &data.output, e.to_string()),
                Error::MetricsError => "Unable to determine the metrics for one or more characters.".to_string(),
                Error::XmlWriterError(_) => "Unable to write the header XML file.".to_string(),
                Error::ZipWriterError(_) => "Unable to file the ZIP file (.ejf).".to_string(),
                Error::RangeParseError(e) => format!("Unable to parse the given character range: {}", e.message),
                Error::NameError => "Unable to determine the name of the resulting font (.ejf) based on the path.".to_string()
            })
        };
    
        if message.is_some() {
            println!("{}: {}", data.input, message.unwrap());
        }
    }
}

fn main() {    
    println!("EJF Font Generator\n");

    match args().nth(1) {
        Some(config_path) => generate_fonts(config_path),
        None => print_usage()
    }
}
