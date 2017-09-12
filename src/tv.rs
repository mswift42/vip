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
    pub index: u16,
}

impl<'a> Programme<'a> {
    fn new(title: &'a str, subtitle: &'a str,
    synopsis: &'a str, pid: &'a str, thumbnail: &'a str,
    url: &'a str, index: u16) -> Programme<'a> {
        Programme {
            title,
            subtitle,
            synopsis,
            pid,
            thumbnail,
            url,
            index,
        }
    }
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

