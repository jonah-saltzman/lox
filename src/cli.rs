use ascii::{AsciiString, AsAsciiStr, AsAsciiStrError};
use std::io::{self, BufRead, StdinLock, Read};
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SourceError {
    #[error("io error")]
    Io(#[from] io::Error),

    #[error("non ascii character")]
    Ascii(#[from] AsAsciiStrError)
}

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
    type Item = Result<AsciiString, SourceError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.lock.read_line(&mut line) {
            Ok(0) => None,
            Ok(_) => {
                match AsciiString::from_str(&line) {
                    Ok(ascii) => Some(Ok(ascii)),
                    Err(e) => Some(Err(SourceError::Ascii(e)))
                }
            },
            Err(e) => Some(Err(SourceError::Io(e))),
        }
    }
}

pub fn load_source(path: &Path) -> Result<AsciiString, SourceError> {
    let mut file = File::open(path)?;
    let mut buff: Vec<u8> = Vec::new();
    file.read_to_end(&mut buff)?;
    Ok(AsciiString::from(buff.as_ascii_str()?))
}
