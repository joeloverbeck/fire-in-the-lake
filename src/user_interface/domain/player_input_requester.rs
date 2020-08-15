use std::io::{stdout, Write};

extern crate termcolor;

pub struct PlayerInputRequester {}

impl Default for PlayerInputRequester {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> PlayerInputRequester {
    pub fn new() -> PlayerInputRequester {
        PlayerInputRequester {}
    }

    pub fn request(&self, text: &str) -> Result<String, String> {
        println!();

        print!("{}", text);

        stdout().flush().unwrap();

        let mut input = String::new();

        if let Err(error) = std::io::stdin().read_line(&mut input) {
            return Err(error.to_string());
        }

        Ok(input.trim_end().to_string())
    }
}
