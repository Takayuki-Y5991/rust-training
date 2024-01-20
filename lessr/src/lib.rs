use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Empty, Write},
};

use clap::{Arg, ArgAction, Command};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

type ResultType<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file: String,
    line_number: bool,
}

pub fn get_args() -> ResultType<Config> {
    let matches = Command::new("lessr")
        .version("0.1.0")
        .author("konkon")
        .about("Less Command for Rust")
        .arg(Arg::new("file").help("file name").action(ArgAction::Set))
        .arg(
            Arg::new("pattern")
                .help("Start at pattern (from command line).")
                .short('p')
                .long("pattern")
                .short_alias('p')
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("line_number")
                .short('N')
                .long("LINE-NUMBERS")
                .short_alias('N')
                .action(ArgAction::SetTrue),
        )
        .get_matches();
    Ok(Config {
        file: matches.get_one("file").cloned().unwrap(),
        line_number: matches.get_flag("line_number"),
    })
}

fn open(filename: &str) -> ResultType<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> ResultType<()> {
    if !config.file.is_empty() {
        match open(&config.file) {
            Err(err) => eprint!("Failed to open {} : {}", config.file, err),
            Ok(fileBuf) => {
                // terminal settings
                let mut stdout = io::stdout();
                execute!(stdout, EnterAlternateScreen, Clear(ClearType::All))?;

                // display setting
                let mut lines: Vec<String> = Vec::new();
                for (line_number, line_result) in fileBuf.lines().enumerate() {
                    let line = line_result?;
                    if config.line_number {
                        lines.push(format!("{:>6}\t{}", line_number + 1, line));
                    } else {
                        lines.push(line)
                    }
                }

                let mut page = 0;
                let (_, height) = terminal::size()?;

                let page_size = height as usize - 1;

                let mut display_page = |page: usize| -> io::Result<()> {
                    execute!(stdout, Clear(ClearType::All))?;
                    for i in 0..page_size {
                        if let Some(line) = lines.get(page * page_size + i) {
                            writeln!(stdout, "{}", line)?;
                        }
                    }
                    stdout.flush()?;
                    Ok(())
                };
                display_page(page)?;

                loop {
                    if event::poll(std::time::Duration::from_millis(500))? {
                        if let Event::Key(key_event) = event::read()? {
                            match key_event.code {
                                KeyCode::Char('q') => {
                                    break;
                                }
                                KeyCode::Down | KeyCode::Char(' ') => {
                                    if page < lines.len() / page_size {
                                        page += 1;
                                        display_page(page)?;
                                    }
                                }
                                KeyCode::Up => {
                                    if page > 0 {
                                        page -= 1;
                                        display_page(page)?;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                execute!(stdout, LeaveAlternateScreen)?;
            }
        }
    } else {
        println!("{}", "--help Display help (from command line)")
    }
    Ok(())
}
