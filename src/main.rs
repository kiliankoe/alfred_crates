extern crate alfred;
extern crate crates_search;

use std::env;
use std::io;

fn main() {
    let query = env::args().nth(1).expect("Failed reading user input");

    match crates_search::search(&query) {
        Ok(crates) => {
            if crates.len() > 0 {
                let items = crates.iter().map(crate_to_item).collect::<Vec<alfred::Item>>();
                let _ = alfred::json::write_items(io::stdout(), &items);
            } else {
                let _ = alfred::json::write_items(io::stdout(),
                                                  &[alfred::ItemBuilder::new("👀 Nothing found")
                                                        .subtitle("Perhaps it's time to write \
                                                                   this crate? 💪")
                                                        .into_item()]);
            }
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
        .subtitle_mod(alfred::Modifier::Shift, "Open repository")
        .arg_mod(alfred::Modifier::Shift,
                 krate.repository_url.clone().unwrap_or("".to_owned()))
        .subtitle_mod(alfred::Modifier::Option, "Insert as Cargo.toml dependency")
        .arg_mod(alfred::Modifier::Option,
                 format!("{} = \"{}\"", krate.name.clone(), krate.version.clone()))
        .into_item()
}
