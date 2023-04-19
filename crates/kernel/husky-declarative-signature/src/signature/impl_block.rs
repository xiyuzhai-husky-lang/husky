mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use self::trai_for_ty_impl_block::*;
pub use self::ty_impl_block::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureTemplateJar)]
#[enum_class::from_variants]
pub enum ImplBlockDeclarativeSignatureTemplate {
    TypeImpl(TypeImplBlockSignature),
    TraitForTypeImpl(TraitForTypeImplBlockDeclarativeSignatureTemplate),
}

impl HasDeclarativeSignatureTemplate for ImplBlockDecl {
    type DeclarativeSignatureTemplate = ImplBlockDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        match self {
            ImplBlockDecl::Type(decl) => decl.declarative_signature_template(db).map(Into::into),
            ImplBlockDecl::TraitForType(decl) => {
                decl.declarative_signature_template(db).map(Into::into)
            }
        }
    }
}

struct A {}

impl ImplBlockDeclarativeSignatureTemplate {
    pub fn implicit_parameters(
        self,
        _db: &dyn DeclarativeSignatureDb,
    ) -> &[ImplicitParameterSignature] {
        todo!()
    }
}
