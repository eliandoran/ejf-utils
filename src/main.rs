mod ejf;
use ejf::{EjfConfig, Error, build_ejf};

fn main() {
    let result = build_ejf(EjfConfig {
        path: "./fonts/Roboto/Roboto-Light.ttf".to_string(),
        size: 25,
        skip_control_characters: true
    });

    let message = match result {
        Ok(_) => ".ejf file generated successfully.",
        Err(error) => match error {
            Error::FreeTypeError(_) => "Unable to initialize FreeType.",
            Error::ImageError(_) => "Unable to generate the image files for one or more characters.",
            Error::IoError(_) => "Unable to read or write the .ejf file.",
            Error::MetricsError => "Unable to determine the metrics for one or more characters.",
            Error::XmlWriterError(_) => "Unable to write the header XML file.",
            Error::ZipWriterError(_) => "Unable to file the ZIP file (.ejf)."
        }
    };

    println!("{}", message);
}
