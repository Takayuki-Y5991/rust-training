use clap::{Arg, ArgAction, Command};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

type ResultType<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> ResultType<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("konkon")
        .about("Rust head")
        .arg(
            Arg::new("files")
                .help("input file path")
                .required(true)
                .default_value("")
                .num_args(1..)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("bytes")
                .help("Print bytes of each of the specified files.")
                .short('c')
                .long("bytes")
                .conflicts_with("count")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("count")
                .help("Print count lines of each of the specified files.")
                .short('n')
                .long("lines")
                .default_value("10")
                .action(ArgAction::Set),
        )
        .get_matches();
    let lines = matches
        .get_one::<String>("count")
        .map(|s| s.as_str())
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))
        .unwrap()
        .unwrap();

    let bytes = matches
        .get_one("bytes")
        .cloned()
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal bytes -- {}", e))
        .unwrap();
    Ok(Config {
        files: matches.get_many("files").unwrap().cloned().collect(),
        lines,
        bytes,
    })
}
pub fn run(config: Config) -> ResultType<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(_) => println!("Opened {}", filename),
        }
    }
    Ok(())
}

fn parse_positive_int(val: &str) -> ResultType<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    // 正常値
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);
    // 数値変換できない文字列
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

fn open(filename: &str) -> ResultType<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
