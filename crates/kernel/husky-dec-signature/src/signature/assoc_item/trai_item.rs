pub mod assoc_ritchie;
pub mod assoc_static_mut;
pub mod assoc_static_var;
pub mod assoc_ty;
pub mod assoc_val;
pub mod memo;
pub mod method_ritchie;

use self::{
    assoc_ritchie::*, assoc_static_mut::*, assoc_static_var::*, assoc_ty::*, assoc_val::*,
    method_ritchie::*,
};
use super::*;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;
use husky_syn_decl::decl::assoc_item::trai_item::TraitItemSynDecl;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum TraitItemDecTemplate {
    AssocRitchie(TraitAssocRitchieDecTemplate),
    MethodRitchie(TraitMethodRitchieDecTemplate),
    AssocType(TraitAssocTypeDecTemplate),
    AssocStaticMut(TraitAssocStaticMutDecTemplate),
    AssocStaticVar(TraitAssocStaticVarDecTemplate),
    AssocVal(TraitAssocValDecTemplate),
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum TraitItemDecTemplates {
    AssocRitchie(SmallVecImpl<TraitAssocRitchieDecTemplate>),
    MethodFn(SmallVecImpl<TraitMethodRitchieDecTemplate>),
    AssocType(SmallVecImpl<TraitAssocTypeDecTemplate>),
    AssocVal(SmallVecImpl<TraitAssocValDecTemplate>),
    // MemoizedField(SmallVecImpl<TraitMemoizedFieldDecTemplate>),
}

impl HasDecTemplate for TraitItemPath {
    type DecTemplate = TraitItemDecTemplate;

    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecTemplate> {
        trai_item_syn_dec_template(db, self)
    }
}

#[salsa::tracked]
pub(crate) fn trai_item_syn_dec_template(
    db: &::salsa::Db,
    path: TraitItemPath,
) -> DecSignatureResult<TraitItemDecTemplate> {
    let decl = path.syn_decl(db)?;
    match decl {
        TraitItemSynDecl::AssocRitchie(decl) => {
            TraitAssocRitchieDecTemplate::from_decl(db, decl).map(Into::into)
        }
        TraitItemSynDecl::MethodFn(decl) => {
            TraitMethodRitchieDecTemplate::from_decl(db, decl).map(Into::into)
        }
        TraitItemSynDecl::AssocType(decl) => {
            TraitAssocTypeDecTemplate::from_decl(db, path, decl).map(Into::into)
        }
        TraitItemSynDecl::AssocVal(decl) => {
            TraitAssocValDecTemplate::from_decl(path, decl, db).map(Into::into)
        }
        TraitItemSynDecl::AssocStaticMut(decl) => {
            TraitAssocStaticMutDecTemplate::from_decl(db, decl).map(Into::into)
        }
        TraitItemSynDecl::AssocStaticVar(decl) => {
            TraitAssocStaticVarDecTemplate::from_decl(path, decl, db).map(Into::into)
        }
    }
}

impl TraitItemDecTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        match self {
            TraitItemDecTemplate::AssocRitchie(slf) => slf.template_parameters(db),
            TraitItemDecTemplate::MethodRitchie(slf) => slf.template_parameters(db),
            TraitItemDecTemplate::AssocType(slf) => {
                // slf.template_parameters(db)
                &[]
            }
            TraitItemDecTemplate::AssocVal(_) => &[],
            TraitItemDecTemplate::AssocStaticMut(_) => &[],
            TraitItemDecTemplate::AssocStaticVar(_) => &[],
        }
    }
}
