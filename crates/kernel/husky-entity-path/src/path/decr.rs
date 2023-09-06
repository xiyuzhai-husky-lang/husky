use vec_like::VecPairMap;

use crate::*;

#[salsa::interned(db = EntityPathDb, jar = EntityPathJar)]
pub struct DecrPath {
    pub parent: DecrParent,
    pub ident: Ident,
    pub disambiguator: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum DecrParent {
    Type(TypePath),
}

impl DecrPath {
    pub fn module_path(self, db: &dyn EntityPathDb) -> ModulePath {
        match self.parent(db) {
            DecrParent::Type(path) => path.module_path(db),
        }
    }

    pub fn toolchain(self, db: &dyn EntityPathDb) -> Toolchain {
        self.module_path(db).toolchain(db)
    }
}

#[derive(Debug)]
pub struct DecrRegistry {
    parent: DecrParent,
    next_decr_disambiguators: VecPairMap<Ident, u8>,
}

impl DecrRegistry {
    pub fn new(decr_parent: DecrParent) -> Self {
        Self {
            parent: decr_parent,
            next_decr_disambiguators: Default::default(),
        }
    }
    pub fn issue(&mut self, ident: Ident, db: &dyn EntityPathDb) -> DecrPath {
        let next_disambiguator = self
            .next_decr_disambiguators
            .get_value_mut_or_insert_default(ident);
        let disambiguator = *next_disambiguator;
        *next_disambiguator += 1;
        DecrPath::new(db, self.parent, ident, disambiguator)
    }
}
