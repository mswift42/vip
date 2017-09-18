extern crate select;
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name, Predicate};

type BeebUrl<'a> = &'a str;

type TestBeebUrl = &'static str;

pub struct Category<'a> {
    name: String,
    programmes: Vec<&'a Programme>,
}

impl<'a> Category<'a> {
    pub fn new(name: String, programmes: Vec<&'a Programme>) -> Category<'a> {
        Category { name, programmes }
    }
}

#[derive(Debug)]
pub struct Programme {
    pub title: String,
    pub subtitle: Option<String>,
    pub synopsis: String,
    pub pid: String,
    pub thumbnail: String,
    pub url: String,
    pub index: u16,
}
impl Programme {
    fn new(
        title: String,
        subtitle: Option<String>,
        synopsis: String,
        pid: String,
        thumbnail: String,
        url: String,
        index: u16,
    ) -> Programme {
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
#[derive(Clone, Debug)]
pub struct IplayerDocument {
    pub idoc: Document,
}

impl IplayerDocument {
    pub fn new(bu: TestBeebUrl) -> IplayerDocument {
        let idoc = Document::from(bu);
        IplayerDocument { idoc }
    }

    pub fn programmes(self) -> Vec<Programme> {
        let mut results: Vec<Programme> = Vec::new();
        for node in self.idoc.find(Class("list-item-inner")) {
            let title = find_title(&node);
            let subtitle = find_subtitle(&node);
            let synopsis = find_synopsis(&node);
            let pid = find_pid(&node);
            let thumbnail = find_thumbnail(&node).to_string();
            let url = find_url(&node);
            let index = 0;
            let prog = Programme::new(title, subtitle, synopsis, pid, thumbnail, url, index);
            results.push(prog);
        }
        results
    }

    fn sub_pages(self) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        for node in self.idoc.find(Class("view-more-container")) {
            let sub = node.attr("href").unwrap().to_string();
            results.push(String::from("http://www.bbc.co.uk") + &sub);
        }
        results
    }

    fn next_pages(self) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        for node in self.idoc.find(Class("page").descendant(Name("a"))) {
            let nxt = node.attr("href").unwrap().to_string();
            results.push(String::from("http://www.bbc.co.uk") + &nxt);
        }
        results
    }

}

fn find_title(node: &Node) -> String {
    node.find(Class("secondary").descendant(Class("title")))
        .next()
        .unwrap()
        .text()
}

fn find_subtitle(node: &Node) -> Option<String> {
    let sub = node.find(Class("secondary").descendant(Class("subtitle")))
        .next();
    match sub {
        None => None,
        Some(text) => Some(text.text()),
    }
}

fn find_url(node: &Node) -> String {
    let path = node.find(Name("a"))
        .next()
        .unwrap()
        .attr("href")
        .unwrap()
        .to_string();
    if path.starts_with("http://www.bbc.co.uk") {
        return path
    } else {
        let url = String::from("http://www.bbc.co.uk");
        return url + &path
    }
}

fn find_thumbnail<'a>(node: &'a Node) -> &'a str {
    node.find(
        Class("rs-image").descendant(Name("picture").descendant(Name("source"))),
    ).next()
        .unwrap()
        .attr("srcset")
        .unwrap()
}

fn find_pid(node: &Node) -> String {
    match node.attr("data-ip-id") {
        None => node.find(Class("list-item-inner").descendant(Name("a")))
            .next()
            .unwrap()
            .attr("data-episode-id")
            .unwrap()
            .to_string(),
        Some(pid) => pid.to_string(),
    }
}

fn find_synopsis(node: &Node) -> String {
    node.find(Class("synopsis")).next().unwrap().text()
}

#[cfg(test)]
mod test {
    use super::Programme;
    use super::Category;
    use super::IplayerDocument;
    use super::{Class, Name};
    use super::Document;

