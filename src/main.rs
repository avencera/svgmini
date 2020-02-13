use clap::{App, AppSettings, Arg, ArgMatches};
use regex::Captures;
use std::fs;
use std::io;
use std::io::{Error, ErrorKind};
use svgmini::defaults::SVGRE;

fn main() {
    let matches = App::new("SVGMini")
        .version(clap::crate_version!())
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

    let file_contents = get_and_read_file(&matches);

    match file_contents {
        Ok((file_path, file_contents)) => {
            let minified_contents = SVGRE.replace_all(&file_contents, |caps: &Captures| {
                svgmini::minify_svg(&caps[0]).unwrap_or(caps[0].to_string())
            });

            fs::write(&file_path, minified_contents.as_bytes()).ok();
        }

        Err(error) => println!("{}", error),
    }
}

fn get_and_read_file<'a>(matches: &'a ArgMatches) -> Result<(&'a str, String), io::Error> {
    let file_path = matches
        .value_of("FILE")
        .ok_or(Error::new(ErrorKind::InvalidInput, "Unable to find file"))?;

    let file_contents = fs::read_to_string(file_path)?;
    Ok((file_path, file_contents))
}
