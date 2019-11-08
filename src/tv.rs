use std::error;
use std::fs;

use crossbeam::thread;
use futures::{Future, Stream, TryFutureExt};
use select::document::Document;
use select::document::Find;
use select::predicate::{Class, Name, Predicate};

pub struct BeebURL<'a> {
    url: &'a str,
}

impl<'a> BeebURL<'a> {
     async fn load_async(&self) -> () Future {
         let a : () = reqwest::get(self.url);
        a
     }
}

pub trait DocumentLoader {
    fn load(&self) -> BoxResult<IplayerDocument>;
}

pub trait NextPager {
    fn main_doc(&self) -> &IplayerDocument;
    fn next_pages(&self) -> Vec<Box<dyn DocumentLoader>>;
    fn programme_pages(&self, _: Vec<IplayerSelection>) -> Vec<Box<dyn DocumentLoader>>;
}

pub type Error = Box<dyn error::Error + Send + Sync>;

type BoxResult<T> = Result<T, Error>;

#[derive(Clone)]
pub struct IplayerDocument<'a> {
    doc: Document,
    url: &'a str,
}

impl IplayerDocument<'_> {
    fn programme_nodes(&self) -> Vec<Option<IplayerNode>> {
        self.doc
            .find(Class("content-item"))
            .map(|nd| match nd.next() {
                None => None,
                Some(nod) => Some(IplayerNode { node: nod }),
            })
            .collect()
    }

    fn main_doc(&self) -> &IplayerDocument {
        self
    }

    fn next_pages(&self) -> Vec<Box<BeebURL>> {
        np_page_options(self)
            .iter()
            .map(|url| Box::new(BeebURL { url }))
            .collect()
    }

    fn programme_pages(self, selections: Vec<IplayerSelection>) -> Vec<Box<BeebURL>> {
        selections
            .iter()
            .filter(|sel| sel.programme_page_url.is_some())
            .map(|opturl| {
                Box::new(BeebURL {
                    url: opturl.programme_page_url.unwrap(),
                })
            })
            .collect()
    }

    fn is_boxset(&self) -> bool {
        self.doc.find(Class("series-nav")).next().is_some()
    }

    fn series_urls(&self) -> Vec<Box<BeebURL>> {
        let urls = self
            .doc
            .find(Name("a").and(Class("series-nav__button")))
            .filter_map(|n| n.attr("href"));
        urls.map(|u| Box::new(BeebURL { url: u })).collect()
    }
}

fn np_page_options<'a>(idoc: &'a IplayerDocument) -> Vec<&'a str> {
    idoc.doc
        .find(Class("pagination__number").descendant(Name("a")))
        .filter_map(|n| n.attr("href"))
        .collect()
}

impl IplayerDocument<'_> {
    fn iplayer_selections(&self) -> Vec<IplayerSelection> {
        self.doc
            .find(Class("content-item"))
            .into_iter()
            .map(|node| IplayerSelection::new(IplayerNode { node }))
            .collect::<Vec<IplayerSelection>>()
    }
}

static BBCPREFIX: &'static str = "https://bbc.co.uk";

struct ProgrammePage<'a> {
    idoc: IplayerDocument<'a>,
}

#[derive(Clone)]
pub struct IplayerSelection<'a> {
    pub programme: Option<Programme<'a>>,
    pub programme_page_url: Option<&'a str>,
}

