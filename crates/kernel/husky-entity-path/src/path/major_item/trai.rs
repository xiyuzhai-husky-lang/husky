use salsa::DisplayWithDb;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[salsa::as_id(jar = EntityPathJar)]
#[salsa::deref_id]
pub struct TraitPath(ItemPathId);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TraitPathData {
    module_path: ModulePath,
    ident: Ident,
    connection: MajorItemConnection,
}

impl TraitPath {
    pub fn new(
        module_path: ModulePath,
        ident: Ident,
        connection: MajorItemConnection,
        db: &::salsa::Db,
    ) -> Self {
        Self(ItemPathId::new(
            db,
            ItemPathData::MajorItem(MajorItemPathData::Trait(TraitPathData {
                module_path,
                ident,
                connection,
            })),
        ))
    }

    #[inline(never)]
    pub fn show(self, db: &::salsa::Db) -> String {
        self.display(db).to_string()
    }

    pub fn data(self, db: &::salsa::Db) -> TraitPathData {
        match self.0.data(db) {
            ItemPathData::MajorItem(MajorItemPathData::Trait(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.data(db).ident
    }

    pub fn show_aux(self, f: &mut std::fmt::Formatter<'_>, db: &::salsa::Db) -> std::fmt::Result {
        self.data(db).show_aux(f, db)
    }
}

impl TraitPathData {
    #[inline(always)]
    pub(super) fn item_path(self, id: ItemPathId) -> TraitPath {
        TraitPath(id)
    }

    pub fn show_aux(self, f: &mut std::fmt::Formatter<'_>, db: &::salsa::Db) -> std::fmt::Result {
        self.module_path.show_aux(f, db)?;
        f.write_str(show_connection(self.connection))?;
        f.write_str(self.ident.data(db))
    }

    pub fn module_path(&self) -> ModulePath {
        self.module_path
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn connection(&self) -> MajorItemConnection {
        self.connection
    }
}

impl salsa::DebugWithDb for TraitPath {
    fn debug_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        f.write_str("TraitPath(`")?;
        self.show_aux(f, db)?;
        f.write_str("`)")
    }
}

impl salsa::DisplayWithDb for TraitPath {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_aux(f, db)
    }
}
