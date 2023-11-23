use husky_hir_defn::HasHirDefn;
use husky_hir_ty::{HirTemplateArguments, HirTemplateArgument};

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
        template_arguments: LinkageTemplateArguments,
    },
    PropsStructField,
    MemoizedField,
    Index,
    Method,
}

impl Linkage {
    pub fn new_suffix(db: &dyn LinkageDb) -> Self {
        todo!()
    }

    pub fn new_props_struct_field(db: &dyn LinkageDb) -> Self {
        Self::new(db, LinkagePathData::PropsStructField)
    }

    pub fn new_memoized_field(db: &dyn LinkageDb) -> Self {
        Self::new(db, LinkagePathData::MemoizedField)
    }

    pub fn new_method(db: &dyn LinkageDb) -> Self {
        Self::new(db, LinkagePathData::Method)
    }

    pub fn new_index(db: &dyn LinkageDb) -> Self {
        Self::new(db, LinkagePathData::Index)
    }

    pub fn new_item(
        path: impl Into<ItemPath>,
        template_arguments: &[HirTemplateArgument],
        db: &dyn LinkageDb,
    ) -> Self {
        Self::new(
            db,
            LinkagePathData::Item {
                path: path.into(),
                template_arguments: LinkageTemplateArgument::from_hir_template_arguments(
                    template_arguments,
                    db,
                ),
            },
        )
    }
}

impl<Db: ?Sized> HasVersionStamp<Db> for Linkage
where
    Db: LinkageDb,
{
    type VersionStamp = LinkageVersionStamp;

    fn version_stamp(self, db: &Db) -> LinkageVersionStamp {
        let db = <Db as salsa::DbWithJar<LinkageJar>>::as_jar_db(db);
        let mut builder = LinkageVersionStampBuilder::new(self, db);
        match self.data(db) {
            LinkagePathData::Coersion {} => (),
            LinkagePathData::Item {
                path,
                template_arguments,
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
