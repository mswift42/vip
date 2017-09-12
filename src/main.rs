extern crate select;
use select::document::Document;
use select::node::Node;
use select::predicate::{Predicate, Attr, Class, Name};

fn main() {
    let pop = Document::from(include_str!("pop.html"));
    for node in pop.find(Class("list-item-inner")) {
        let title = find_title(&node);
        let sub_title = find_subtitle(&node);
        println!("{}", title);
        println!("{:?}", sub_title);
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
