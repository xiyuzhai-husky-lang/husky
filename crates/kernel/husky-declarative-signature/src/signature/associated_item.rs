mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use super::*;

type SmallVecImpl<T> = SmallVec<[T; 2]>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
#[enum_class::from_variants]
pub enum AssociatedItemDeclarativeSignatureTemplate {
    TypeItem(TypeItemDeclarativeSignatureTemplate),
    TraitItem(TraitItemDeclarativeSignatureTemplate),
    TraitForTypeItem(TraitForTypeItemDeclarativeSignatureTemplate),
}

impl AssociatedItemDeclarativeSignatureTemplate {
    pub fn template_parameters(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> &[DeclarativeTemplateParameter] {
        match self {
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(decl) => {
                decl.template_parameters(db)
            }
            AssociatedItemDeclarativeSignatureTemplate::TraitItem(decl) => {
                decl.template_parameters(db)
            }
            AssociatedItemDeclarativeSignatureTemplate::TraitForTypeItem(_) => todo!(),
        }
    }
}

impl HasDeclarativeSignatureTemplate for AssociatedItemPath {
    type DeclarativeSignatureTemplate = AssociatedItemDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        Ok(match self {
            AssociatedItemPath::TypeItem(path) => path.declarative_signature_template(db)?.into(),
            AssociatedItemPath::TraitItem(path) => path.declarative_signature_template(db)?.into(),
            AssociatedItemPath::TraitForTypeItem(path) => {
                path.declarative_signature_template(db)?.into()
            }
        })
    }
}
