extern crate reqwest;
extern crate select;
extern crate url;
use std::path::PathBuf;

use std::error;
use std::fs;

use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};

pub trait DocumentLoader {
    fn load(&self) -> BoxResult<IplayerDocument>;
}

type BoxResult<T> = Result<T, Box<error::Error>>;

pub struct IplayerDocument<'a> {
    doc: Document,
    url: &'a str,
}

impl<'a> IplayerDocument<'a> {
    fn programme_nodes(&self) -> Vec<Option<IplayerNode>> {
        self.doc
            .find(Class("content-item"))
            .map(|nd| match nd.next() {
                None => None,
                Some(nod) => Some(IplayerNode { node: nod }),
            }).collect()
    }
}

pub struct BeebURL<'a> {
    url: &'a str,
}

pub struct TestHTMLURL<'a> {
    url: &'a str,
}

struct ProgrammePage<'a> {
    idoc: IplayerDocument<'a>,
}

struct IplayerSelection<'a> {
    prog: Option<Programme<'a>>,
    programme_page: Option<&'a str>,
}

impl<'a> IplayerSelection<'a> {
    fn new(inode: IplayerNode<'a>) -> IplayerSelection<'a> {
        let title = inode.title();
        let subtitle = inode.subtitle();
        match inode.programme_site() {
            None => IplayerSelection {
                prog: Some(Programme::new(title, subtitle, inode)),
                programme_page: None,
            },
            Some(u) => IplayerSelection {
                prog: None,
                programme_page: Some(u),
            },
        }
    }
}

struct IplayerNode<'a> {
    node: select::node::Node<'a>,
}

impl<'a> IplayerNode<'a> {
    fn programme_site(&self) -> Option<&'a str> {
        self.node.find(Class("lnk")).next()?.attr("href")
    }

    fn title(&self) -> Option<String> {
        match self.node.find(Class("content-item__title")).next() {
            None => None,
            Some(nd) => Some(nd.text()),
        }
    }

    fn subtitle(&self) -> Option<String> {
        match self
            .node
            .find(Class("content-item__info__primary"))
            .next()?
            .descendants()
            .next()?
            .find(Class("content-item__description"))
            .next()
        {
            None => None,
            Some(nd) => Some(nd.text()),
        }
    }

    fn synopsis(&self) -> Option<String> {
        match self
            .node
            .find(Class("content-item__info__secondary"))
            .next()?
            .descendants()
            .next()?
            .find(Class("content-item__description"))
            .next()
        {
            None => None,
            Some(nd) => Some(nd.text()),
        }
    }

    fn url(&self) -> Option<&'a str> {
        self.node.find(Name("a")).next()?.attr("href")
    }

    fn thumbnail(&self) -> Option<&'a str> {
        match self
            .node
            .find(Class("rs-image"))
            .next()?
            .descendants()
            .next()?
            .find(Class("picture"))
            .next()?
            .find(Class("source"))
            .next()?
            .attr("srcset")
        {
            None => None,
            Some(set) => set.split(' ').next(),
        }
    }

    fn available(&self) -> Option<String> {
        match self
            .node
            .find(Class("content-item__sublabels"))
            .next()?
            .descendants()
            .next()?
            .find(Name("span"))
            .last()
        {
            None => None,
            Some(sp) => Some(sp.text()),
        }
    }

    fn duration(&self) -> Option<String> {
        match self
            .node
            .find(Class("content-item__sublabels"))
            .next()?
            .descendants()
            .next()?
            .find(Name("span"))
            .next()
        {
            None => None,
            Some(sp) => Some(sp.text()),
        }
    }
    fn iplayer_selections(&self) -> Vec<IplayerSelection<'a>> {
        self.node
            .descendants()
            .map(|node| IplayerSelection::new(IplayerNode { node }))
            .collect()
    }
}

