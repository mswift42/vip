extern crate select;
use select::document::Document;

type BeebUrl<'a> = &'a str;

type TestBeebUrl = &'static str;

pub struct Programme<'a> {
    pub title: &'astr,
    pub subtitle: &'str,
    pub synopsis: &'str,
    pub pid: &'str,
    pub thumbnail: &'str,
    pub url: &'str,
    pub index: u16,
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

