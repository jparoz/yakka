extern crate serde_json;
extern crate yakka;

use yakka::Entry;

macro_rules! entry {
    ($text:expr) => {
        Entry {
            text: $text.to_string(),
            children: Vec::new(),
        }
    };
    ($text:expr, [$($child:expr),+]) => {
        {
            let mut e = Entry {
                text: $text.to_string(),
                children: Vec::new(),
            };
            $(
                e.children.push(entry!($child));
            )+
            e
        }
    };
}

fn main() {
    let mut root: Vec<Entry> = Vec::new();

    root.push(entry!("Lists",
                     ["Bible", "Shopping", "Big spends", "Music", "Books", "Jokes"]));
    root.push(entry!("Projects"));
    root.push(entry!("St Barts"));
    root.push(entry!("Scratch"));
    println!("root:");
    println!("{:?}", root);

    // let writer = std::io::stdout(); // @Temp
    // serde_json::to_writer(writer, &root).unwrap();
}
