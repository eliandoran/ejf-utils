use quick_xml::Error as XmlError;
use binstall_zip::result::ZipError;
use std::io::Error as IoError;
use image::ImageError;
use freetype::Error as FreeTypeError;
use super::char_range::ParseError as RangeParseError;

pub enum Error {

    /// Error when writing the XML Header of the .ejf file.
    XmlWriterError(XmlError),

    /// Error when writing the zip container of the .ejf data.
    ZipWriterError(ZipError),

    /// Input/output error when writing the zip/ejf file.
    IoError(IoError),

    /// Unable to determine the name of the resulting font (.ejf) based on the path.
    NameError,

    /// Error while generating character images.
    ImageError(ImageError),

    /// Error while initializing or using the FreeType engine.
    FreeTypeError(FreeTypeError),

    /// Error when parsing the list of characters to be imported.
    RangeParseError(RangeParseError),

    /// Unable to determine the character metrics.
    MetricsError

}

impl From<XmlError> for Error {
    #[inline]
    fn from(error: XmlError) -> Self {
        Error::XmlWriterError(error)
    }
}

impl From<ZipError> for Error {
    #[inline]
    fn from(error: ZipError) -> Self {
        Error::ZipWriterError(error)
    }
}

impl From<IoError> for Error {
    #[inline]
    fn from(error: IoError) -> Self {
        Error::IoError(error)
    }
}

impl From<ImageError> for Error {
    #[inline]
    fn from(error: ImageError) -> Self {
        Error::ImageError(error)
    }
}

impl From<FreeTypeError> for Error {
    #[inline]
    fn from(error: FreeTypeError) -> Self {
        Error::FreeTypeError(error)
    }
}

impl From<RangeParseError> for Error {
    #[inline]
    fn from(error: RangeParseError) -> Self {
        Error::RangeParseError(error)
    }
}