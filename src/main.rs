struct Crate {
    name: String,
    author: String,
    link: String,
}

impl Crate {
    fn to_xml(&self) -> String {
        format!("<item arg=\"{}\">
<title>{}</title>
<subtitle>{}</subtitle>
</item>", self.link, self.name, self.author)
    }
}

fn main() {
    let crate1 = Crate {
        name: "crate1".to_owned(),
        author: "author".to_owned(),
        link: "https://github.com/author/crate1".to_owned(),
    };
    let crate2 = Crate {
        name: "crate2".to_owned(),
        author: "author".to_owned(),
        link: "https://github.com/author/crate2".to_owned(),
    };

    let crates = [crate1, crate2];

    println!("<?xml version=\"1.0\"?><items>");
    for c in crates.iter() {
        println!("{}", c.to_xml());
    }
    println!("</items></xml>");
}
