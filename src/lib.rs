use failure::{Error, Fail};
use svgcleaner;
use svgdom;

pub mod defaults;
pub mod options;
#[derive(Debug, Fail)]
pub enum MinifyError {
    #[fail(display = "Unable to parse SVG")]
    UnableToParseSVG,
    #[fail(display = "Unable to save as UTF8")]
    UnableToSaveAsUTF8,
}

impl From<std::string::FromUtf8Error> for MinifyError {
    fn from(_error: std::string::FromUtf8Error) -> Self {
        MinifyError::UnableToSaveAsUTF8
    }
}

pub fn minify_svg(svg_text: &str) -> Result<String, Error> {
    let mut doc = svgdom::Document::from_str_with_opt(svg_text, &defaults::parse_options())
        .map_err(|_error| MinifyError::UnableToParseSVG)?;

    let _ = svgcleaner::cleaner::clean_doc(
        &mut doc,
        &defaults::cleaning_options(),
        &defaults::write_options(),
    );
    let mut buffer: Vec<u8> = vec![];
    svgcleaner::cleaner::write_buffer(&doc, &defaults::write_options(), &mut buffer);

    Ok(String::from_utf8(buffer)?)
}
