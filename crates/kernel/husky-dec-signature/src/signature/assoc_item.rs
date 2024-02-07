mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use super::*;

type SmallVecImpl<T> = SmallVec<[T; 2]>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum AssocItemDecTemplate {
    TypeItem(TypeItemDecTemplate),
    TraitItem(TraitItemDecTemplate),
    TraitForTypeItem(TraitForTypeItemDecTemplate),
}

impl AssocItemDecTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        match self {
            AssocItemDecTemplate::TypeItem(decl) => decl.template_parameters(db),
            AssocItemDecTemplate::TraitItem(decl) => decl.template_parameters(db),
            AssocItemDecTemplate::TraitForTypeItem(_) => todo!(),
        }
    }
}

impl HasDecTemplate for AssocItemPath {
    type DecTemplate = AssocItemDecTemplate;

    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecTemplate> {
        Ok(match self {
            AssocItemPath::TypeItem(path) => path.dec_template(db)?.into(),
            AssocItemPath::TraitItem(path) => path.dec_template(db)?.into(),
            AssocItemPath::TraitForTypeItem(path) => path.dec_template(db)?.into(),
        })
    }
}
