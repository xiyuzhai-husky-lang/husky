mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum TraitItemDecTemplate {
    AssociatedFn(TraitAssociatedFnDecTemplate),
    MethodFn(TraitMethodFnDecTemplate),
    AssociatedType(TraitAssociatedTypeDecTemplate),
    AssociatedVal(TraitAssociatedValDecTemplate),
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum TraitItemDecTemplates {
    AssociatedFn(SmallVecImpl<TraitAssociatedFnDecTemplate>),
    MethodFn(SmallVecImpl<TraitMethodFnDecTemplate>),
    AssociatedType(SmallVecImpl<TraitAssociatedTypeDecTemplate>),
    AssociatedVal(SmallVecImpl<TraitAssociatedValDecTemplate>),
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
        TraitItemSynDecl::AssociatedFn(decl) => {
            TraitAssociatedFnDecTemplate::from_decl(db, decl).map(Into::into)
        }
        TraitItemSynDecl::MethodFn(decl) => {
            TraitMethodFnDecTemplate::from_decl(db, decl).map(Into::into)
        }
        TraitItemSynDecl::AssociatedType(decl) => {
            TraitAssociatedTypeDecTemplate::from_decl(db, decl).map(Into::into)
        }
        TraitItemSynDecl::AssociatedVal(decl) => {
            TraitAssociatedValDecTemplate::from_decl(db, decl).map(Into::into)
        }
    }
}

impl TraitItemDecTemplate {
    pub fn template_parameters(self, db: &::salsa::Db) -> &[DeclarativeTemplateParameter] {
        match self {
            TraitItemDecTemplate::AssociatedFn(slf) => slf.template_parameters(db),
            TraitItemDecTemplate::MethodFn(slf) => slf.template_parameters(db),
            TraitItemDecTemplate::AssociatedType(slf) => {
                // slf.template_parameters(db)
                &[]
            }
            TraitItemDecTemplate::AssociatedVal(slf) => &[],
        }
    }
}
