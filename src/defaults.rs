use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref SVGRE: Regex = Regex::new(r#"<svg[^>]*>((.|\n)*?)</svg>"#).unwrap();
}
