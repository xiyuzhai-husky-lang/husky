mod assoc_fn;
mod assoc_ty;
mod assoc_val;
mod method_fn;

pub use self::assoc_fn::*;
pub use self::assoc_ty::*;
pub use self::assoc_val::*;
pub use self::method_fn::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum TraitForTypeItemDecTemplate {
    AssocFn(TraitForTypeAssocFnDecTemplate),
    MethodFn(TraitForTypeMethodFnDecTemplate),
    AssocType(TraitForTypeAssocTypeDecTemplate),
    AssocVal(TraitForTypeAssocValDecTemplate),
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
        TraitForTypeItemSynDecl::AssocFn(decl) => {
            TraitForTypeAssocFnDecTemplate::from_decl(db, decl).map(Into::into)
        }
        TraitForTypeItemSynDecl::MethodFn(decl) => {
            TraitForTypeMethodFnDecTemplate::from_decl(db, decl).map(Into::into)
        }
        TraitForTypeItemSynDecl::AssocType(decl) => {
            TraitForTypeAssocTypeDecTemplate::from_decl(db, path, decl).map(Into::into)
        }
        TraitForTypeItemSynDecl::AssocVal(decl) => {
            TraitForTypeAssocValDecTemplate::from_decl(db, decl).map(Into::into)
        }
    }
}

impl TraitForTypeItemDecTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        match self {
            TraitForTypeItemDecTemplate::AssocFn(tmpl) => tmpl.template_parameters(db),
            TraitForTypeItemDecTemplate::MethodFn(tmpl) => tmpl.template_parameters(db),
            TraitForTypeItemDecTemplate::AssocType(tmpl) => tmpl.template_parameters(db),
            TraitForTypeItemDecTemplate::AssocVal(tmpl) => &[],
        }
    }
}
