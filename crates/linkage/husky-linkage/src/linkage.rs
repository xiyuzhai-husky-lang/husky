use crate::*;
use husky_hir_decl::parameter::template::item_hir_template_parameter_stats;
use husky_hir_defn::HasHirDefn;
use husky_hir_ty::{instantiation::HirInstantiation, HirTemplateArgument, HirTemplateArguments};

#[salsa::interned(db = LinkageDb, jar = LinkageJar, constructor = new)]
pub struct Linkage {
    pub data: LinkageData,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LinkageData {
    Coersion {},
    Item {
        item_path: ItemPath,
        instantiation: LinkageInstantiation,
    },
    // todo: merge into Item
    PropsStructField,
    // todo: merge into Item
    MemoizedField,
    // todo: merge into Item
    Index,
    // todo: merge into Item
    Method,
}

impl Linkage {
    pub fn from_item_path(item_path: ItemPath, db: &::salsa::Db) -> Option<Self> {
        let stats = item_hir_template_parameter_stats(db, item_path)?;
        if stats.tys + stats.constants > 0 {
            return None;
        }
        Some(Self::new(
            db,
            LinkageData::Item {
                item_path,
                // ad hoc consider places
                instantiation: LinkageInstantiation::new_first_born(),
            },
        ))
    }

    pub fn new_suffix(db: &::salsa::Db) -> Self {
        todo!()
    }

    pub fn new_props_struct_field(db: &::salsa::Db) -> Self {
        Self::new(db, LinkageData::PropsStructField)
    }

    pub fn new_memoized_field(db: &::salsa::Db) -> Self {
        Self::new(db, LinkageData::MemoizedField)
    }

    pub fn new_method(db: &::salsa::Db) -> Self {
        Self::new(db, LinkageData::Method)
    }

    pub fn new_index(db: &::salsa::Db) -> Self {
        Self::new(db, LinkageData::Index)
    }

    pub fn new_item(
        path: impl Into<ItemPath>,
        instantiation: &HirInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            LinkageData::Item {
                item_path: path.into(),
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
            LinkageData::Coersion {} => (),
            LinkageData::Item {
                item_path: path,
                instantiation,
            } => {
                builder.add(path.hir_defn(db).unwrap());
                todo!()
            }
            LinkageData::PropsStructField => todo!(),
            LinkageData::MemoizedField => todo!(),
            LinkageData::Index => todo!(),
            LinkageData::Method => todo!(),
        }
        todo!()
    }
}