impl<'a> IplayerSelection<'a> {
    fn new(inode: IplayerNode<'a>) -> IplayerSelection<'a> {
        let title = inode.title();
        let subtitle = inode.subtitle();
        match inode.programme_site() {
            None => IplayerSelection {
                programme: Some(Programme::new(title, subtitle, inode)),
                programme_page_url: None,
            },
            Some(u) => IplayerSelection {
                programme: None,
                programme_page_url: Some(u),
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

    fn programme(title: Option<String>, node: Option<select::node::Node>) -> Option<Programme> {
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

impl<'a> BeebURL<'a> {
    async fn load(&self) -> BoxResult<IplayerDocument<'a>> {
        let resp = reqwest::get(self.url)
            .await?.text().await?;
        let doc = select::document::Document::from_read(resp)?;
        Ok(IplayerDocument { doc, url: self.url })
    }

    //    async fn load_async(&self) -> BoxResult<IplayerDocument<'a>> {
    //        let uri = url::Url::parse(self.url)?;
    //
    //    }

    // async fn load_async(&self) -> BoxResult<IplayerDocument<'a>> {

    // }
}

fn programme_sites<'a>(nodes: Find<'a, Class<&str>>) -> Vec<IplayerNode<'a>> {
    nodes
        .filter(|node| IplayerNode { node: *node }.programme_site().is_some())
        .map(|node| IplayerNode { node })
        .collect()
}

mod testutils {
    use super::*;

    pub struct TestHTMLURL<'a> {
        pub url: &'a str,
    }

    impl<'a> TestHTMLURL<'a> {
        pub fn load(&self) -> super::BoxResult<IplayerDocument> {
            let html = fs::read(self.url)?;
            let doc = Document::from_read(&html[..])?;
            Ok(IplayerDocument { doc, url: self.url })
        }
    }
}

//async fn collect_pages<'a>(urls: Vec<BeebURL<'_>>) -> Vec<BoxResult<IplayerDocument<'a>>> {
//    let mut idocs: Vec<BoxResult<IplayerDocument>> = Vec::new();
//    thread::scope(|s| {
//        for url in &urls {
//            s.spawn(move |_| {
//                idocs.push(url.load())
//            });
//        }
//    }).unwrap();
//
//    idocs
//}

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
        let sites = programme_sites(nodes);
        assert_eq!(sites.len(), 3);
        assert_eq!(
            sites[0].programme_site().unwrap(),
            "testhtml/adam_curtis.html"
        );
        assert_eq!(
            sites[1].programme_site().unwrap(),
            "/iplayer/episodes/b08kfrzk/home-from-home-chronicle-of-a-vision"
        );
        let tu = testutils::TestHTMLURL {
            url: "testhtml/food1.html",
        };
        let idr = tu.load();
        assert!(idr.is_ok());
        let id = idr.unwrap();
        let nodes = id.doc.find(Class("content-item"));
        let sites = programme_sites(nodes);
        assert_eq!(sites.len(), 31);
        assert_eq!(
            sites[0].programme_site().unwrap(),
            "/iplayer/episodes/b052hdnr/a-cook-abroad"
        );
        assert_eq!(
            sites[1].programme_site().unwrap(),
            "/iplayer/episodes/b0863g11/best-bakes-ever"
        );
        assert_eq!(
            sites[2].programme_site().unwrap(),
            "/iplayer/episodes/b00mh31r/caribbean-food-made-easy"
        );
        assert_eq!(
            sites[3].programme_site().unwrap(),
            "/iplayer/episodes/m00077h7/the-chefs-brigade"
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
        assert_eq!(isels.len(), 36);
        let prog_sites = isels.iter().filter(|sel| sel.programme_page_url.is_some());
        assert_eq!(prog_sites.count(), 3);
        let progs = isels.iter().filter(|sel| sel.programme.is_some());
        assert_eq!(progs.count(), 33);
        let tu = TestHTMLURL {
            url: "testhtml/food1.html",
        };
        let idr = tu.load();
        assert!(idr.is_ok());
        let id = idr.unwrap();
        let isel = id.iplayer_selections();
        assert_eq!(isel.len(), 36);
        let prog_sites = isel.iter().filter(|sel| sel.programme_page_url.is_some());
        assert_eq!(prog_sites.count(), 31);
        let progs = isels.iter().filter(|sel| sel.programme.is_none());
        assert_eq!(progs.count(), 3);
    }

    #[test]
    fn test_next_pages() {
        let tu = testutils::TestHTMLURL {
            url: "testhtml/films1.html",
        };
        let idr = tu.load();
        assert!(idr.is_ok());
        let idoc = idr.unwrap();
        let np = idoc.next_pages();
        assert_eq!(np.len(), 1);
        assert_eq!(np[0].url, "?page=2");
        let tu = testutils::TestHTMLURL {
            url: "testhtml/food1.html",
        };
        let idr = tu.load();
        assert!(idr.is_ok());
        let id = idr.unwrap();
        let np = id.next_pages();
        assert_eq!(np.len(), 0);
    }

    #[test]
    fn test_boxset() {
        let tu = testutils::TestHTMLURL {
            url: "testhtml/peaky_blinders.html",
        };
        let idr = tu.load();
        assert!(idr.is_ok());
        let idoc = idr.unwrap();
        assert!(idoc.is_boxset());
        let tu = testutils::TestHTMLURL {
            url: "testhtml/adam_curtis.html",
        };
        let idr = tu.load();
        assert_eq!(idr.unwrap().is_boxset(), false);
        let tu = testutils::TestHTMLURL {
            url: "testhtml/gentleman_jack.html",
        };
        let idr = tu.load();
        let id = idr.unwrap();
        assert_eq!(id.clone().is_boxset(), false);
        let urls = id.series_urls();
        assert_eq!(urls.len(), 0);

        let tu = testutils::TestHTMLURL {
            url: "testhtml/peaky_blinders.html",
        };
        let idr = tu.load();
        let id = idr.unwrap();
        assert_eq!(id.clone().is_boxset(), true);
        let urls = id.series_urls();
        assert_eq!(urls.len(), 3);
        assert_eq!(
            urls[0].url,
            "/iplayer/episodes/b045fz8r/peaky-blinders?seriesId=b04kkm8q",
        );
        assert_eq!(
            urls[1].url,
            "/iplayer/episodes/b045fz8r/peaky-blinders?seriesId=b079vp14"
        );
        assert_eq!(
            urls[2].url,
            "/iplayer/episodes/b045fz8r/peaky-blinders?seriesId=p05hgs13"
        );

        let tu = testutils::TestHTMLURL {
            url: "testhtml/wrong_mans.html",
        };
        let idr = tu.load();
        let id = idr.unwrap();
        assert_eq!(id.clone().is_boxset(), true);
        let urls = id.series_urls();
        assert_eq!(urls.len(), 1);
        assert_eq!(
            urls[0].url,
            "/iplayer/episodes/p02bhkmm/the-wrong-mans?seriesId=p02bhlq2"
        )
    }

    #[test]
    fn test_programme_pages() {
        let tu = testutils::TestHTMLURL {
            url: "testhtml/films1.html",
        };
        let idr = tu.load();
        assert!(idr.is_ok());
        let idoc = idr.unwrap();
        let isel = idoc.iplayer_selections();
        let progpages = idoc.clone().programme_pages(isel);
        assert_eq!(progpages.len(), 3);
        assert_eq!(progpages[0].url, "testhtml/adam_curtis.html");
        assert_eq!(
            progpages[1].url,
            "/iplayer/episodes/b08kfrzk/home-from-home-chronicle-of-a-vision"
        );
        let tu = testutils::TestHTMLURL {
            url: "testhtml/food1.html",
        };
        let idr = tu.load();
        let idoc = idr.unwrap();
        let isel = idoc.iplayer_selections();
        let progpages = idoc.clone().programme_pages(isel);
        assert_eq!(progpages.len(), 31);
        assert_eq!(progpages[0].url, "/iplayer/episodes/b052hdnr/a-cook-abroad");
        assert_eq!(
            progpages[1].url,
            "/iplayer/episodes/b0863g11/best-bakes-ever"
        );
        assert_eq!(
            progpages[2].url,
            "/iplayer/episodes/b00mh31r/caribbean-food-made-easy"
        );
        assert_eq!(
            progpages[3].url,
            "/iplayer/episodes/m00077h7/the-chefs-brigade"
        );
    }
}
