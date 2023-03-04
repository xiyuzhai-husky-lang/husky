mod ty_as_trai_im;
mod ty_im;

pub use ty_as_trai_im::*;
pub use ty_im::*;

use super::*;

pub(crate) fn im_raw_signature(
    db: &dyn RawSignatureDb,
    decl: ImplDecl,
) -> RawSignatureResultRef<ImplRawSignature> {
    match decl {
        ImplDecl::Type(decl) => ty_im_raw_signature(db, decl).as_ref().map(|s| (*s).into()),
        ImplDecl::TypeAsTrait(decl) => ty_as_trai_im_raw_signature(db, decl)
            .as_ref()
            .map(|s| (*s).into()), // TypeDecl::Union(decl) => union_ty_raw_signature(db, decl).into(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = RawSignatureDb, jar = RawSignatureJar)]
pub enum ImplRawSignature {
    TypeImpl(TypeImplRawSignature),
    TypeAsTraitImpl(TypeAsTraitImplRawSignature),
}

impl From<TypeAsTraitImplRawSignature> for ImplRawSignature {
    fn from(v: TypeAsTraitImplRawSignature) -> Self {
        Self::TypeAsTraitImpl(v)
    }
}

impl From<TypeImplRawSignature> for ImplRawSignature {
    fn from(v: TypeImplRawSignature) -> Self {
        Self::TypeImpl(v)
    }
}

impl ImplRawSignature {
    pub fn implicit_parameters(self, db: &dyn RawSignatureDb) -> &[ImplicitParameterRawSignature] {
        todo!()
    }
}
