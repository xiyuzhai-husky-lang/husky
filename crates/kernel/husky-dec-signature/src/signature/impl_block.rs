pub mod trai_for_ty_impl_block;
pub mod ty_impl_block;

use self::trai_for_ty_impl_block::*;
use self::ty_impl_block::*;
use super::*;
use husky_entity_path::path::impl_block::ImplBlockPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum ImplBlockDecTemplate {
    TypeImpl(TypeImplBlockDecTemplate),
    TraitForTypeImpl(TraitForTypeImplBlockDecTemplate),
}

impl HasDecTemplate for ImplBlockPath {
    type DecTemplate = ImplBlockDecTemplate;

    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecTemplate> {
        Ok(match self {
            ImplBlockPath::TypeImplBlock(path) => path.dec_template(db)?.into(),
            ImplBlockPath::TraitForTypeImplBlock(path) => path.dec_template(db)?.into(),
        })
    }
}

impl ImplBlockDecTemplate {
    pub fn template_parameters(self, _db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        todo!()
    }
}
