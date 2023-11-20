use crate::*;

#[salsa::interned(db = LinkagePathDb, jar = LinkagePathJar, constructor = new)]
pub struct LinkagePath {
    pub data: LinkagePathData,
}

impl LinkagePath {
    pub fn new_suffix(db: &dyn LinkagePathDb) -> Self {
        todo!()
    }

    pub fn new_field(db: &dyn LinkagePathDb) -> Self {
        Self::new(db, LinkagePathData::Field)
    }

    pub fn new_method(db: &dyn LinkagePathDb) -> Self {
        Self::new(db, LinkagePathData::Method)
    }

    pub fn new_index(db: &dyn LinkagePathDb) -> Self {
        Self::new(db, LinkagePathData::Index)
    }

    pub fn new_item(
        db: &dyn LinkagePathDb,
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
    Field,
    Index,
    Method,
}
