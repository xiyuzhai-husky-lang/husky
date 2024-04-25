use super::*;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::as_id]
#[salsa::deref_id]
pub struct MajorFormPath(ItemPathId);

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct FormPathData {
    module_path: ModulePath,
    ident: Ident,
    connection: MajorItemConnection,
    form_kind: MajorFormKind,
}

impl MajorFormPath {
    pub fn new(
        module_path: ModulePath,
        ident: Ident,
        connection: MajorItemConnection,
        form_kind: MajorFormKind,
        db: &::salsa::Db,
    ) -> Self {
        Self(ItemPathId::new(
            db,
            ItemPathData::MajorItem(MajorItemPathData::Form(FormPathData {
                module_path,
                ident,
                connection,
                form_kind,
            })),
        ))
    }

    pub fn data(self, db: &::salsa::Db) -> FormPathData {
        match self.0.data(db) {
            ItemPathData::MajorItem(MajorItemPathData::Form(data)) => data,
            _ => unreachable!(),
        }
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.data(db).ident
    }

    pub fn major_form_kind(self, db: &::salsa::Db) -> MajorFormKind {
        self.data(db).form_kind
    }

    #[inline(never)]
    pub fn show_aux(self, f: &mut std::fmt::Formatter<'_>, db: &::salsa::Db) -> std::fmt::Result {
        self.data(db).show_aux(f, db)
    }
}

impl FormPathData {
    #[inline(always)]
    pub(super) fn item_path(self, id: ItemPathId) -> MajorFormPath {
        MajorFormPath(id)
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

    pub fn form_kind(&self) -> MajorFormKind {
        self.form_kind
    }

    #[inline(never)]
    pub fn show_aux(self, f: &mut std::fmt::Formatter<'_>, db: &::salsa::Db) -> std::fmt::Result {
        self.module_path.show_aux(f, db)?;
        f.write_str(show_connection(self.connection))?;
        f.write_str(self.ident.data(db))
    }
}

impl salsa::DebugWithDb for MajorFormPath {
    fn debug_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        let data = self.data(db);
        f.write_str("FormPath(`")?;
        data.show_aux(f, db)?;
        f.write_str("`, `")?;
        data.form_kind.fmt(f)?;
        f.write_str("`)")
    }
}

impl salsa::DisplayWithDb for MajorFormPath {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_aux(f, db)
    }
}
