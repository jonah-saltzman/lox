use clap::{Command, Arg};
use std::path::Path;
use jlox::{cli, JLox, LoxError, object::Object};
use anyhow::Result;

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
        let out = interpreter.handle_source(&source);
        handle_output(out);
        Ok(())
    } else {
        let reader = cli::StdinLines::new();
        for line in reader {
            let line = line?;
            let out = interpreter.handle_source(&line);
            handle_output(out);
            println!("");
            interpreter.reset_error();
        }
        Ok(())
    }
}

fn handle_output(out: Result<Object, LoxError>) {
    match out {
        Ok(obj) if obj.is_some() => println!("{}", obj),
        Err(e) => {
            let err = anyhow::Error::from(e);
            eprintln!("{}", err);
            let mut cause = err.source();
            let mut spaces = 2;
            while let Some(inner) = cause {
                let prefix: String = std::iter::repeat(' ').take(spaces).collect();
                eprintln!("{}caused by: {}", prefix, inner);
                cause = inner.source();
                spaces += 2;
            }
        },
        _ => {}
    }
}