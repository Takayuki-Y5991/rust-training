use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
        .arg(
            Arg::new("text")
                .help("Input text")
                .required(true)
                .num_args(1..)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("omit_newline")
                .help("Do not print new lines")
                .short('n')
                .short_alias('n')
                .action(ArgAction::SetTrue),
        )
        .version("0.1.0")
        .get_matches();

    let text: Vec<String> = matches.get_many("text").unwrap().cloned().collect();
    let omit_newline = matches.get_flag("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
