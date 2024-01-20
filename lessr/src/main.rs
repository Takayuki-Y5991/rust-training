use clap::{Arg, ArgAction, Args, Command};

fn main() {
    if let Err(err) = lessr::get_args().and_then(lessr::run) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
