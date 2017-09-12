extern crate select;
use select::document::Document;
use select::node::Node;
use select::predicate::{Predicate, Attr, Class, Name};

fn main() {
    let pop = Document::from(include_str!("pop.html"));
    for node in pop.find(Class("list-item-inner")) {
        let title = find_title(&node);
        let sub_title = find_subtitle(&node);
        let url = find_url(&node);
        let thumbnail = find_thumbnail(&node);
        let pid = find_pid(&node);
        let synopsis = find_synopsis(&node);
        println!("{}", title);
        println!("{:?}", sub_title);
        println!("{}", url);
        println!("{}", thumbnail);
        println!("{}", pid);
        println!("{}", synopsis);
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

fn find_thumbnail(node: &Node) -> String {
    node.find(Class("rs-image").descendant(Name("picture").descendant(Name("source"))))
        .next()
        .unwrap()
        .attr("srcset")
        .unwrap()
        .to_string()
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
