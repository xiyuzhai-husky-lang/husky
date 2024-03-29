mod assoc_fn;
mod assoc_ty;
mod assoc_val;
mod memo_field;
mod method_fn;

pub use self::assoc_fn::*;
pub use self::assoc_ty::*;
pub use self::assoc_val::*;
pub use self::method_fn::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum TraitItemDecTemplate {
    AssocFn(TraitAssocFnDecTemplate),
    MethodFn(TraitMethodFnDecTemplate),
    AssocType(TraitAssocTypeDecTemplate),
    AssocVal(TraitAssocValDecTemplate),
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
pub enum TraitItemDecTemplates {
    AssocFn(SmallVecImpl<TraitAssocFnDecTemplate>),
    MethodFn(SmallVecImpl<TraitMethodFnDecTemplate>),
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

// #[salsa::tracked(jar = DecSignatureJar)]
pub(crate) fn trai_item_syn_dec_template(
    db: &::salsa::Db,
    path: TraitItemPath,
) -> DecSignatureResult<TraitItemDecTemplate> {
    let decl = path.syn_decl(db)?;
    match decl {
        TraitItemSynDecl::AssocFn(decl) => {
            TraitAssocFnDecTemplate::from_decl(db, decl).map(Into::into)
        }
        TraitItemSynDecl::MethodFn(decl) => {
            TraitMethodFnDecTemplate::from_decl(db, decl).map(Into::into)
        }
        TraitItemSynDecl::AssocType(decl) => {
            TraitAssocTypeDecTemplate::from_decl(db, path, decl).map(Into::into)
        }
        TraitItemSynDecl::AssocVal(decl) => {
            TraitAssocValDecTemplate::from_decl(db, decl).map(Into::into)
        }
    }
}

impl TraitItemDecTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        match self {
            TraitItemDecTemplate::AssocFn(slf) => slf.template_parameters(db),
            TraitItemDecTemplate::MethodFn(slf) => slf.template_parameters(db),
            TraitItemDecTemplate::AssocType(slf) => {
                // slf.template_parameters(db)
                &[]
            }
            TraitItemDecTemplate::AssocVal(slf) => &[],
        }
    }
}
