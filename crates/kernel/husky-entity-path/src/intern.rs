use super::*;
use husky_word::Identifier;
use interner::{DefaultItd, Interned, Interner};
use optional::{Noned, OptEq};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct EntityPathItd(DefaultItd<EntityPath, EntityPath>);

impl std::fmt::Debug for EntityPathItd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("`")?;
        std::fmt::Display::fmt(self, f)?;
        f.write_str("`")
    }
}

impl std::fmt::Display for EntityPathItd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.opt_parent.into_option() {
            Some(parent) => {
                parent.fmt(f)?;
                "::".fmt(f)?
            }
            None => (),
        }
        self.ident.fmt(f)
    }
}

impl EntityPathItd {
    pub(crate) fn child(self, db: &dyn EntityPathDb, ident: &str) -> Self {
        db.it_child_entity_path(self, ident)
    }
}

impl Interned for EntityPathItd {
    type T = EntityPath;

    type Owned = EntityPath;

    fn new_interned(id: usize, target: &'static Self::T) -> Self {
        Self(DefaultItd::new_interned(id, target))
    }

    fn new_itr() -> Interner<Self> {
        Interner::new_empty()
    }

    fn opt_atom_itd(t: &Self::T) -> Option<Self> {
        None
    }
}

impl std::borrow::Borrow<EntityPath> for EntityPathItd {
    fn borrow(&self) -> &EntityPath {
        &self.0
    }
}

impl std::ops::Deref for EntityPathItd {
    type Target = EntityPath;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Noned for EntityPathItd {
    fn is_none(&self) -> bool {
        self.0.is_none()
    }

    fn get_none() -> Self {
        Self(DefaultItd::get_none())
    }
}

impl OptEq for EntityPathItd {
    fn opt_eq(&self, other: &Self) -> bool {
        self.0.opt_eq(&other.0)
    }
}

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
