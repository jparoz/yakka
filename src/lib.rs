extern crate serde;
extern crate serde_json;

use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::ser::SerializeTuple;
use serde::de::{Visitor, SeqAccess, Error};

use std::fmt;

#[macro_export]
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

#[derive(Debug)]
pub struct Entry {
    pub text: String,
    pub children: Vec<Entry>,
}

impl Serialize for Entry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        if self.children.is_empty() {
            let mut tuple = serializer.serialize_tuple(1)?;
            tuple.serialize_element(&self.text)?;
            tuple.end()
        } else {
            let mut tuple = serializer.serialize_tuple(2)?;
            tuple.serialize_element(&self.text)?;
            tuple.serialize_element(&self.children)?;
            tuple.end()
        }
    }
}

struct EntryVisitor;

impl<'de> Deserialize<'de> for Entry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_seq(EntryVisitor)
    }
}

impl<'de> Visitor<'de> for EntryVisitor {
    type Value = Entry;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a sequence with length 1 or 2")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Entry, A::Error>
        where A: SeqAccess<'de>
    {
        let text = match seq.next_element()? {
            Some(text) => text,
            None => return Err(Error::invalid_length(0, &self)),
        };
        let children = match seq.next_element()? {
            Some(children) => children,
            None => Vec::new(),
        };
        Ok(Entry {
            text: text,
            children: children,
        })
    }
}
