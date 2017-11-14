extern crate serde;
extern crate serde_json;
extern crate termion;
#[macro_use]
extern crate lazy_static;

use termion::screen;
use termion::raw::IntoRawMode;
use termion::input::TermRead;

use std::io::Write;

#[macro_use]
mod entry;
mod settings;

use entry::Entry;
use settings::Settings;

lazy_static! {
    pub static ref SETTINGS: Settings = {
        // @Todo: configurability
        let (width, height) = termion::terminal_size().expect("can't get window size");
        Settings {
            complete_char: '*',
            incomplete_char: '-',

            width: width as usize,
            height: height as usize,
        }
    };
}

fn main() {
    let mut root: Vec<Entry> = Vec::new();

    root.push(entry!("Lists",
                     ["Bible",
                      "Shopping",
                      "Big speffffffffffffffffffffnds",
                      "Music",
                      "Books",
                      "Jokes"]));
    root.push(entry!("Projects"));
    root.push(entry!("St Barts"));
    root.push(entry!("Scratch"));

    let key = {
        let stdin = std::io::stdin();
        let stdout = std::io::stdout().into_raw_mode().unwrap();

        let mut screen = screen::AlternateScreen::from(stdout);
        for entry in root.iter() {
            // @Todo: don't use fmt, just write the chars manually and move cursor instead of
            // printing spaces and newlines. lol
            write!(screen, "{}", entry).unwrap();
        }
        screen.flush().unwrap();

        print!("{}", screen::ToAlternateScreen);
        let key = stdin.keys().next().unwrap().unwrap();
        print!("{}", screen::ToMainScreen);
        key
    };
    println!("got key: {:?}", key);
}
