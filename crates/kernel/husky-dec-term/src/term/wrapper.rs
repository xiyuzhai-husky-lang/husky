use super::*;

/// wrappers are special applications
///
/// we treat them separately because we need to apply special reduction and avoid toolchain
#[salsa::interned(constructor = new)]
pub struct DecWrapper {
    pub kind: DecTermWrapperKind,
    pub inner_ty: DecTerm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecTermWrapperKind {
    ValType,
    VarType,
}

impl DecTerm {
    pub fn val_ty(self, db: &::salsa::Db) -> Self {
        DecWrapper::new(db, DecTermWrapperKind::ValType, self).into()
    }

    pub fn var_ty(self, db: &::salsa::Db) -> Self {
        DecWrapper::new(db, DecTermWrapperKind::VarType, self).into()
    }
}

impl DecWrapper {
    #[inline(never)]
    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &DecSymbolicVariableNameMap,
    ) -> std::fmt::Result {
        match self.kind(db) {
            DecTermWrapperKind::ValType => f.write_str("{val_type} ")?,
            DecTermWrapperKind::VarType => f.write_str("{var_type} ")?,
        }
        self.inner_ty(db).display_fmt_with_db_and_ctx(f, db, ctx)
    }
}
