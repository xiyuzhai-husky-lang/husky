mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use self::trai_for_ty_impl_block::*;
pub use self::ty_impl_block::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
#[enum_class::from_variants]
pub enum ImplBlockDeclarativeSignature {
    TypeImpl(TypeImplBlockSignature),
    TraitForTypeImpl(TraitForTypeImplBlockDeclarativeSignature),
}

impl HasDeclarativeSignature for ImplBlockDecl {
    type DeclarativeSignature = ImplBlockDeclarativeSignature;

    fn declarative_signature(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignature> {
        match self {
            ImplBlockDecl::Type(decl) => decl.declarative_signature(db).map(Into::into),
            ImplBlockDecl::TraitForType(decl) => decl.declarative_signature(db).map(Into::into),
        }
    }
}

impl ImplBlockDeclarativeSignature {
    pub fn implicit_parameters(
        self,
        _db: &dyn DeclarativeSignatureDb,
    ) -> &[ImplicitParameterSignature] {
        todo!()
    }
}
