mod trai_for_ty_impl_block;
mod ty_impl_block;

pub use self::trai_for_ty_impl_block::*;
pub use self::ty_impl_block::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum ImplBlockDecTemplate {
    TypeImpl(TypeImplBlockDecTemplate),
    TraitForTypeImpl(TraitForTypeImplBlockDecTemplate),
}

impl HasDecTemplate for ImplBlockPath {
    type DecTemplate = ImplBlockDecTemplate;

    fn declarative_signature_template(
        self,
        db: &::salsa::Db,
    ) -> DeclarativeSignatureResult<Self::DecTemplate> {
        Ok(match self {
            ImplBlockPath::TypeImplBlock(path) => path.declarative_signature_template(db)?.into(),
            ImplBlockPath::TraitForTypeImplBlock(path) => {
                path.declarative_signature_template(db)?.into()
            }
        })
    }
}

impl ImplBlockDecTemplate {
    pub fn template_parameters(self, _db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        todo!()
    }
}
