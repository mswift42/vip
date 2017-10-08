extern crate serde;
extern crate serde_json;
extern crate time;


use tv::Category;
pub struct ProgrammeDB<'a> {
    pub categoryies: Vec<Category<'a>>,
    pub saved: time::Tm,
}

impl<'a> ProgrammeDB<'a> {
    pub fn new<'b>(cats: Vec<Category<'a>>, saved_at: time::Tm) -> ProgrammeDB {
        ProgrammeDB { categoryies: cats, saved: saved_at}
    }
}