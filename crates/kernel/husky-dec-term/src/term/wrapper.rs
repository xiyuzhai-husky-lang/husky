use super::*;

/// wrappers are special applications
///
/// we treat them separately because we need to apply special reduction and avoid toolchain
#[salsa::interned(db = DecTermDb, jar = DecTermJar, constructor = new)]
pub struct DecWrapper {
    pub kind: DecTermWrapperKind,
    pub inner_ty: DecTerm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecTermWrapperKind {
    ValReturnType,
}

impl DecTerm {
    pub fn leashed_ty(self, db: &::salsa::Db) -> Self {
        DecWrapper::new(db, DecTermWrapperKind::ValReturnType, self).into()
    }
}

impl DecWrapper {
    #[inline(never)]
    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &DecSymbolNameMap,
    ) -> std::fmt::Result {
        match self.kind(db) {
            DecTermWrapperKind::ValReturnType => f.write_str("{val_type} ")?,
        }
        self.inner_ty(db).display_fmt_with_db_and_ctx(f, db, ctx)
    }
}
