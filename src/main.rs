extern crate serde;
extern crate serde_json as json;
#[macro_use]
extern crate lazy_static;
#[macro_use]
mod entry;

fn main() {}

#[cfg(test)]
mod tests {
    use json;
    use entry::Entry;

    lazy_static! {
        static ref ROOT: Vec<Entry> = {
            let mut root: Vec<Entry> = Vec::new();

            root.push(entry!("Lists",
                            ["Bible", "Shopping", "Big spends", "Music", "Books", "Jokes"]));
            root.push(entry!("Projects"));
            root.push(entry!("St Barts"));
            root.push(entry!("Scratch"));

            root
        };
    }

    #[test]
    fn round_trip() {
        let serialized = json::to_string(&*ROOT).unwrap();
        let deserialized: Vec<Entry> = json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, *ROOT);
    }
}
