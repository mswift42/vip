extern crate reqwest;
extern crate select;
extern crate url;

use std::error;
use std::fs;

use select::document::Document;
use select::predicate::{Class, Name};

pub trait DocumentLoader {
    fn load(&self) -> BoxResult<IplayerDocument>;
}

type BoxResult<T> = Result<T, Box<error::Error>>;

pub struct IplayerDocument<'a> {
    doc: Document,
    url: &'a str,
}

pub struct BeebURL<'a> {
    url: &'a str,
}

pub struct TestHTMLURL<'a> {
    url: &'a str,
}

struct IplayerSelection<'a> {
    prog: Option<Programme<'a>>,
    programme_page: Option<&'a str>,
}

struct IplayerNode<'a> {
    node: select::node::Node<'a>,
}

impl<'a> IplayerNode<'a> {
    fn programme_node(&self) -> Option<IplayerNode> {
       match self.node.find(Class("content-item")).next() {
          None => None,
           Some(nd) => Some(IplayerNode{node: nd}),
       }
    }

    fn programme_site(&self) ->Option<&'a str> {
        match self.node.find(Class("lnk")).next() {
            None => None,
            Some(nd) => Some(nd.attr("href").unwrap()),
        }
    }

    fn title(&self) -> Option<String> {
        match self.node.find(Class("content-item__title")).next() {
            None => None,
            Some(nd) => Some(nd.text()),
        }

    }

    fn subtitle(&self) -> Option<String> {
        match self.node.find(Class("content-item__info__primary"))
            .next()?.descendants().next()?
            .find(Class("content-item__description")).next() {
            None => None,
            Some(nd) => Some(nd.text()),
        }
    }
}

pub struct Programme<'a> {
    pub title: String,
    pub subtitle: Option<String>,
    pub synopsis: String,
    pub thumbnail: &'a str,
    pub url: String,
    pub index: usize,
    pub available: String,
    pub duration: String,
}

impl<'a> BeebURL<'a> {
    fn load(&self) -> BoxResult<IplayerDocument> {
        let uri = url::Url::parse(self.url)?;
        let resp = reqwest::get(uri)?;
        let doc = select::document::Document::from_read(resp)?;
        Ok(IplayerDocument {
            doc,
            url: self.url,
        })
    }
}

impl<'a> TestHTMLURL<'a> {
    fn load(&self) -> BoxResult<IplayerDocument> {
        let html = fs::read(self.url)?;
        let doc = Document::from_read(&html[..])?;
        Ok(IplayerDocument {
            doc,
            url: self.url,
        })
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn test_load() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("testhtml");
        let tu = TestHTMLURL {
            url: "testhtml/films1.html",
        };
        let id = tu.load();
        assert!(id.is_ok());
    }
}
