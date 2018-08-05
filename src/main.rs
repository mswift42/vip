extern crate reqwest;
extern crate select;
extern crate url;

use std::error;
use std::fs;

use select::document::Document;

pub trait DocumentLoader {
    fn load(&self) -> BoxResult<IplayerDocument>;
}

type BoxResult<T> = Result<T, Box<error::Error>>;

pub struct IplayerDocument {
    doc: Document,
}

pub struct BeebURL<'a> {
    url: &'a str,
}

pub struct TestHTMLURL<'a> {
    url: &'a str
}

impl<'a> BeebURL<'a> {
    fn load(&self) -> BoxResult<IplayerDocument> {
        let uri = url::Url::parse(self.url)?;
        let resp = reqwest::get(uri)?;
        let doc = select::document::Document::from_read(resp)?;
        Ok(IplayerDocument { doc })
    }
}

impl<'a> TestHTMLURL<'a> {
    fn load(&self) -> BoxResult<IplayerDocument> {
        let html = fs::read(self.url)?;
        let doc = Document::from_read(html)?;
        Ok(IplayerDocument{doc})
    }
}

fn main() {
    println!("Hello, world!");
}
