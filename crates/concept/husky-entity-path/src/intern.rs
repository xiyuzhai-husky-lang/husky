use super::EntityPath;
use husky_word::Identifier;
use interner::{DefaultInternedPtr, Interner};
pub type EntityPathPtr = DefaultInternedPtr<EntityPath, EntityPath>;

pub type EntityPathInterner = Interner<EntityPathPtr>;

pub trait InternEntityPath {
    fn entity_path_interner(&self) -> &EntityPathInterner;
    fn it_entity_path(&self, pth: EntityPath) -> EntityPathPtr {
        self.entity_path_interner().intern(pth)
    }
    fn it_root_entity_path(&self, ident: Identifier) -> EntityPathPtr {
        self.it_entity_path(EntityPath::root(ident))
    }
}

impl InternEntityPath for EntityPathInterner {
    fn entity_path_interner(&self) -> &EntityPathInterner {
        self
    }
}

pub fn new_entity_path_interner() -> EntityPathInterner {
    EntityPathInterner::new_empty()
}

#[test]
fn it_works() {
    let itr = new_entity_path_interner();
}
