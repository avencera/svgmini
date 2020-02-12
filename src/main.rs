use clap::{App, AppSettings, Arg};
use regex::Captures;
use std::fs;
use svg_mini::defaults::SVGRE;

fn main() {
    let matches = App::new("SVGMini")
        .version("1.0")
        .author("Praveen Perera <praveen@avencera.com>")
        .about("Minify SVGs inside other files")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("FILE")
                .value_name("FILE")
                .help("File path with SVGs you would like to minify")
                .index(1)
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let file_path = matches.value_of("FILE").expect("Invalid File Contents");

    let file_contents = fs::read_to_string(file_path).unwrap();

    let minified_contents = SVGRE
        .replace_all(&file_contents, |caps: &Captures| {
            format!("{}", svg_mini::minify_svg(&caps[0]),)
        })
        .to_string();

    fs::write(file_path, minified_contents.as_bytes()).unwrap()
}
