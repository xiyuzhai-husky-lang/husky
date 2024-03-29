use super::*;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id]
#[salsa::deref_id]
pub struct FugitivePath(ItemPathId);

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct FugitivePathData {
    module_path: ModulePath,
    ident: Ident,
    connection: MajorItemConnection,
    fugitive_kind: MajorFugitiveKind,
}

impl FugitivePath {
    pub fn new(
        module_path: ModulePath,
        ident: Ident,
        connection: MajorItemConnection,
        fugitive_kind: MajorFugitiveKind,
        db: &::salsa::Db,
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

    pub fn data(self, db: &::salsa::Db) -> FugitivePathData {
        match self.0.data(db) {
            ItemPathData::MajorItem(MajorItemPathData::Fugitive(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.data(db).ident
    }

    pub fn major_fugitive_kind(self, db: &::salsa::Db) -> MajorFugitiveKind {
        self.data(db).fugitive_kind
    }

    #[inline(never)]
    pub fn show_aux(self, f: &mut std::fmt::Formatter<'_>, db: &::salsa::Db) -> std::fmt::Result {
        self.data(db).show_aux(f, db)
    }
}

impl FugitivePathData {
    #[inline(always)]
    pub(super) fn item_path(self, id: ItemPathId) -> FugitivePath {
        FugitivePath(id)
    }

    pub fn crate_path(self, db: &::salsa::Db) -> CratePath {
        self.module_path.crate_path(db)
    }

    pub fn toolchain(self, db: &::salsa::Db) -> Toolchain {
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

    pub fn fugitive_kind(&self) -> MajorFugitiveKind {
        self.fugitive_kind
    }

    #[inline(never)]
    pub fn show_aux(self, f: &mut std::fmt::Formatter<'_>, db: &::salsa::Db) -> std::fmt::Result {
        self.module_path.show_aux(f, db)?;
        f.write_str(show_connection(self.connection))?;
        f.write_str(self.ident.data(db))
    }
}

impl salsa::DebugWithDb for FugitivePath {
    fn debug_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        let data = self.data(db);
        f.write_str("FugitivePath(`")?;
        data.show_aux(f, db)?;
        f.write_str("`, `")?;
        data.fugitive_kind.fmt(f)?;
        f.write_str("`)")
    }
}

impl salsa::DisplayWithDb for FugitivePath {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_aux(f, db)
    }
}
