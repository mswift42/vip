extern crate select;
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name, Predicate};

type BeebUrl<'a> = &'a str;

type TestBeebUrl = &'static str;

pub struct Category<'a> {
    name: String,
    programmes: Vec<&'a Programme>,
}

impl<'a> Category<'a> {
    pub fn new(name: String, programmes: Vec<&'a Programme>) -> Category<'a> {
        Category { name, programmes }
    }
}

#[derive(Debug)]
pub struct Programme {
    pub title: String,
    pub subtitle: String,
    pub synopsis: String,
    pub pid: String,
    pub thumbnail: String,
    pub url: String,
    pub index: u16,
}
impl Programme {
    fn new(
        title: String,
        subtitle: String,
        synopsis: String,
        pid: String,
        thumbnail: String,
        url: String,
        index: u16,
    ) -> Programme {
        Programme {
            title,
            subtitle,
            synopsis,
            pid,
            thumbnail,
            url,
            index,
        }
    }
}
#[derive(Clone, Debug)]
pub struct IplayerDocument {
    pub idoc: Document,
}

impl IplayerDocument {
    pub fn new(bu: TestBeebUrl) -> IplayerDocument {
        let idoc = Document::from(bu);
        IplayerDocument { idoc }
    }

    pub fn programmes(self) -> Vec<Programme> {
        let mut results: Vec<Programme> = Vec::new();
        for node in self.idoc.find(Class("list-item-inner")) {
            let title = find_title(&node);
            let subtitle = {
                match find_subtitle(&node) {
                    None => "".to_string(),
                    Some(sub) => sub.to_string(),
                }
            };
            let synopsis = find_synopsis(&node);
            let pid = find_pid(&node);
            let thumbnail = find_thumbnail(&node).to_string();
            let url = find_url(&node);
            let index = 0;
            let prog = Programme::new(title, subtitle, synopsis, pid, thumbnail, url, index);
            results.push(prog);
        }
        results
    }
}

fn find_title(node: &Node) -> String {
    node.find(Class("secondary").descendant(Class("title")))
        .next()
        .unwrap()
        .text()
}

fn find_subtitle(node: &Node) -> Option<String> {
    let sub = node.find(Class("secondary").descendant(Class("subtitle")))
        .next();
    match sub {
        None => None,
        Some(text) => Some(text.text()),
    }
}

fn find_url(node: &Node) -> String {
    let path = node.find(Name("a"))
        .next()
        .unwrap()
        .attr("href")
        .unwrap()
        .to_string();
    let url = String::from("www.bbc.co.uk");
    url + &path
}

fn find_thumbnail<'a>(node: &'a Node) -> &'a str {
    node.find(
        Class("rs-image").descendant(Name("picture").descendant(Name("source"))),
    ).next()
        .unwrap()
        .attr("srcset")
        .unwrap()
}

fn find_pid(node: &Node) -> String {
    match node.attr("data-ip-id") {
        None => node.find(Class("list-item-inner").descendant(Name("a")))
            .next()
            .unwrap()
            .attr("data-episode-id")
            .unwrap()
            .to_string(),
        Some(pid) => pid.to_string(),
    }
}

fn find_synopsis(node: &Node) -> String {
    node.find(Class("synopsis")).next().unwrap().text()
}

#[cfg(test)]
mod test {
    use super::Programme;
    use super::Category;
    use super::IplayerDocument;
    use super::{Class, Name};
    use super::Document;

    #[test]
    fn test_document() {
        let doc = IplayerDocument::new(include_str!("pop.html"));
        assert_eq!(
            doc.idoc.find(Name("h1")).next().unwrap().text(),
            "Most Popular"
        );
        assert_eq!(doc.idoc.find(Class("subtitle")).next().unwrap().text(),
        "Today's most popular programmes available on BBC iPlayer.");
    }
    #[test]
    fn test_programmes() {
        let doc = IplayerDocument::new(include_str!("pop.html"));
        let progr = doc.programmes();
        assert_eq!(
            progr[0].title, "Strike"
        );
        assert_eq!(
            progr[0].subtitle, "The Silkworm: Episode 1"
        );
        assert_eq!(
            progr[0].pid, "b0959ppk"
        );
        assert_eq!(
            progr[0].url, "www.bbc.co.uk/iplayer/episode/b0959ppk/strike-the-silkworm-episode-1"
        );
    }
}
