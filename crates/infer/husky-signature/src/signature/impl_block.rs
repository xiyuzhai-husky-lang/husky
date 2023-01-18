mod ty_as_trai_impl_block;
mod ty_impl_block;

pub use ty_as_trai_impl_block::*;
pub use ty_impl_block::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ImplBlockSignature {
    TypeImplBlock(TypeImplBlockSignature),
    TypeAsTraitImplBlock(TypeAsTraitImplBlockSignature),
}

impl From<TypeAsTraitImplBlockSignature> for ImplBlockSignature {
    fn from(v: TypeAsTraitImplBlockSignature) -> Self {
        Self::TypeAsTraitImplBlock(v)
    }
}

impl From<TypeImplBlockSignature> for ImplBlockSignature {
    fn from(v: TypeImplBlockSignature) -> Self {
        Self::TypeImplBlock(v)
    }
}

impl ImplBlockSignature {
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        todo!()
    }

    pub fn expr_page(self, db: &dyn SignatureDb) -> ExprPage {
        match self {
            ImplBlockSignature::TypeImplBlock(decl) => decl.expr_page(db),
            ImplBlockSignature::TypeAsTraitImplBlock(decl) => decl.expr_page(db),
        }
    }
}

impl<Db: SignatureDb + ?Sized> salsa::DebugWithDb<Db> for ImplBlockSignature {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<SignatureJar>>::as_jar_db(db);
        match self {
            ImplBlockSignature::TypeImplBlock(decl) => f
                .debug_tuple("TypeImplBlock")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            ImplBlockSignature::TypeAsTraitImplBlock(decl) => f
                .debug_tuple("TypeAsTraitImplBlock")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
        }
    }
}
