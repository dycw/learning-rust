use opml::OPML;
use std::fs;

fn read() -> OPML {
    let file = "overcast.opml";
    let contents = fs::read_to_string(file).expect("Failed to read file");
    OPML::from_str(&contents).unwrap()
}

fn read_titles() -> Vec<String> {
    let doc = read();
    doc.body
        .outlines
        .into_iter()
        .filter_map(|outline| outline.title)
        .collect()
}

fn main() {
    let titles = read_titles();
    for title in titles {
        println!("{}", title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version() {
        let doc = read();
        assert_eq!(doc.version, "1.0");
        assert_eq!(
            doc.head
                .expect("Failed to read head")
                .title
                .expect("Failed to read title"),
            "Overcast Podcast Subscriptions"
        )
    }

    #[test]
    fn head() {
        let titles = read_titles();
        assert_eq!(titles.len(), 60);
        assert_eq!(titles[0], "Go Time: Golang, Software Engineering");
    }
}
