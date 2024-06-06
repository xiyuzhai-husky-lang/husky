pub mod compterm;
pub mod function_ritchie;
pub mod static_mut;
pub mod static_var;
pub mod ty_alias;
pub mod val;

use self::{compterm::*, function_ritchie::*, static_mut::*, static_var::*, ty_alias::*, val::*};
use super::*;
use husky_entity_path::path::major_item::form::MajorFormPath;
use husky_syn_decl::decl::major_item::form::FormSynDecl;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum MajorFormHirDecl {
    Ritchie(MajorFunctionRitchieHirDecl),
    Val(MajorValHirDecl),
    Compterm(MajorComptermHirDecl),
    StaticMut(MajorStaticMutHirDecl),
    StaticVar(MajorStaticVarHirDecl),
    TypeAlias(MajorTypeAliasHirDecl),
}

impl MajorFormHirDecl {
    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> Option<&'a HirTemplateParameters> {
        match self {
            MajorFormHirDecl::Ritchie(decl) => Some(decl.template_parameters(db)),
            MajorFormHirDecl::Val(_decl) => None,
            MajorFormHirDecl::TypeAlias(_) => todo!(),
            MajorFormHirDecl::Compterm(_decl) => None,
            MajorFormHirDecl::StaticMut(_) => None,
            MajorFormHirDecl::StaticVar(_) => None,
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> HirExprRegion {
        match self {
            MajorFormHirDecl::Ritchie(slf) => slf.hir_expr_region(db).into(),
            MajorFormHirDecl::Val(slf) => slf.hir_eager_expr_region(db).into(),
            MajorFormHirDecl::TypeAlias(slf) => slf.hir_eager_expr_region(db).into(),
            MajorFormHirDecl::Compterm(_) => todo!(),
            MajorFormHirDecl::StaticMut(slf) => slf.hir_eager_expr_region(db).into(),
            MajorFormHirDecl::StaticVar(slf) => slf.hir_eager_expr_region(db).into(),
        }
    }

    pub fn path(self, db: &::salsa::Db) -> MajorFormPath {
        match self {
            MajorFormHirDecl::Ritchie(decl) => decl.path(db),
            MajorFormHirDecl::Val(decl) => decl.path(db),
            MajorFormHirDecl::TypeAlias(decl) => decl.path(db),
            MajorFormHirDecl::Compterm(_) => todo!(),
            MajorFormHirDecl::StaticMut(_) => todo!(),
            MajorFormHirDecl::StaticVar(_) => todo!(),
        }
    }
}

impl HasHirDecl for MajorFormPath {
    type HirDecl = MajorFormHirDecl;

    fn hir_decl(self, db: &::salsa::Db) -> Option<Self::HirDecl> {
        major_form_hir_decl(db, self)
    }
}

#[salsa::tracked]
fn major_form_hir_decl(db: &::salsa::Db, path: MajorFormPath) -> Option<MajorFormHirDecl> {
    match path.syn_decl(db).expect("no errors for hir stage") {
        FormSynDecl::Ritchie(syn_decl) => {
            Some(MajorFunctionRitchieHirDecl::from_syn(path, syn_decl, db).into())
        }
        FormSynDecl::Val(syn_decl) => Some(MajorValHirDecl::from_syn(path, syn_decl, db).into()),
        // todo: reconsider?
        FormSynDecl::TypeAlias(_) => None,
        // todo: reconsider?
        FormSynDecl::TypeVar(_) => None,
        FormSynDecl::Compterm(syn_decl) => {
            Some(MajorComptermHirDecl::from_syn(path, syn_decl, db).into())
        }
        FormSynDecl::StaticMut(syn_decl) => {
            Some(MajorStaticMutHirDecl::from_syn(path, syn_decl, db).into())
        }
        FormSynDecl::StaticVar(syn_decl) => {
            Some(MajorStaticVarHirDecl::from_syn(path, syn_decl, db).into())
        }
    }
}
