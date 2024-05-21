pub mod cli;

pub struct JLox {}

impl JLox {

    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_source(&mut self, source: &str) {
        println!("{}", source)
    }
}