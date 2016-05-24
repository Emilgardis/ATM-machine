extern crate rustbox;

use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key as RKey;
pub enum Key {
    /// Implements the keys used,
    Right,
    Left,
    Up,
    Down,
    Char(char),
    F1,
    F2,
    ESC,
}

struct TermBoxUI<'a> {
    rustbox: &'a RustBox,
}

impl<'a> TermBoxUI<'a> {
    fn wait_key(&self) -> Option<Key> {
        match self.rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                	Some(RKey::Up) => Some(Key::Up),
          			Some(RKey::Down) => Some(Key::Down),
          			Some(RKey::Left) => Some(Key::Left),
          			Some(RKey::Right) => Some(Key::Right),
                    _ => None,
                }
            },
            Err(e) => panic!("{}", e),
        }
    }
}
/*
fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Black, "Hello, world!");
    rustbox.print(1, 3, rustbox::RB_BOLD, Color::White, Color::Black,
                  "Press 'q' to quit.");
    rustbox.present();
    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
    }
}*/
