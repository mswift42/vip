extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate time;

use std::fs::File;
use std::io::prelude::*;
use chrono::prelude::*;
use tv::Category;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgrammeDB {
    pub categories: Vec<Category>,
    pub saved: DateTime<Utc>,
}

impl ProgrammeDB {
    pub fn new(cats: Vec<Category>) -> ProgrammeDB {
        ProgrammeDB {
            categories: cats,
            saved: Utc::now(),
        }
    }

    pub fn from_saved() -> ProgrammeDB {
        let decoded = serde_json::from_reader(File::open("testdb.json").unwrap()).unwrap();
        decoded
    }

    pub fn save(&mut self) {
        self.index();
        let serialized = serde_json::to_string(self).unwrap();
        let mut file = match File::open("/home/martin/github/vip/src/testdb.json") {
            Err(why) => panic!("{:}", why),
            Ok(file) => file,
        };
        file.write_all(serialized.as_bytes());
    }

    fn index(&mut self) {
        let mut index: u32 = 0;
        for i in &mut self.categories {
            for j in i.programmes.iter_mut() {
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
        db.save();
        assert_eq!(db.categories[0].programmes[1].index, 1);
        assert_eq!(db.categories[0].programmes[2].index, 2);
    }



}
