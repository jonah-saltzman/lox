use std::{io::{self, BufRead, Error, StdinLock, Read}, path::Path};
use std::fs::File;

pub struct StdinLines {
    lock: StdinLock<'static>
}

impl StdinLines {
    pub fn new() -> StdinLines {
        let stdin = io::stdin();
        let lock = stdin.lock();
        StdinLines {
            lock
        }
    }
}

impl Iterator for StdinLines {
    type Item = Result<String, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.lock.read_line(&mut line) {
            Ok(0) => None,
            Ok(_) => {
                Some(Ok(line.trim_end().to_string()))
            },
            Err(e) => Some(Err(e)),
        }
    }
}

pub fn load_source(path: &Path) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}