    #[test]
    fn test_document() {
        let doc = IplayerDocument::new(include_str!("../testhtml/pop.html"));
        assert_eq!(
            doc.idoc.find(Name("h1")).next().unwrap().text(),
            "Most Popular"
        );
        assert_eq!(
            doc.idoc.find(Class("subtitle")).next().unwrap().text(),
            "Today's most popular programmes available on BBC iPlayer."
        );
    }
    #[test]
    fn test_programmes() {
        let doc = IplayerDocument::new(include_str!("../testhtml/pop.html"));
        let progr = doc.programmes();
        assert_eq!(progr[0].title, "Strike");
        assert_eq!(progr[0].subtitle, Some("The Silkworm: Episode 1".to_string()));
        assert_eq!(progr[0].pid, "b0959ppk");
        assert_eq!(
            progr[0].url,
            "http://www.bbc.co.uk/iplayer/episode/b0959ppk/strike-the-silkworm-episode-1"
        );
    }
    #[test]
    fn test_find_title() {
        let doc = IplayerDocument::new(include_str!("../testhtml/pop.html"));
        let prog = doc.programmes();
        assert_eq!(prog[0].title, "Strike");
        assert_eq!(prog[1].title, "Doctor Foster");
        assert_eq!(prog[2].title, "Strictly Come Dancing");
        let doc = IplayerDocument::new(include_str!("../testhtml/films1.html"));
        let prog = doc.programmes();
        assert_eq!(prog[0].title, "Adam Curtis");
        assert_eq!(prog[1].title, "Broken");
        assert_eq!(prog[2].title, "Echoes from the Dead");
        assert_eq!(prog[3].title, "Emma");
        let doc = IplayerDocument::new(include_str!("../testhtml/comedy1.html"));
        let prog = doc.programmes();
        assert_eq!(prog[0].title, "Asian Network Comedy");
        assert_eq!(prog[1].title, "Bad Education");
        assert_eq!(prog[2].title, "BBC New Comedy Award");
        assert_eq!(prog[3].title, "Being Human");
    }

    #[test]
    fn test_find_subtitle() {
        let doc = IplayerDocument::new(include_str!("../testhtml/pop.html"));
        let prog = doc.programmes();
        assert_eq!(prog[0].subtitle, Some("The Silkworm: Episode 1".to_string()));
        assert_eq!(prog[1].subtitle, Some("Series 2: Episode 1".to_string()));
        assert_eq!(prog[2].subtitle, Some("Series 15: 1. Launch".to_string()));
        assert_eq!(prog[39].subtitle, Some("04/09/2017".to_string()));


        let doc = IplayerDocument::new(include_str!("../testhtml/films1.html"));
        let prog = doc.programmes();
        assert_eq!(prog[0].subtitle, Some("HyperNormalisation".to_string()));
        assert_eq!(prog[1].subtitle, None);
        assert_eq!(prog[2].subtitle, None);

        let doc = IplayerDocument::new(include_str!("../testhtml/comedy1.html"));
        let prog = doc.programmes();
        assert_eq!(prog[0].subtitle, Some("Live in Edinburgh 2017".to_string()));
        assert_eq!(prog[1].subtitle, Some("Series 3: 6. The Finale".to_string()));
        assert_eq!(prog[2].subtitle, Some("2017: Live from Edinburgh".to_string()));
    }

    #[test]
    fn test_find_pid() {
        let doc = IplayerDocument::new(include_str!("../testhtml/pop.html"));
        let prog = doc.programmes();
        assert_eq!(prog[0].pid, "b0959ppk");
        assert_eq!(prog[1].pid, "b094m49d");
        assert_eq!(prog[2].pid, "b0957wrf");
        assert_eq!(prog[3].pid, "b0956h5y");

        let doc = IplayerDocument::new(include_str!("../testhtml/films1.html"));
        let prog = doc.programmes();
        assert_eq!(prog[0].pid, "");
        assert_eq!(prog[1].pid, "");
    }

