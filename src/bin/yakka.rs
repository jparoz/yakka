extern crate serde_json;
extern crate cursive;

#[macro_use]
extern crate yakka;

use yakka::Entry;
use cursive::{Cursive, views};

fn main() {
    let mut root: Vec<Entry> = Vec::new();

    root.push(entry!("Lists",
                     ["Bible", "Shopping", "Big spends", "Music", "Books", "Jokes"]));
    root.push(entry!("Projects"));
    root.push(entry!("St Barts"));
    root.push(entry!("Scratch"));

    // let writer = std::io::stdout(); // @Temp
    // serde_json::to_writer(writer, &root).unwrap();

    let mut siv = Cursive::new();
    siv.add_layer(views::TextView::new(format!("{:?}", root)));
    siv.add_global_callback('q', |s| s.quit());
    siv.run();
}
