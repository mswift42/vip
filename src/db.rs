extern crate serde;
extern crate serde_json;
extern crate time;


use tv::Category;
#[derive(Debug)]
pub struct ProgrammeDB {
    pub categories: Vec<Category>,
    pub saved: time::Tm,
}

impl ProgrammeDB {
    pub fn new(cats: Vec<Category>) -> ProgrammeDB {
        ProgrammeDB {
            categories: cats,
            saved: time::now(),
        }
    }

    pub fn save(&self) {

    }

    fn index(&mut self) {
        let mut index: u32 =  0;
        for i in &mut self.categories {
            for j  in i.programmes.iter_mut() {
                j.update_index(index);
                index += 1;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tv::*;

    #[test]
    fn test_programme_db() {
        let doc = IplayerDocument::new(include_str!("../testhtml/pop.html"));
        let progs = doc.programmes();
        let cat = Category::new("mostpopular".to_string(), progs);
        let mut db = ProgrammeDB::new(vec![cat]);
        assert_eq!(db.categories[0].programmes[0].title, "Strike");
        assert_eq!(db.categories[0].name, "mostpopular");
        db.index();
        assert_eq!(db.categories[0].programmes[1].index, 1);
        assert_eq!(db.categories[0].programmes[2].index, 2);

    }


}