pub struct Programme<'a> {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub synopsis: Option<String>,
    pub thumbnail: Option<&'a str>,
    pub url: Option<&'a str>,
    pub index: usize,
    pub available: Option<String>,
    pub duration: Option<String>,
}

impl<'a> Programme<'a> {
    fn new(title: Option<String>, subtitle: Option<String>, inode: IplayerNode) -> Programme {
        let synopsis = inode.synopsis();
        let url = inode.url();
        let thumbnail = inode.thumbnail();
        let available = inode.available();
        let duration = inode.duration();
        let index = 0;
        Programme {
            title,
            subtitle,
            synopsis,
            thumbnail,
            url,
            index,
            available,
            duration,
        }
    }
}

impl<'a> ProgrammePage<'a> {
    fn programmes(&self) -> Vec<Option<Programme>> {
        let title = match self.idoc.doc.find(Class("hero-header__title")).next() {
            None => None,
            Some(nd) => Some(nd.text()),
        };
        self.idoc
            .doc
            .find(Class("content-item"))
            .map(move |node| ProgrammePage::programme(title.clone(), node.next()))
            .collect()
    }
    fn programme(title: Option<String>, node: Option<Node>) -> Option<Programme> {
        match node {
            None => None,
            Some(nod) => {
                let inode = IplayerNode { node: nod };
                Some(Programme::new(title, inode.subtitle(), inode))
            }
        }
    }
}

impl<'a> BeebURL<'a> {
    fn load(&self) -> BoxResult<IplayerDocument> {
        let uri = url::Url::parse(self.url)?;
        let resp = reqwest::get(uri)?;
        let doc = select::document::Document::from_read(resp)?;
        Ok(IplayerDocument { doc, url: self.url })
    }
}

impl<'a> TestHTMLURL<'a> {
    fn load(&self) -> BoxResult<IplayerDocument> {
        let html = fs::read(self.url)?;
        let doc = Document::from_read(&html[..])?;
        Ok(IplayerDocument { doc, url: self.url })
    }
}

fn main() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("testhtml");
    let tu = TestHTMLURL {
        url: "testhtml/films1.html",
    };
    let idr = tu.load();
    assert!(idr.is_ok());
    let id = idr.unwrap();
    let inode = IplayerNode {
        node: id.doc.find(Class("content-item")).next().unwrap(),
    };
    let isels = inode.iplayer_selections();
    assert_eq!(isels.len(), 24);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn test_load() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("testhtml");
        let tu = TestHTMLURL {
            url: "testhtml/food1.html",
        };
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("testhtml");
        let tu = TestHTMLURL {
            url: "testhtml/films1.html",
        };
        let idr = tu.load();
        assert!(idr.is_ok());
        let id = idr.unwrap();
        let inode = IplayerNode {
            node: id.doc.find(Class("content-item")).next().unwrap(),
        };
        let isels = inode.iplayer_selections();
        assert_eq!(isels.len(), 24);
        let id = tu.load();
        assert!(id.is_ok());
        let doc = id.unwrap();
        let pn = &doc.programme_nodes();
        assert_eq!(pn.len(), 26);
    }

    #[test]
    fn test_programme_page() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("testhtml");
        let tu = TestHTMLURL {
            url: "testhtml/delia_smiths_cookery_course.html",
        };
        let idr = tu.load();
        assert!(idr.is_ok());
        let id = idr.unwrap();
        let progpage = ProgrammePage { idoc: id };
        let progs = progpage.programmes();
        assert_eq!(progs.len(), 10);
    }

    #[test]
    fn test_iplayer_selections() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("testhtml");
        let tu = TestHTMLURL {
            url: "testhtml/films1.html",
        };
        let idr = tu.load();
        assert!(idr.is_ok());
        let id = idr.unwrap();
        let inode = IplayerNode {
            node: id.doc.find(Class("content-item")).next().unwrap(),
        };
        let isels = inode.iplayer_selections();
        assert_eq!(isels.len(), 24);
    }
}
