use clap::{Command, Arg};
use std::path::Path;
use jlox::{JLox, cli};

fn main() {
    let matches = Command::new("My App")
        .version("1.0")
        .author("Your Name")
        .about("Handles an optional file path argument")
        .arg(Arg::new("path")
            .help("The file path to check")
            .index(1)
            .required(false))
        .get_matches();

    let mut interpreter = JLox::new();

    if let Some(path) = matches.get_one::<String>("path") {
        let p = Path::new(path);
        if p.exists() {
            match cli::load_source(p) {
                Ok(source) => interpreter.handle_source(&source),
                Err(e) => println!("Err: {e}")
            }
        } else {
            println!("{} not found", path)
        }
    } else {
        let reader = cli::StdinLines::new();
        for line in reader {
            match line {
                Ok(source) => interpreter.handle_source(&source),
                Err(e) => println!("Err: {e}")
            }
        }
    }
}
