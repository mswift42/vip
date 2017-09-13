extern crate select;
use select::document::Document;
use select::node::Node;
use select::predicate::{Predicate, Attr, Class, Name};

type BeebUrl<'a> = &'a str;

type TestBeebUrl = &'static str;

pub struct Programme {
    pub title: String,
    pub subtitle: String,
    pub synopsis: String,
    pub pid: String,
    pub thumbnail: String,
    pub url: String,
    pub index: u16,
}

impl<'a> Programme<'a> {
    fn new(title: String, subtitle: &'a str,
    synopsis: &'a str, pid: &'a str, thumbnail: &'a str,
    url: &'a str, index: u16) -> Programme<'a> {
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

pub struct IplayerDocument {
    pub idoc: Document,
}

impl IplayerDocument {
    fn new(bu: TestBeebUrl) -> IplayerDocument {
        let idoc = Document::from(bu);
        IplayerDocument{
            idoc
        }
    }

    fn programmes<'a>(&'a self) -> Vec<&'a Programme> {
        let mut results: Vec<&Programme> = Vec::new();
        for node in self.idoc.find(Class("list-item-inner")) {
            let title = find_title(&node);
            let subtitle = &find_subtitle(&node).unwrap();
            let synopsis = &find_synopsis(&node);
            let pid = &find_pid(&node);
            let thumbnail = &find_thumbnail(&node);
            let url = &find_url(&node);
            let index = 0;
            let prog = &Programme::new(title,
            subtitle, synopsis, pid, thumbnail, url, index);
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
    node.find(Class("rs-image").descendant(Name("picture").descendant(Name("source"))))
        .next()
        .unwrap()
        .attr("srcset")
        .unwrap()
}

fn find_pid(node: &Node) -> String {
    match node.attr("data-ip-id") {
        None => {
            node.find(Class("list-item-inner").descendant(Name("a")))
                .next()
                .unwrap()
                .attr("data-episode-id")
                .unwrap()
                .to_string()
        }
        Some(pid) => pid.to_string(),
    }
}

fn find_synopsis(node: &Node) -> String {
    node.find(Class("synopsis")).next().unwrap().text()
}