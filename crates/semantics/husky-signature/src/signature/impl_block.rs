mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use self::trai_for_ty_impl_block::*;
pub use self::ty_impl_block::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = SignatureDb, jar = SignatureJar)]
#[enum_class::from_variants]
pub enum ImplSignature {
    TypeImpl(TypeImplBlockSignature),
    TraitForTypeImpl(TraitForTypeImplBlockSignature),
}

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

impl ImplSignature {
    pub fn implicit_parameters(self, _db: &dyn SignatureDb) -> &[ImplicitParameterSignature] {
        todo!()
    }
}
