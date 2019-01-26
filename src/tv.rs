use std::error;
use std::fs;

use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name, Predicate};


pub trait DocumentLoader {
    fn load(&self) -> BoxResult<IplayerDocument>;
}

trait NextPager {
    fn main_doc(&self) -> &IplayerDocument;
    fn next_pages(&self) -> Vec<Box<dyn DocumentLoader>>;
    fn programme_pages(_: Vec<IplayerSelection>) -> Vec<Box<dyn DocumentLoader>>;
}

type BoxResult<T> = Result<T, Box<error::Error>>;

#[derive(Clone)]
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

    fn main_doc(&self) -> &IplayerDocument {
        self
    }

    fn next_pages(&self) -> Vec<Box<BeebURL>> {
        np_page_options(self).iter().map(|url| Box::new(BeebURL { url })).collect()
    }

    fn programme_pages(selections: Vec<IplayerSelection>) -> Vec<Box<BeebURL>> {
        selections.iter().filter(|sel| sel.programme_page.is_some())
            .map(|opturl| Box::new(BeebURL { url: opturl.programme_page.unwrap() }))
            .collect()
    }
}

fn np_page_options<'a>(idoc: &'a IplayerDocument) -> Vec<&'a str> {
    idoc.doc.find(Class("pagination__number")
        .descendant(Name("a")))
        .filter_map(|n| n.attr("href"))
        .collect()
}

impl<'a> IplayerDocument<'a> {
    fn iplayer_selections(&self) -> Vec<IplayerSelection> {
        self.doc
            .find(Class("content-item"))
            .into_iter()
            .map(|node| IplayerSelection::new(IplayerNode { node }))
            .collect::<Vec<IplayerSelection>>()
    }
}

pub struct BeebURL<'a> {
    url: &'a str,
}

static BBCPREFIX: &'static str = "https://bbc.co.uk";


struct ProgrammePage<'a> {
    idoc: IplayerDocument<'a>,
}

#[derive(Clone)]
pub struct IplayerSelection<'a> {
    pub prog: Option<Programme<'a>>,
    pub programme_page: Option<&'a str>,
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
            .children()
            .map(|node| IplayerSelection::new(IplayerNode { node }))
            .collect()
    }
}

#[derive(Clone)]
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

struct MainCategoryDocument<'a> {
    maindoc: IplayerDocument<'a>,
    nextdocs: Vec<IplayerDocument<'a>>,
    programme_page_docs: Vec<IplayerDocument<'a>>,
    selections: Vec<IplayerSelection<'a>>,
}

impl<'a> DocumentLoader for BeebURL<'a> {
    fn load(&self) -> BoxResult<IplayerDocument> {
        let uri = url::Url::parse(self.url)?;
        let resp = reqwest::get(uri)?;
        let doc = select::document::Document::from_read(resp)?;
        Ok(IplayerDocument { doc, url: self.url })
    }
}


mod testutils {
    use super::*;

