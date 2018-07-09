#![feature(test,custom_attribute)]
extern crate chrono;
#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate select;
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name, Predicate};

#[macro_use]
extern crate serde_derive;
mod db;
mod tv;
fn main() {
    //  PROFILER.lock().unwrap().start("mainprofile").unwrap();
    //let doc = tv::IplayerDocument::new(include_str!("../testhtml/pop.html"));
    //let progs = doc.programmes();
    // PROFILER.lock().unwrap().stop().unwrap();
    //let doc2 = Document::from(include_str!("../testhtml/comedy1.html"));
    //    for node in doc2.find(Class("page").descendant(Name("a"))) {
    //        println!("{:?}", node.attr("href"))
    //    }
    // let np =
    //        doc2.find(Class("page").descendant(Name("a")))
    //            .filter_map( |n| n.attr("href"));
    //    for i in np {
    //        println!("{}", i);
    //    }
    //    let pr = tv::IplayerDocument::new(include_str!("../testhtml/pop.html"));
    //    println!("{:?}", pr.programmes());
    run();
}

#[async]
fn fetch_documents() {}

fn run() -> reqwest::Result<()> {
    let mpophtml = reqwest::get("http://www.bbc.co.uk/iplayer/group/most-popular")?;
    let popdoc = Document::from_read(mpophtml).unwrap();
    let idoc = tv::IplayerDocument { idoc: popdoc };
    let programmes = idoc.programmes();
    let titles: Vec<(&str, &str)> = programmes
        .iter()
        .map(|i| (&*i.title, &*i.synopsis))
        .collect();
    for i in titles {
        println!("Programme: {:?}\n", i);
    }
    let cat = tv::Category::new("mostpopular".to_string(), idoc.programmes());
    let mut db = db::ProgrammeDB::new(vec![cat]);
    db.save();
    let db2 = db::ProgrammeDB::from_saved();
    let comedyhtml = reqwest::get("http://www.bbc.co.uk/iplayer/categories/comedy/all?sort=atoz")?;
    let comedydoc = Document::from_read(comedyhtml).unwrap();
    let comedyidoc = tv::IplayerDocument { idoc: comedydoc };
    let comedyprogrammes = comedyidoc.programmes();
    let titles: Vec<(&str, &str)> = comedyprogrammes
        .iter()
        .map(|i| (&*i.title, &*i.synopsis))
        .collect();
    for i in titles {
        println!("Programme: {:?}\n", i);
    }
    println!("{:?}", db2);
    Ok(())
}


error_chain! {
   foreign_links {
       ReqError(reqwest::Error);
       IoError(std::io::Error);
   }
}

//fn run2() -> Result<()> {
//    let res = reqwest::get("https://www.rust-lang.org/en-US/")?;
//
//    let document = Document::from_read(res)?;
//
//    let links = document.find(Name("a"))
//        .filter_map(|n| n.attr("href"));
//
//    for link in links {
//        println!("{}", link);
//    }
//
//    Ok(())
//}
