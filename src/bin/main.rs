use clap::{Command, Arg};
use std::path::Path;
use jlox::{JLox, cli};
use anyhow::{Ok, Result};

fn main() -> Result<()> {
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
        let source = cli::load_source(p)?;
        let out = interpreter.handle_source(&source)?;
        println!("{:?}", out);
        Ok(())
    } else {
        let reader = cli::StdinLines::new();
        for line in reader {
            let line = line?;
            let out = interpreter.handle_source(&line);
            println!("{:?}", out);
            interpreter.reset_error();
        }
        Ok(())
    }
}