    pub struct TestHTMLURL<'a> {
        pub url: &'a str,
    }

    impl<'a> DocumentLoader for TestHTMLURL<'a> {
        fn load(&self) -> super::BoxResult<IplayerDocument> {
            let html = fs::read(self.url)?;
            let doc = Document::from_read(&html[..])?;
            Ok(IplayerDocument { doc, url: self.url })
        }
    }

    #[derive(Clone)]
    pub struct TestIplayerDocument<'a> {
        pub idoc: IplayerDocument<'a>,
    }

    impl<'a> NextPager for TestIplayerDocument<'a> {
        fn main_doc(&self) -> &IplayerDocument {
            &self.idoc
        }

        pub fn next_pages(&self) -> Vec<Box<dyn DocumentLoader>> {
            np_page_options(&self.idoc).iter().map(
                |url| Box::new(TestHTMLURL { url })
            ).collect()
        }

        pub fn programme_pages(selres: &'a Vec<IplayerSelection>) -> Vec<TestHTMLURL<'a>> {
            selres.iter().filter(|sel|
                sel.programme_page.is_some())
                .map(|sel| TestHTMLURL { url: sel.programme_page.unwrap() })
                .collect()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tv::testutils::*;

    #[test]
    fn test_load() {
        let tu = testutils::TestHTMLURL {
            url: "testhtml/films1.html",
        };
        let idr = tu.load();
        assert!(idr.is_ok());
    }

    #[test]
    fn test_programme_page() {
        let tu = testutils::TestHTMLURL {
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
    fn test_programme_site() {
        let tu = testutils::TestHTMLURL {
            url: "testhtml/films1.html",
        };
        let idr = tu.load();
        assert!(idr.is_ok());
        let id = idr.unwrap();
        let nodes = id.doc.find(Class("content-item"));
        let sites: Vec<IplayerNode> = nodes
            .filter(|node| IplayerNode { node: *node }.programme_site().is_some())
            .map(|node| IplayerNode { node })
            .collect();
        assert_eq!(sites.len(), 2);
        assert_eq!(
            sites[0].programme_site().unwrap(),
            "testhtml/adam_curtis.html"
        );
        assert_eq!(
            sites[1].programme_site().unwrap(),
            "testhtml/storyville.html"
        );
        let tu = testutils::TestHTMLURL {
            url: "testhtml/food1.html",
        };
        let idr = tu.load();
        assert!(idr.is_ok());
        let id = idr.unwrap();
        let nodes = id.doc.find(Class("content-item"));
        let sites: Vec<IplayerNode> = nodes
            .filter(|node| IplayerNode { node: *node }.programme_site().is_some())
            .map(|node| IplayerNode { node })
            .collect();
        assert_eq!(sites.len(), 20);
        assert_eq!(
            sites[0].programme_site().unwrap(),
            "testhtml/britains_best_home_cook.html"
        );
        assert_eq!(
            sites[1].programme_site().unwrap(),
            "testhtml/britains_fat_fight.html"
        );
        assert_eq!(
            sites[2].programme_site().unwrap(),
            "testhtml/caribbean_food_made_easy.html"
        );
        assert_eq!(
            sites[3].programme_site().unwrap(),
            "testhtml/delia_smiths_cookery_course.html"
        );
    }

    #[test]
    fn test_iplayer_selections() {
        let tu = testutils::TestHTMLURL {
            url: "testhtml/films1.html",
        };
        let idr = tu.load();
        assert!(idr.is_ok());
        let id = idr.unwrap();
        let isels = id.iplayer_selections();
        assert_eq!(isels.len(), 24);
        let prog_sites = isels.iter().filter(|sel| sel.programme_page.is_some());
        assert_eq!(prog_sites.count(), 2);
        let progs = isels.iter().filter(|sel| sel.prog.is_some());
        assert_eq!(progs.count(), 22);
        let tu = TestHTMLURL {
            url: "testhtml/food1.html",
        };
        let idr = tu.load();
        assert!(idr.is_ok());
        let id = idr.unwrap();
        let isel = id.iplayer_selections();
        assert_eq!(isel.len(), 26);
        let prog_sites = isel.iter().filter(|sel| sel.programme_page.is_some());
        assert_eq!(prog_sites.count(), 20);
        let progs = isels.iter().filter(|sel| sel.prog.is_some());
        assert_eq!(progs.count(), 22);
    }

    #[test]
    fn test_next_pages() {
        let tu = testutils::TestHTMLURL {
            url: "testhtml/comedy1.html"
        };
        let idr = tu.load();
        assert!(idr.is_ok());
        let idoc = idr.unwrap();
        let tdoc = TestIplayerDocument { idoc };
        let np = tdoc.next_pages();
        assert_eq!(np.len(), 1);
        assert_eq!(np[0].url, "testhtml/comedy2.html");
        let tu = testutils::TestHTMLURL {
            url: "testhtml/food1.html"
        };
        let idr = tu.load();
        assert!(idr.is_ok());
        let tdoc = TestIplayerDocument { idoc: idr.unwrap() };
        let np = tdoc.next_pages();
        assert_eq!(np.len(), 0);
    }

    #[test]
    fn test_programme_pages() {
        let tu = testutils::TestHTMLURL {
            url: "testhtml/films1.html"
        };
        let idr = tu.load();
        assert!(idr.is_ok());
        let tdoc = TestIplayerDocument { idoc: idr.unwrap() };
        let seldoc = tdoc.clone();
        let isel = tdoc.idoc.iplayer_selections();
        let progpages = seldoc.programme_pages(&isel);
        assert_eq!(progpages.len(), 2);
        assert_eq!(progpages[0].url, "testhtml/adam_curtis.html");
        assert_eq!(progpages[1].url, "testhtml/storyville.html");
        let tu = testutils::TestHTMLURL {
            url: "testhtml/food1.html"
        };
        let idr = tu.load();
        let tdoc = TestIplayerDocument { idoc: idr.unwrap() };
        let seldoc = tdoc.clone();
        let isel = tdoc.idoc.iplayer_selections();
        let progpages = seldoc.programme_pages(&isel);
        assert_eq!(progpages.len(), 20);
        assert_eq!(progpages[0].url, "testhtml/britains_best_home_cook.html");
        assert_eq!(progpages[1].url, "testhtml/britains_fat_fight.html");
        assert_eq!(progpages[2].url, "testhtml/caribbean_food_made_easy.html");
        assert_eq!(progpages[3].url, "testhtml/delia_smiths_cookery_course.html");
        assert_eq!(progpages[19].url, "testhtml/top_of_the_shop_with_tom_kerridge.html");
    }

    #[test]
    fn test_new_main_category() {
        let tu = testutils::TestHTMLURL {
            url: "testhtml/food1.html"
        };
        let idr = tu.load();
        assert!(idr.is_ok());
        let tdoc = TestIplayerDocument { idoc: idr.unwrap() };
    }
}