    #[test]
    fn test_thumbnail() {
        let doc = IplayerDocument::new(include_str!("../testhtml/pop.html"));
        let prog = doc.programmes();
        assert_eq!(prog[0].thumbnail, "https://ichef.bbci.co.uk/images/ic/336x189/p05f6rgl.jpg");
        assert_eq!(prog[1].thumbnail, "https://ichef.bbci.co.uk/images/ic/336x189/p05fdxqf.jpg");
        assert_eq!(prog[2].thumbnail, "https://ichef.bbci.co.uk/images/ic/336x189/p05fb1zb.jpg");

        let doc = IplayerDocument::new(include_str!("../testhtml/films1.html"));
        let prog = doc.programmes();
        assert_eq!(prog[0].thumbnail, "https://ichef.bbci.co.uk/images/ic/336x189/p04c0tsb.jpg");
        assert_eq!(prog[1].thumbnail, "https://ichef.bbci.co.uk/images/ic/336x189/p02j8jt8.jpg");
        assert_eq!(prog[2].thumbnail, "https://ichef.bbci.co.uk/images/ic/336x189/p02t9h5f.jpg");

        let doc = IplayerDocument::new(include_str!("../testhtml/comedy1.html"));
        let prog = doc.programmes();
        assert_eq!(prog[0].thumbnail, "https://ichef.bbci.co.uk/images/ic/336x189/p05cyq28.jpg");
        assert_eq!(prog[1].thumbnail, "https://ichef.bbci.co.uk/images/ic/336x189/p028nmdv.jpg");
        assert_eq!(prog[2].thumbnail, "https://ichef.bbci.co.uk/images/ic/336x189/p05ccy3l.jpg");
        assert_eq!(prog[3].thumbnail, "https://ichef.bbci.co.uk/images/ic/336x189/p01j34d4.jpg");
    }
    #[test]
    fn test_find_url() {
        let doc = IplayerDocument::new(include_str!("../testhtml/pop.html"));
        let prog = doc.programmes();
        assert_eq!(prog[0].url, "http://www.bbc.co.uk/iplayer/episode/b0959ppk/strike-the-silkworm-episode-1");
        assert_eq!(prog[1].url, "http://www.bbc.co.uk/iplayer/episode/b094m49d/doctor-foster-series-2-episode-1");
        assert_eq!(prog[2].url, "http://www.bbc.co.uk/iplayer/episode/b0957wrf/strictly-come-dancing-series-15-1-launch");
        assert_eq!(prog[3].url, "http://www.bbc.co.uk/iplayer/episode/b0956h5y/dragons-den-series-15-episode-4");

        let doc = IplayerDocument::new(include_str!("../testhtml/films1.html"));
        let prog = doc.programmes();
        assert_eq!(prog[1].url, "http://www.bbc.co.uk/iplayer/episode/b03bm29q/broken");
        assert_eq!(prog[0].url, "http://www.bbc.co.uk/iplayer/episode/p04b183c/adam-curtis-hypernormalisation");
        assert_eq!(prog[2].url, "http://www.bbc.co.uk/iplayer/episode/b04lp7xn/echoes-from-the-dead");

        let doc = IplayerDocument::new(include_str!("../testhtml/comedy1.html"));
        let prog = doc.programmes();
        assert_eq!(prog[0].url, "http://www.bbc.co.uk/iplayer/episode/p05by123/asian-network-comedy-live-in-edinburgh-2017");
        assert_eq!(prog[1].url, "http://www.bbc.co.uk/iplayer/episode/b04m9twt/bad-education-series-3-6-the-finale");
        assert_eq!(prog[2].url, "http://www.bbc.co.uk/iplayer/episode/b0920yy0/bbc-new-comedy-award-2017-live-from-edinburgh");
        assert_eq!(prog[3].url, "http://www.bbc.co.uk/iplayer/episode/b01r82f3/being-human-series-5-6-the-last-broadcast");
    }
    #[test]
    fn test_sub_pages() {
        let doc = IplayerDocument::new(include_str!("../testhtml/films1.html"));
        let sub_pages = doc.sub_pages();
        assert_eq!(sub_pages[0], "http://www.bbc.co.uk/iplayer/episodes/p04bkttz");

        let doc = IplayerDocument::new(include_str!("../testhtml/comedy1.html"));
        let sub_pages = doc.sub_pages();
        assert_eq!(sub_pages.len(), 10);
        assert_eq!(sub_pages[0], "http://www.bbc.co.uk/iplayer/episodes/p01djw5m");
        assert_eq!(sub_pages[1], "http://www.bbc.co.uk/iplayer/episodes/b00hqlc4");
    }

    #[test]
    fn test_next_page() {
        let doc = IplayerDocument::new(include_str!("../testhtml/comedy1.html"));
        let next_pages = doc.next_pages();
        assert_eq!(next_pages[0],
                   "http://www.bbc.co.uk/iplayer/categories/comedy/all?sort=atoz&page=2");
        let doc = IplayerDocument::new(include_str!("../testhtml/films1.html"));
        let next_pages = doc.next_pages();
        assert_eq!(next_pages[0],
                   "http://www.bbc.co.uk/iplayer/categories/films/all?sort=atoz&page=2");

    }

}
