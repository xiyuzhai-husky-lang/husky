mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use self::trai_for_ty_impl_block::*;
pub use self::ty_impl_block::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
#[enum_class::from_variants]
pub enum ImplBlockDeclarativeSignatureTemplate {
    TypeImpl(TypeImplBlockDeclarativeSignatureTemplate),
    TraitForTypeImpl(TraitForTypeImplBlockDeclarativeSignatureTemplate),
}

impl HasDeclarativeSignatureTemplate for ImplBlockPath {
    type DeclarativeSignatureTemplate = ImplBlockDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        Ok(match self {
            ImplBlockPath::TypeImplBlock(path) => path.declarative_signature_template(db)?.into(),
            ImplBlockPath::TraitForTypeImplBlock(path) => {
                path.declarative_signature_template(db)?.into()
            }
        })
    }
}

impl ImplBlockDeclarativeSignatureTemplate {
    pub fn template_parameters(
        self,
        _db: &dyn DeclarativeSignatureDb,
    ) -> &[DeclarativeTemplateParameter] {
        todo!()
    }
}
