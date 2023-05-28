use super::*;

/// wrappers are special applications
///
/// we treat them separately because we need to apply special reduction and avoid toolchain
#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar, constructor = new)]
pub struct DeclarativeTermWrapper {
    pub kind: DeclarativeTermWrapperKind,
    pub inner_ty: DeclarativeTerm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeclarativeTermWrapperKind {
    ValType,
}

impl DeclarativeTerm {
    pub fn leashed_ty(self, db: &dyn DeclarativeTermDb) -> Self {
        DeclarativeTermWrapper::new(db, DeclarativeTermWrapperKind::ValType, self).into()
    }
}

impl DeclarativeTermWrapper {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn DeclarativeTermDb,
        ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        match self.kind(db) {
            DeclarativeTermWrapperKind::ValType => f.write_str("{val_type} ")?,
        }
        self.inner_ty(db).show_with_db_fmt(f, db, ctx)
    }
}
