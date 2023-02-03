mod ty_as_trai_impl_block;
mod ty_impl_block;

pub use ty_as_trai_impl_block::*;
pub use ty_impl_block::*;

use super::*;

pub(crate) fn impl_block_signature(
    db: &dyn SignatureDb,
    decl: ImplBlockDecl,
) -> SignatureResultRef<ImplBlockSignature> {
    match decl {
        ImplBlockDecl::TypeImplBlock(decl) => ty_impl_block_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()),
        ImplBlockDecl::TypeAsTraitImplBlock(decl) => ty_as_trai_impl_block_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()), // TypeDecl::Union(decl) => union_ty_signature(db, decl).into(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SignatureDb, jar = SignatureJar)]
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
}
