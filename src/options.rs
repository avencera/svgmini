use clap::ArgMatches;
use failure::{Error, Fail};
use std::fs;
pub struct Options {
    pub file_path: String,
    pub file_contents: String,
    pub replace_fill: bool,
}

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

impl Options {
    pub fn new_from_matches(matches: &ArgMatches) -> Result<Options, Error> {
        let (file_path, file_contents) = get_and_read_file(matches)?;

        Ok(Options {
            file_path: file_path,
            file_contents: file_contents,
            replace_fill: matches.is_present("replace-fill"),
        })
    }
}

fn get_and_read_file(matches: &ArgMatches) -> Result<(String, String), Error> {
    let file_path = matches.value_of("FILE").ok_or(CLIError::UnableToFindFile)?;

    let file_contents = fs::read_to_string(file_path)?;
    Ok((file_path.to_string(), file_contents))
}
