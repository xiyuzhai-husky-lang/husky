mod ty;
mod ty_as_trai;

pub use ty::*;
pub use ty_as_trai::*;

use super::*;

pub(crate) fn im_signature(db: &dyn SignatureDb, decl: ImplDecl) -> SignatureResult<ImplSignature> {
    match decl {
        ImplDecl::Type(decl) => ty_impl_block_signature(db, decl).map(Into::into),
        ImplDecl::TypeAsTrait(decl) => ty_as_trai_im_signature(db, decl).map(Into::into),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SignatureDb, jar = SignatureJar)]
pub enum ImplSignature {
    TypeImpl(TypeImplSignature),
    TypeAsTraitImpl(TypeAsTraitImplSignature),
}

impl From<TypeAsTraitImplSignature> for ImplSignature {
    fn from(v: TypeAsTraitImplSignature) -> Self {
        Self::TypeAsTraitImpl(v)
    }
}

impl From<TypeImplSignature> for ImplSignature {
    fn from(v: TypeImplSignature) -> Self {
        Self::TypeImpl(v)
    }
}

impl ImplSignature {
    pub fn implicit_parameters(self, _db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        todo!()
    }
}
