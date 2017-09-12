extern crate select;
use select::document::Document;

type BeebUrl<'a> = &'a str;

type TestBeebUrl = &'static str;

pub struct Programme<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
    pub synopsis: &'a str,
    pub pid: &'a str,
    pub thumbnail: &'a str,
    pub url: &'a str,
    pub index: &'a u16,
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
}

