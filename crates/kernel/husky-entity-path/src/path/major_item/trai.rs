use salsa::{Database, DisplayWithDb};

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
        db: &dyn EntityPathDb,
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
    pub fn show(self, db: &dyn EntityPathDb) -> String {
        self.display(db).to_string()
    }

    pub fn data(self, db: &dyn EntityPathDb) -> TraitPathData {
        match self.0.data(db) {
            ItemPathData::MajorItem(MajorItemPathData::Trait(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn ident(self, db: &dyn EntityPathDb) -> Ident {
        self.data(db).ident
    }

    pub fn show_aux(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityPathDb,
    ) -> std::fmt::Result {
        self.data(db).show_aux(f, db)
    }
}

impl TraitPathData {
    pub fn show_aux(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityPathDb,
    ) -> std::fmt::Result {
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, db: &dyn ::salsa::Database) -> std::fmt::Result {
        f.write_str("TraitPath(`")?;
        self.show_aux(f, db.as_jar_db_dyn::<EntityPathJar>())?;
        f.write_str("`)")
    }
}

impl salsa::DisplayWithDb for TraitPath {
    fn display_with_db_fmt(&self, f: &mut std::fmt::Formatter<'_>, db: &Db) -> std::fmt::Result {
        self.show_aux(f, db.as_jar_db_dyn::<EntityPathJar>())
    }
}
