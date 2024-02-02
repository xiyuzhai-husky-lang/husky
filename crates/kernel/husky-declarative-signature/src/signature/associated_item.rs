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
pub enum AssociatedItemDecTemplate {
    TypeItem(TypeItemDecTemplate),
    TraitItem(TraitItemDecTemplate),
    TraitForTypeItem(TraitForTypeItemDecTemplate),
}

impl AssociatedItemDecTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        match self {
            AssociatedItemDecTemplate::TypeItem(decl) => decl.template_parameters(db),
            AssociatedItemDecTemplate::TraitItem(decl) => decl.template_parameters(db),
            AssociatedItemDecTemplate::TraitForTypeItem(_) => todo!(),
        }
    }
}

impl HasDecTemplate for AssociatedItemPath {
    type DecTemplate = AssociatedItemDecTemplate;

    fn declarative_signature_template(
        self,
        db: &::salsa::Db,
    ) -> DeclarativeSignatureResult<Self::DecTemplate> {
        Ok(match self {
            AssociatedItemPath::TypeItem(path) => path.declarative_signature_template(db)?.into(),
            AssociatedItemPath::TraitItem(path) => path.declarative_signature_template(db)?.into(),
            AssociatedItemPath::TraitForTypeItem(path) => {
                path.declarative_signature_template(db)?.into()
            }
        })
    }
}
