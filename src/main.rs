use clap::{App, AppSettings, Arg, ArgMatches};
use failure::Error;
use regex::Captures;
use std::fs;
use svgmini::defaults::SVGRE;
use svgmini::options::Options;

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
        .arg(
            Arg::with_name("replace-fill")
                .short("r")
                .long("replace-fill")
                .help("Replace all fill values with 'currentColor'"),
        )
        .get_matches();

    std::process::exit(match run(&matches) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}

fn run(matches: &ArgMatches) -> Result<(), Error> {
    let options = Options::new_from_matches(&matches)?;

    let minified_contents = SVGRE.replace_all(&options.file_contents, |caps: &Captures| {
        svgmini::minify_svg(&caps[0], &options).unwrap_or_else(|_| caps[0].to_string())
    });

    fs::write(&options.file_path, minified_contents.as_bytes())?;

    Ok(())
}
