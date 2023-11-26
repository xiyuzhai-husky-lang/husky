use vec_like::VecPairMap;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
// #[salsa::as_id(jar =  EntityPathJar)]
#[salsa::deref_id]
pub struct AttrItemPath(ItemPathId);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct AttrItemPathData {
    // todo: change type to AttrParentPath
    parent: ItemPath,
    // ad hoc
    // todo: change it with OriginalAttrPath
    ident: Ident,
    disambiguator: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum AttrParentPath {
    Type(TypePath),
}

impl AttrItemPath {
    pub fn data(self, db: &::salsa::Db) -> AttrItemPathData {
        match self.0.data(db) {
            ItemPathData::Attr(data) => data,
            _ => unreachable!(),
        }
    }

    pub fn parent(self, db: &::salsa::Db) -> ItemPath {
        self.data(db).parent
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.data(db).ident
    }
}

impl AttrItemPathData {
    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.parent.module_path(db)
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
        self.module_path(db).toolchain(db)
    }
}

#[derive(Debug)]
pub struct AttrRegistry {
    parent: ItemPath,
    next_attr_disambiguators: VecPairMap<Ident, u8>,
}

impl AttrRegistry {
    pub fn new(parent: ItemPath) -> Self {
        Self {
            parent,
            next_attr_disambiguators: Default::default(),
        }
    }
    pub fn issue(&mut self, ident: Ident, db: &::salsa::Db) -> AttrItemPath {
        let next_disambiguator = self
            .next_attr_disambiguators
            .get_value_mut_or_insert_default(ident);
        let disambiguator = *next_disambiguator;
        *next_disambiguator += 1;
        AttrItemPath(ItemPathId::new(
            db,
            ItemPathData::Attr(AttrItemPathData {
                parent: self.parent,
                ident,
                disambiguator,
            }),
        ))
    }
}
