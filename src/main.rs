extern crate select;
extern crate cpuprofiler;
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name, Predicate};
use cpuprofiler::PROFILER;

mod tv;

fn main() {
    PROFILER.lock().unwrap().start("mainprofile").unwrap();
    let doc = tv::IplayerDocument::new(include_str!("../testhtml/pop.html"));
    PROFILER.lock().unwrap().stop().unwrap();
    let doc2 = Document::from(include_str!("../testhtml/comedy1.html"));
    for node in doc2.find(Class("page").descendant(Name("a"))) {
        println!("{:?}", node.attr("href"))
    }
}
