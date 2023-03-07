mod ty_as_trai_im;
mod ty_im;

pub use ty_as_trai_im::*;
pub use ty_im::*;

use super::*;

pub(crate) fn im_signature(db: &dyn SignatureDb, decl: ImplDecl) -> SignatureResult<ImplSignature> {
    match decl {
        ImplDecl::Type(decl) => ty_im_signature(db, decl).map(Into::into),
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
    pub fn implicit_parameters(self, db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        todo!()
    }
}
