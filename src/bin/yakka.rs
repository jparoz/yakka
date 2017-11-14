extern crate serde_json;
extern crate cursive;

#[macro_use]
extern crate yakka;

use yakka::Entry;
use cursive::Cursive;
use cursive::views::TextView;
use cursive::event::Key;
use cursive::theme::{Theme, BorderStyle, Palette, Color, BaseColor};

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
    siv.set_theme(Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        colors: Palette {
            background: Color::Dark(BaseColor::Black),
            shadow: Color::Dark(BaseColor::Black),
            view: Color::Dark(BaseColor::Black),
            primary: Color::Dark(BaseColor::White),
            secondary: Color::Light(BaseColor::White),
            tertiary: Color::Light(BaseColor::Magenta),
            title_primary: Color::Dark(BaseColor::White),
            title_secondary: Color::Light(BaseColor::White),
            highlight: Color::Light(BaseColor::Red),
            highlight_inactive: Color::Light(BaseColor::Blue),
        },
    });
    siv.add_fullscreen_layer(TextView::new(format!("{:?}", root)));
    siv.add_global_callback(Key::Esc, |siv| siv.quit());
    siv.run();
}
