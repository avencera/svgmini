use clap::{App, AppSettings, Arg, ArgMatches};
use failure::{Error, Fail};
use regex::Captures;
use std::fs;
use svgmini::defaults::SVGRE;

#[derive(Debug, Fail)]
enum CLIError {
    #[fail(display = "Unable to find file")]
    UnableToFindFile,

    #[fail(display = "Unable to read file")]
    UnableToReadFile,
}

impl From<std::io::Error> for CLIError {
    fn from(_error: std::io::Error) -> Self {
        CLIError::UnableToReadFile
    }
}

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
                svgmini::minify_svg(&caps[0]).unwrap_or_else(|_| caps[0].to_string())
            });

            fs::write(&file_path, minified_contents.as_bytes()).ok();
        }

        Err(error) => println!("{}", error),
    }
}

fn get_and_read_file<'a>(matches: &'a ArgMatches) -> Result<(&'a str, String), Error> {
    let file_path = matches.value_of("FILE").ok_or(CLIError::UnableToFindFile)?;

    let file_contents = fs::read_to_string(file_path)?;
    Ok((file_path, file_contents))
}
