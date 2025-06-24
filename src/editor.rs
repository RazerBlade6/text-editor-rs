use crossterm::event::{Event::Key, KeyCode::Char, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

#[derive(Debug)]
pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(event) => {
                    if let event = Key(event) {
                        println!("{:?}\r", event)
                    }
                }
                Err(e) => panic!("{e}"),
            }
        }
    }
}
