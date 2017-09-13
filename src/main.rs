extern crate select;
use select::document::Document;
use select::node::Node;
use select::predicate::{Predicate, Attr, Class, Name};
mod tv;
fn main() {
    let doc = tv::IplayerDocument::new(include_str!("pop.html"));
    let results = doc.programmes();
    for i in results {
        println!("{:?}", i);
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
