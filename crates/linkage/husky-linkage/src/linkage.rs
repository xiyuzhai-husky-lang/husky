use husky_hir_defn::HasHirDefn;
use husky_hir_ty::{instantiation::HirInstantiation, HirTemplateArgument, HirTemplateArguments};

use crate::*;

#[salsa::interned(db = LinkageDb, jar = LinkageJar, constructor = new)]
pub struct Linkage {
    pub data: LinkagePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LinkagePathData {
    Coersion {},
    Item {
        path: ItemPath,
        instantiation: LinkageInstantiation,
    },
    PropsStructField,
    MemoizedField,
    Index,
    Method,
}

impl Linkage {
    pub fn new_suffix(db: &::salsa::Db) -> Self {
        todo!()
    }

    pub fn new_props_struct_field(db: &::salsa::Db) -> Self {
        Self::new(db, LinkagePathData::PropsStructField)
    }

    pub fn new_memoized_field(db: &::salsa::Db) -> Self {
        Self::new(db, LinkagePathData::MemoizedField)
    }

    pub fn new_method(db: &::salsa::Db) -> Self {
        Self::new(db, LinkagePathData::Method)
    }

    pub fn new_index(db: &::salsa::Db) -> Self {
        Self::new(db, LinkagePathData::Index)
    }

    pub fn new_item(
        path: impl Into<ItemPath>,
        instantiation: &HirInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            LinkagePathData::Item {
                path: path.into(),
                instantiation: LinkageInstantiation::from_hir(instantiation, db),
            },
        )
    }
}

impl HasVersionStamp for Linkage {
    type VersionStamp = LinkageVersionStamp;

    fn version_stamp(self, db: &::salsa::Db) -> LinkageVersionStamp {
        let mut builder = LinkageVersionStampBuilder::new(self, db);
        match self.data(db) {
            LinkagePathData::Coersion {} => (),
            LinkagePathData::Item {
                path,
                instantiation,
            } => {
                builder.add(path.hir_defn(db).unwrap());
                todo!()
            }
            LinkagePathData::PropsStructField => todo!(),
            LinkagePathData::MemoizedField => todo!(),
            LinkagePathData::Index => todo!(),
            LinkagePathData::Method => todo!(),
        }
        todo!()
    }
}
