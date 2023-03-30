mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use trai_for_ty_impl_block::*;
pub use ty_impl_block::*;

use super::*;

pub(crate) fn impl_block_signature_from_decl(
    db: &dyn SignatureDb,
    decl: ImplBlockDecl,
) -> SignatureResult<ImplSignature> {
    match decl {
        ImplBlockDecl::Type(decl) => ty_impl_block_signature(db, decl).map(Into::into),
        ImplBlockDecl::TraitForType(decl) => {
            trai_for_ty_impl_block_signature(db, decl).map(Into::into)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SignatureDb, jar = SignatureJar)]
pub enum ImplSignature {
    TypeImpl(TypeImplSignature),
    TraitForTypeImpl(TraitForTypeImplBlockSignature),
}

impl From<TraitForTypeImplBlockSignature> for ImplSignature {
    fn from(v: TraitForTypeImplBlockSignature) -> Self {
        Self::TraitForTypeImpl(v)
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
