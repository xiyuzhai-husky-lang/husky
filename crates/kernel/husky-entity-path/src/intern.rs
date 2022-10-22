use super::EntityPath;
use husky_word::Identifier;
use interner::{DefaultInternedPtr, Interner};
pub type EntityPathItd = DefaultInternedPtr<EntityPath, EntityPath>;

pub type EntityPathInterner = Interner<EntityPathItd>;

pub trait InternEntityPath {
    fn entity_path_itr(&self) -> &EntityPathInterner;
    fn it_entity_path(&self, pth: EntityPath) -> EntityPathItd {
        self.entity_path_itr().intern(pth)
    }
}

impl InternEntityPath for EntityPathInterner {
    fn entity_path_itr(&self) -> &EntityPathInterner {
        self
    }
}

pub fn new_entity_path_itr() -> EntityPathInterner {
    EntityPathInterner::new_empty()
}

#[test]
fn it_works() {
    let itr = new_entity_path_itr();
}
