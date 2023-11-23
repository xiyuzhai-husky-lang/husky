use crate::*;

#[salsa::interned(db = LinkageDb, jar = LinkageJar, constructor = new)]
pub struct Linkage {
    pub data: LinkagePathData,
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
        db: &dyn LinkageDb,
        path: impl Into<ItemPath>,
        template_arguments: HirTemplateArgumentLiterals,
    ) -> Self {
        Self::new(
            db,
            LinkagePathData::Item {
                path: path.into(),
                template_arguments,
            },
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LinkagePathData {
    Coersion {},
    Item {
        path: ItemPath,
        template_arguments: HirTemplateArgumentLiterals,
    },
    PropsStructField,
    MemoizedField,
    Index,
    Method,
}
