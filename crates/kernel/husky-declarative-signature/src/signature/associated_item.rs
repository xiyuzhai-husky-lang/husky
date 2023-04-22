mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use trai_for_ty_item::*;
pub use trai_item::*;
pub use ty_item::*;

use super::*;

type SmallVecImpl<T> = SmallVec<[T; 2]>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
#[enum_class::from_variants]
pub enum AssociatedItemDeclarativeSignatureTemplate {
    TypeItem(TypeItemDeclarativeSignatureTemplate),
    TraitItem(TraitItemDeclarativeSignatureTemplate),
    TraitForTypeItem(TraitForTypeItemDeclarativeSignatureTemplate),
}

pub(crate) fn associated_item_declarative_signature_from_decl(
    db: &dyn DeclarativeSignatureDb,
    decl: AssociatedItemDecl,
) -> DeclarativeSignatureResult<AssociatedItemDeclarativeSignatureTemplate> {
    match decl {
        AssociatedItemDecl::TypeItem(decl) => {
            ty_item_declarative_signature_from_decl(db, decl).map(|s| s.into())
        }
        AssociatedItemDecl::TraitItem(decl) => {
            trai_associated_item_declarative_signature_from_decl(db, decl).map(|s| s.into())
        }
        AssociatedItemDecl::TraitForTypeItem(decl) => {
            trai_for_ty_associated_item_declarative_signature_from_decl(db, decl).map(|s| s.into())
        } // TypeDecl::Enum(decl) => enum_ty_declarative_signature_template(db, decl).into(),
    }
}

impl AssociatedItemDeclarativeSignatureTemplate {
    pub fn implicit_parameters(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> &[ImplicitParameterSignature] {
        match self {
            AssociatedItemDeclarativeSignatureTemplate::TypeItem(decl) => {
                decl.implicit_parameters(db)
            }
            AssociatedItemDeclarativeSignatureTemplate::TraitItem(decl) => {
                decl.implicit_parameters(db)
            }
            AssociatedItemDeclarativeSignatureTemplate::TraitForTypeItem(_) => todo!(),
        }
    }
}
