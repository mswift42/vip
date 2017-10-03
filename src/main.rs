#[macro_use]
extern crate error_chain;
extern crate select;
extern crate reqwest;
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name, Predicate};

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

fn run() -> reqwest::Result<()> {

    let mpophtml = reqwest::get("http://www.bbc.co.uk/iplayer/group/most-popular")?;
    let popdoc = Document::from_read(mpophtml).unwrap();
    let idoc = tv::IplayerDocument{idoc: popdoc};
    let programmes = idoc.programmes();
    let titles: Vec<(&str, &str)> = programmes.iter().map(|i| (&*i.title, &*i.synopsis)).collect();
    println!("{:?}", titles);
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

