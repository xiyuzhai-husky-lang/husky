mod associated_fn;
mod associated_ty;
mod associated_val;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeSignatureDb)]
#[enum_class::from_variants]
pub enum TraitForTypeItemDeclarativeSignatureTemplate {
    AssociatedFn(TraitForTypeAssociatedFnDeclarativeSignatureTemplate),
    MethodFn(TraitForTypeMethodFnDeclarativeSignatureTemplate),
    AssociatedType(TraitForTypeAssociatedTypeDeclarativeSignatureTemplate),
    AssociatedVal(TraitForTypeAssociatedValDeclarativeSignatureTemplate),
}

impl HasDeclarativeSignatureTemplate for TraitForTypeItemPath {
    type DeclarativeSignatureTemplate = TraitForTypeItemDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        trai_for_ty_item_syn_declarative_signature_from_decl(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_for_ty_item_syn_declarative_signature_from_decl(
    db: &dyn DeclarativeSignatureDb,
    path: TraitForTypeItemPath,
) -> DeclarativeSignatureResult<TraitForTypeItemDeclarativeSignatureTemplate> {
    let decl = path.syn_decl(db)?;
    match decl {
        TraitForTypeItemSynDecl::AssociatedFn(decl) => {
            TraitForTypeAssociatedFnDeclarativeSignatureTemplate::from_decl(db, decl)
                .map(Into::into)
        }
        TraitForTypeItemSynDecl::MethodFn(decl) => {
            TraitForTypeMethodFnDeclarativeSignatureTemplate::from_decl(db, decl).map(Into::into)
        }
        TraitForTypeItemSynDecl::AssociatedType(decl) => {
            TraitForTypeAssociatedTypeDeclarativeSignatureTemplate::from_decl(db, decl)
                .map(Into::into)
        }
        TraitForTypeItemSynDecl::AssociatedVal(decl) => {
            TraitForTypeAssociatedValDeclarativeSignatureTemplate::from_decl(db, decl)
                .map(Into::into)
        }
    }
}

impl TraitForTypeItemDeclarativeSignatureTemplate {
    pub fn template_parameters(
        self,
        _db: &dyn DeclarativeSignatureDb,
    ) -> &[DeclarativeTemplateParameter] {
        match self {
            TraitForTypeItemDeclarativeSignatureTemplate::AssociatedFn(_) => todo!(),
            TraitForTypeItemDeclarativeSignatureTemplate::MethodFn(_) => todo!(),
            TraitForTypeItemDeclarativeSignatureTemplate::AssociatedType(_) => todo!(),
            TraitForTypeItemDeclarativeSignatureTemplate::AssociatedVal(_) => todo!(),
        }
    }
}
