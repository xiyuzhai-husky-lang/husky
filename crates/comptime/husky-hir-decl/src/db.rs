use crate::*;
use husky_hir_expr::db::*;

pub trait HirDeclDb: salsa::DbWithJar<HirDeclJar> + HirExprDb {}

impl<Db> HirDeclDb for Db where Db: salsa::DbWithJar<HirDeclJar> + HirExprDb {}

#[salsa::jar(db = HirDeclDb)]
pub struct HirDeclJar(
    // associated_items
    // - type items
    ty_item_hir_decl,
    TypeMethodFnHirDecl,
    TypeMethodFunctionHirDecl,
    TypeMemoizedFieldHirDecl,
    TypeAssociatedFnHirDecl,
    // - trait items
    TraitAssociatedFnHirDecl,
    TraitMethodFnHirDecl,
    // - trait for type
    trai_for_ty_item_hir_decl,
    TraitForTypeAssociatedTypeHirDecl,
    TraitForTypeMethodFnHirDecl,
    // ty
    ty_hir_decl,
    EnumHirDecl,
    ExternHirDecl,
    RecordHirDecl,
    PropsStructHirDecl,
    TupleStructHirDecl,
    UnionHirDecl,
    UnitStructHirDecl,
    // ty variant
    ty_variant_hir_decl,
    EnumTupleTypeVariantHirDecl,
    EnumPropsTypeVariantHirDecl,
    EnumUnitTypeVariantHirDecl,
    // impl block
    // - type
    TypeImplBlockHirDecl,
    // - trait for type
    trai_for_ty_impl_block_hir_decl,
    TraitForTypeImplBlockHirDecl,
    // decr
    DeriveDecrHirDecl,
);
