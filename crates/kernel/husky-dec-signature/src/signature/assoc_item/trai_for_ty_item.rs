mod assoc_ritchie;
mod assoc_ty;
mod assoc_val;
mod method_ritchie;

pub use self::assoc_ritchie::*;
pub use self::assoc_ty::*;
pub use self::assoc_val::*;
pub use self::method_ritchie::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum TraitForTypeItemDecTemplate {
    AssocRitchie(TraitForTypeAssocRitchieDecTemplate),
    MethodRitchie(TraitForTypeMethodRitchieDecTemplate),
    AssocType(TraitForTypeAssocTypeDecTemplate),
    AssocVal(TraitForTypeAssocValDecTemplate),
}

impl HasDecTemplate for TraitForTypeItemPath {
    type DecTemplate = TraitForTypeItemDecTemplate;

    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecTemplate> {
        trai_for_ty_item_syn_declarative_signature_from_decl(db, self)
    }
}

#[salsa::tracked(jar = DecSignatureJar)]
pub(crate) fn trai_for_ty_item_syn_declarative_signature_from_decl(
    db: &::salsa::Db,
    path: TraitForTypeItemPath,
) -> DecSignatureResult<TraitForTypeItemDecTemplate> {
    let decl = path.syn_decl(db)?;
    match decl {
        TraitForTypeItemSynDecl::AssocRitchie(decl) => {
            TraitForTypeAssocRitchieDecTemplate::from_decl(db, decl).map(Into::into)
        }
        TraitForTypeItemSynDecl::MethodFn(decl) => {
            TraitForTypeMethodRitchieDecTemplate::from_decl(db, decl).map(Into::into)
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
            TraitForTypeItemDecTemplate::AssocRitchie(tmpl) => tmpl.template_parameters(db),
            TraitForTypeItemDecTemplate::MethodRitchie(tmpl) => tmpl.template_parameters(db),
            TraitForTypeItemDecTemplate::AssocType(tmpl) => tmpl.template_parameters(db),
            TraitForTypeItemDecTemplate::AssocVal(tmpl) => &[],
        }
    }
}
