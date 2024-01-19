use clap::{Arg, ArgAction, Command};
use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufRead, BufReader},
};

type ResultType<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> ResultType<()> {
    for file_name in config.files {
        match open(&file_name) {
            Err(err) => eprintln!("Failed to open {}: {}", file_name, err),
            Ok(file) => {
                let mut last_num = 0;
                for (line_number, line_result) in file.lines().enumerate() {
                    let line = line_result?;

                    if config.number_lines {
                        println!("{:>6}\t{}", line_number + 1, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, line);
                        } else {
                            println!("");
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> ResultType<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("konkon")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .help("Input file path")
                .default_value("-")
                .num_args(0..)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .short_alias('n')
                .help("Number lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::new("number_nonblank")
                .short_alias('b')
                .short('b')
                .long("number-nonblank")
                .help("Number nonBlank lines")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    Ok(Config {
        files: matches.get_many("files").unwrap().cloned().collect(),
        number_lines: matches.get_flag("number"),
        number_nonblank_lines: matches.get_flag("number_nonblank"),
    })
}

fn open(filename: &str) -> ResultType<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
