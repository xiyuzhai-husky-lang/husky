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
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum TraitForTypeItemDecTemplate {
    AssociatedFn(TraitForTypeAssociatedFnDecTemplate),
    MethodFn(TraitForTypeMethodFnDecTemplate),
    AssociatedType(TraitForTypeAssociatedTypeDecTemplate),
    AssociatedVal(TraitForTypeAssociatedValDecTemplate),
}

impl HasDecTemplate for TraitForTypeItemPath {
    type DecTemplate = TraitForTypeItemDecTemplate;

    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecTemplate> {
        trai_for_ty_item_syn_declarative_signature_from_decl(db, self)
    }
}

// #[salsa::tracked(jar = DecSignatureJar)]
pub(crate) fn trai_for_ty_item_syn_declarative_signature_from_decl(
    db: &::salsa::Db,
    path: TraitForTypeItemPath,
) -> DecSignatureResult<TraitForTypeItemDecTemplate> {
    let decl = path.syn_decl(db)?;
    match decl {
        TraitForTypeItemSynDecl::AssociatedFn(decl) => {
            TraitForTypeAssociatedFnDecTemplate::from_decl(db, decl).map(Into::into)
        }
        TraitForTypeItemSynDecl::MethodFn(decl) => {
            TraitForTypeMethodFnDecTemplate::from_decl(db, decl).map(Into::into)
        }
        TraitForTypeItemSynDecl::AssociatedType(decl) => {
            TraitForTypeAssociatedTypeDecTemplate::from_decl(db, path, decl).map(Into::into)
        }
        TraitForTypeItemSynDecl::AssociatedVal(decl) => {
            TraitForTypeAssociatedValDecTemplate::from_decl(db, decl).map(Into::into)
        }
    }
}

impl TraitForTypeItemDecTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        match self {
            TraitForTypeItemDecTemplate::AssociatedFn(tmpl) => tmpl.template_parameters(db),
            TraitForTypeItemDecTemplate::MethodFn(tmpl) => tmpl.template_parameters(db),
            TraitForTypeItemDecTemplate::AssociatedType(tmpl) => tmpl.template_parameters(db),
            TraitForTypeItemDecTemplate::AssociatedVal(tmpl) => &[],
        }
    }
}
