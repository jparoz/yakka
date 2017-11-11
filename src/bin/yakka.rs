extern crate serde_json;
extern crate yakka;

use yakka::Entry;

fn entry(text: &str) -> Entry {
    Entry {
        text: text.to_string(),
        children: Vec::new(),
    }
}

fn main() {
    let mut root: Vec<Entry> = Vec::new();
    root.push(entry("Lists"));
    root.push(entry("Projects"));
    root.push(entry("St Barts"));
    root.push(entry("Scratch"));
    root[0].children.push(entry("Bible"));
    root[0].children.push(entry("Shopping"));
    root[0].children.push(entry("Big spends"));
    root[0].children.push(entry("Music"));
    root[0].children.push(entry("Books"));
    root[0].children.push(entry("Jokes"));
    println!("root:");
    println!("{:?}", root);

    println!();

    // println!("serialized:");
    // let writer = std::io::stdout(); // @Temp
    // serde_json::to_writer(writer, &root).unwrap();

    let serialized = serde_json::to_string(&root).unwrap();
    println!("serialized:");
    println!("{}", serialized);

    println!();

    let deserialized: Vec<Entry> = serde_json::from_str(&serialized).unwrap();
    println!("deserialized:");
    println!("{:?}", deserialized);
}
