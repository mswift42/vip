extern crate select;
extern crate url;
extern crate reqwest;

use select::document::Document;

pub trait DocumentLoader {
    fn load(&self) -> Document;
}

pub struct IplayerDocument {
    doc: Document
}

// type Result<IplayerDocument> = Result<IplayerDocument, >
pub struct BeebURL<'a> {
    url: &'a str
}

impl<'a> BeebURL<'a> {
    fn load(&self) -> Document {

    }
}
fn main() {
    println!("Hello, world!");
}
