use salsa::Database;

use super::*;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::deref_id]
pub struct FugitivePath(ItemPathId);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct FugitivePathData {
    module_path: ModulePath,
    ident: Ident,
    connection: MajorItemConnection,
    fugitive_kind: FugitiveKind,
}

impl FugitivePath {
    pub fn new(
        module_path: ModulePath,
        ident: Ident,
        connection: MajorItemConnection,
        fugitive_kind: FugitiveKind,
        db: &dyn EntityPathDb,
    ) -> Self {
        Self(ItemPathId::new(
            db,
            ItemPathData::MajorItem(MajorItemPathData::Fugitive(FugitivePathData {
                module_path,
                ident,
                connection,
                fugitive_kind,
            })),
        ))
    }

    pub fn data(self, db: &dyn EntityPathDb) -> FugitivePathData {
        match self.0.data(db) {
            ItemPathData::MajorItem(MajorItemPathData::Fugitive(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn ident(self, db: &dyn EntityPathDb) -> Ident {
        self.data(db).ident
    }

    pub fn fugitive_kind(self, db: &dyn EntityPathDb) -> FugitiveKind {
        self.data(db).fugitive_kind
    }

    #[inline(never)]
    pub fn show_aux(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityPathDb,
    ) -> std::fmt::Result {
        self.data(db).show_aux(f, db)
    }
}

impl FugitivePathData {
    pub fn crate_path(self, db: &dyn EntityPathDb) -> CratePath {
        self.module_path.crate_path(db)
    }

    pub fn toolchain(self, db: &dyn EntityPathDb) -> Toolchain {
        self.crate_path(db).toolchain(db)
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

    pub fn fugitive_kind(&self) -> FugitiveKind {
        self.fugitive_kind
    }

    #[inline(never)]
    pub fn show_aux(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityPathDb,
    ) -> std::fmt::Result {
        self.module_path.show_aux(f, db)?;
        f.write_str(show_connection(self.connection))?;
        f.write_str(self.ident.data(db))
    }
}

impl salsa::DebugWithDb for FugitivePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, db: &::salsa::Db) -> std::fmt::Result {
        let data = self.data(db);
        f.write_str("FugitivePath(`")?;
        data.show_aux(f, db())?;
        f.write_str("`, `")?;
        data.fugitive_kind.fmt(f)?;
        f.write_str("`)")
    }
}

impl salsa::DisplayWithDb for FugitivePath {
    fn display_with_db_fmt(&self, f: &mut std::fmt::Formatter<'_>, db: &Db) -> std::fmt::Result {
        self.show_aux(f, db())
    }
}
