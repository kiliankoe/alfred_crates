extern crate alfred;
extern crate crates_search;

use std::env;
use std::io;

fn main() {
    // let query = env::args().nth(1).unwrap_or("none".to_owned());

    match crates_search::search(&query) {
        Ok(crates) => {
            let items =
                crates.iter().map(|krate| crate_to_item(krate)).collect::<Vec<alfred::Item>>();
            let _ = alfred::json::write_items(io::stdout(), &items);
        }
        Err(err) => {
            let _ = alfred::json::write_items(io::stdout(),
                                              &[alfred::ItemBuilder::new("Unexpected error \
                                                                          loading crates 😵")
                                                    .subtitle(format!("{}", err))
                                                    .into_item()]);
        }
    }
}

// TODO: Is there a way around all that `.clone()`ing?
fn crate_to_item<'a>(krate: &crates_search::Crate) -> alfred::Item<'a> {
    alfred::ItemBuilder::new(format!("{} v{}", krate.name.clone(), krate.version.clone()))
        .subtitle(krate.description.clone().unwrap_or("No description available".to_owned()))
        .arg(krate.homepage_url
            .clone()
            .unwrap_or(format!("https://crates.io/crates/{}", krate.name.clone())))
        .subtitle_mod(alfred::Modifier::Command, "Open documentation")
        .arg_mod(alfred::Modifier::Command,
                 krate.documentation_url
                     .clone()
                     .unwrap_or(format!("https://docs.rs/{}", krate.name.clone())))
        .subtitle_mod(alfred::Modifier::Option, "Insert as Cargo.toml dependency")
        .arg_mod(alfred::Modifier::Option,
                 format!("{} = \"{}\"", krate.name.clone(), krate.version.clone()))
        .into_item()
}
