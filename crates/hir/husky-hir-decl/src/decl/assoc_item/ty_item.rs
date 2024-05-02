mod assoc_ritchie;
mod assoc_ty;
mod assoc_val;
mod memo_field;
mod method_ritchie;

pub use self::assoc_ritchie::*;
pub use self::assoc_ty::*;
pub use self::assoc_val::*;
pub use self::memo_field::*;
pub use self::method_ritchie::*;

use super::*;
use husky_entity_path::path::assoc_item::ty_item::TypeItemPath;
use husky_syn_decl::decl::TypeItemSynDecl;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypeItemHirDecl {
    AssocRitchie(TypeAssocRitchieHirDecl),
    AssocType(TypeAssocTypeHirDecl),
    AssocVal(TypeAssocValHirDecl),
    MethodFn(TypeMethodRitchieHirDecl),
    MemoizedField(TypeMemoFieldHirDecl),
}

impl TypeItemHirDecl {
    pub fn path(self, db: &::salsa::Db) -> TypeItemPath {
        match self {
            TypeItemHirDecl::AssocRitchie(decl) => decl.path(db),
            TypeItemHirDecl::MethodFn(decl) => decl.path(db),
            TypeItemHirDecl::AssocType(_) => todo!(),
            TypeItemHirDecl::AssocVal(_) => todo!(),
            TypeItemHirDecl::MemoizedField(decl) => decl.path(db),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> Option<&'a HirTemplateParameters> {
        match self {
            TypeItemHirDecl::AssocRitchie(slf) => Some(slf.template_parameters(db)),
            TypeItemHirDecl::MethodFn(slf) => Some(slf.template_parameters(db)),
            TypeItemHirDecl::AssocType(slf) =>
            /* ad hoc */
            {
                None
            }
            TypeItemHirDecl::AssocVal(slf) => None,
            TypeItemHirDecl::MemoizedField(slf) => None,
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> HirExprRegion {
        match self {
            TypeItemHirDecl::AssocRitchie(decl) => decl.hir_eager_expr_region(db).into(),
            TypeItemHirDecl::MethodFn(decl) => decl.hir_eager_expr_region(db).into(),
            TypeItemHirDecl::AssocType(decl) => decl.hir_eager_expr_region(db).into(),
            TypeItemHirDecl::AssocVal(decl) => decl.hir_expr_region(db).into(),
            TypeItemHirDecl::MemoizedField(decl) => decl.hir_eager_expr_region(db).into(),
        }
    }
}

impl HasHirDecl for TypeItemPath {
    type HirDecl = TypeItemHirDecl;

    fn hir_decl(self, db: &::salsa::Db) -> Option<Self::HirDecl> {
        ty_item_hir_decl(db, self)
    }
}

// #[salsa::tracked]
pub(crate) fn ty_item_hir_decl(db: &::salsa::Db, path: TypeItemPath) -> Option<TypeItemHirDecl> {
    match path.syn_decl(db).expect("ok") {
        TypeItemSynDecl::AssocRitchie(syn_decl) => {
            Some(TypeAssocRitchieHirDecl::from_syn(path, syn_decl, db).into())
        }
        TypeItemSynDecl::MethodFn(syn_decl) => {
            Some(TypeMethodRitchieHirDecl::from_syn(path, syn_decl, db).into())
        }
        TypeItemSynDecl::AssocType(syn_decl) => {
            Some(TypeAssocTypeHirDecl::from_syn(path, syn_decl, db).into())
        }
        TypeItemSynDecl::AssocVal(syn_decl) => {
            Some(TypeAssocValHirDecl::from_syn(path, syn_decl, db).into())
        }
        TypeItemSynDecl::MemoizedField(syn_decl) => {
            Some(TypeMemoFieldHirDecl::from_syn(path, syn_decl, db).into())
        }
    }
}
// TypeItemEthTemplate::AssocRitchie(eth_template) => {
//     Some(TypeAssocRitchieHirDecl::from_syn(path, eth_template, db).into())
// }
// TypeItemEthTemplate::MethodFn(syn_decl) => {
//     Some(TypeMethodRitchieHirDecl::from_syn(path, syn_decl, db).into())
// }
// TypeItemEthTemplate::MethodFunction(_) => None,
// TypeItemEthTemplate::MemoizedField(eth_template) => {
//     Some(TypeMemoFieldHirDecl::from_syn(path, eth_template, db).into())
// }

// TypeItemDecTemplate::AssocRitchie(template) => {
//     TypeAssocRitchieHirDecl::from_dec(db, path, template)?.into()
// }
// TypeItemDecTemplate::MethodFn(template) => {
//     TypeMethodRitchieHirDecl::from_dec(db, template)?.into()
// }
// TypeItemDecTemplate::AssocType(_) => todo!(),
// TypeItemDecTemplate::AssocVal(_) => todo!(),
// TypeItemDecTemplate::MemoizedField(template) => {
//     TypeMemoFieldHirDecl::from_dec(db, template)?.into()
// }
