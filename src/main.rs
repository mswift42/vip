extern crate select;
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name, Predicate};
mod tv;
fn main() {
    let doc = tv::IplayerDocument::new(include_str!("../testhtml/pop.html"));
    let results = doc.programmes();
    for i in results {
        println!("{:?}", i);
    }
}
