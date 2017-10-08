extern crate serde;
extern crate serde_json;
extern crate time;


use tv::Category;
#[derive(Debug)]
pub struct ProgrammeDB<'a> {
    pub categories: Vec<Category<'a>>,
    pub saved: time::Tm,
}

impl<'a> ProgrammeDB<'a> {
    pub fn new<'b>(cats: Vec<Category<'a>>) -> ProgrammeDB {
        ProgrammeDB { categories: cats, saved: time::now()}
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tv::*;

    #[test]
    fn test_programme_db() {
        let doc = IplayerDocument::new(include_str!("../testhtml/pop.html"))
        let progs = doc.programmes();
        let cat := Category::new("mostpopular", &progs);
        assert_eq!(progr[0].title, "Strike");
        let db = ProgrammeDB::new(cat);
        assert_eq!(db.)
    }
}