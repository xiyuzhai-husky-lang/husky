use crate::*;
use husky_hir_decl::db::HirDeclDb;

pub trait HirDefnDb: salsa::DbWithJar<HirDefnJar> + HirDeclDb {}

impl<Db> HirDefnDb for Db where Db: salsa::DbWithJar<HirDefnJar> + HirDeclDb {}

#[salsa::jar(db = HirDefnDb)]
pub struct HirDefnJar(
    // type
    ty_hir_defn,
    EnumTypeHirDefn,
    UnitStructTypeHirDefn,
    TupleStructTypeHirDefn,
    PropsStructTypeHirDefn,
    ExternTypeHirDefn,
    UnionTypeHirDefn,
    // fugitive
    fugitive_hir_defn,
    ValHirDefn,
    FunctionFnHirDefn,
    FunctionGnHirDefn,
    // morphism_defn,
    TypeAliasHirDefn,
    // type_alias_defn,
    // trait
    TraitHirDefn,
    trai_hir_defn,
    // enum variant,
    EnumUnitVariantHirDefn,
    EnumTupleVariantHirDefn,
    EnumPropsVariantHirDefn,
    // type item
    ty_item_hir_defn,
    TypeAssociatedFnHirDefn,
    TypeMethodFnHirDefn,
    TypeAssociatedTypeHirDefn,
    TypeAssociatedValHirDefn,
    TypeMemoizedFieldHirDefn,
    // trait item
    trai_item_hir_defn,
    TraitAssociatedFnHirDefn,
    TraitMethodFnHirDefn,
    TraitAssociatedTypeHirDefn,
    TraitAssociatedValHirDefn,
    // trait for type item
    trai_for_ty_item_hir_defn,
    TraitForTypeAssociatedFnHirDefn,
    TraitForTypeMethodFnHirDefn,
    TraitForTypeAssociatedTypeHirDefn,
    TraitForTypeAssociatedValHirDefn,
);
