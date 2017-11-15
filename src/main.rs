extern crate serde;
extern crate serde_json;
// #[macro_use]
// extern crate lazy_static;

#[macro_use]
mod entry;

use entry::Entry;

fn main() {
    let mut root: Vec<Entry> = Vec::new();

    root.push(entry!("Lists",
                     ["Bible", "Shopping", "Big spends", "Music", "Books", "Jokes"]));
    root.push(entry!("Projects"));
    root.push(entry!("St Barts"));
    root.push(entry!("Scratch"));

    println!("root: {:?}", root);
    println!();
    println!("serialized: {}", serde_json::to_string(&root).unwrap());
}
