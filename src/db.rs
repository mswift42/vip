extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate time;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
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
        let path = Path::new("/home/martin/github/vip/src/testdb.json");
        let mut file = match File::open(&path) {
            Err(why) => panic!("{:?}", why),
            Ok(file) => file,
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let decoded: Result<ProgrammeDB, serde_json::Error> = serde_json::from_str(&contents);
        match decoded {
            Err(why) => panic!("{:}", why),
            Ok(dec) => return dec,
        };
    }

    pub fn save(&mut self) {
        self.index();
        let serialized = match serde_json::to_string(self) {
            Err(why) => panic!("{:?}", why),
            Ok(file) => file,
        };
        let path = Path::new("/home/martin/github/vip/src/testdb.json");
        let mut file = match File::create(&path) {
            Err(why) => panic!("{:?}", why),
            Ok(file) => file,
        };
        file.write_all(serialized.as_bytes()).expect("unable to write data");
    }

    fn index(&mut self) {
        let mut index: u32 = 0;
        for i in &mut self.categories {
            for j in &mut i.programmes {
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
    #[test]
    fn test_programme_db_from_saved() {
        let db = ProgrammeDB::from_saved();
        assert_eq!(db.categories[0].programmes[0].title, "EastEnders");
    }

}
