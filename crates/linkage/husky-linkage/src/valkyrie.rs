use crate::{instantiation::LinkageInstantiation, path::LinkageItemPath, *};
use husky_entity_path::ItemPathId;
use husky_entity_syn_tree::helpers::paths::module_item_paths;
use husky_hir_decl::{parameter::template::HirTemplateParameters, HasHirDecl};
use husky_hir_defn::HasHirDefn;
use husky_hir_eager_expr::{HirEagerExprData, HirEagerExprRegion};
use husky_hir_expr::HirExprRegion;
use husky_hir_lazy_expr::{HirLazyExprData, HirLazyExprRegion};
use husky_hir_ty::instantiation::HirInstantiation;
use smallvec::ToSmallVec;

/// can be instantiated to a path leading linkage given LinkageInstantiation
#[derive(Debug, PartialEq, Eq)]
pub struct ValkyrieRide {
    linkage_path: LinkageItemPath,
    instantiation: HirInstantiation,
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct ValkyrieRides {
    hir_template_parameters: Option<HirTemplateParameters>,
    rides: Vec<ValkyrieRide>,
}

#[salsa::tracked(jar = LinkageJar, return_ref)]
fn item_valkyrie_rides(db: &::salsa::Db, id: ItemPathId) -> Option<ValkyrieRides> {
    let item_path = id.item_path(db);
    let hir_decl = item_path.hir_decl(db)?;
    let hir_template_parameters = hir_decl.template_parameters(db).map(Clone::clone);
    let mut valkyrie_ride = ValkyrieRides {
        hir_template_parameters,
        rides: vec![],
    };
    valkyrie_ride.add_hir_expr_region(hir_decl.hir_expr_region(db)?, db);
    if let Some(hir_expr_region) = item_path.hir_defn(db)?.hir_expr_region(db) {
        valkyrie_ride.add_hir_expr_region(hir_expr_region, db);
    }
    Some(valkyrie_ride)
}

impl ValkyrieRides {
    fn add_hir_expr_region(&mut self, hir_expr_region: HirExprRegion, db: &::salsa::Db) {
        match hir_expr_region {
            HirExprRegion::Eager(hir_eager_expr_region) => {
                self.add_hir_eager_expr_region(hir_eager_expr_region, db)
            }
            HirExprRegion::Lazy(hir_lazy_expr_region) => {
                self.add_hir_lazy_expr_region(hir_lazy_expr_region, db)
            }
        }
    }

    fn add_hir_eager_expr_region(
        &mut self,
        hir_eager_expr_region: HirEagerExprRegion,
        db: &::salsa::Db,
    ) {
        for data in hir_eager_expr_region.expr_arena(db) {
            match *data {
                HirEagerExprData::AssociatedFn {
                    associated_item_path,
                } => todo!(),
                HirEagerExprData::Literal(_) => todo!(),
                HirEagerExprData::PrincipalEntityPath(_) => todo!(),
                HirEagerExprData::ConstSymbol(_) => todo!(),
                HirEagerExprData::Variable(_) => todo!(),
                HirEagerExprData::Binary { lopd, opr, ropd } => todo!(),
                HirEagerExprData::Be { src, ref target } => todo!(),
                HirEagerExprData::Prefix {
                    opr,
                    opd_hir_expr_idx,
                } => todo!(),
                HirEagerExprData::Suffix {
                    opd_hir_expr_idx,
                    opr,
                } => todo!(),
                HirEagerExprData::TypeConstructorFnCall {
                    path,
                    function_hir_eager_expr_idx,
                    ref template_arguments,
                    ref item_groups,
                } => todo!(),
                HirEagerExprData::TypeVariantConstructorCall {
                    path,
                    function_hir_eager_expr_idx,
                    ref template_arguments,
                    ref item_groups,
                } => todo!(),
                HirEagerExprData::FunctionFnCall {
                    path,
                    function_hir_eager_expr_idx,
                    ref template_arguments,
                    ref item_groups,
                } => todo!(),
                HirEagerExprData::AssociatedFunctionFnCall {
                    path,
                    function_hir_eager_expr_idx,
                    ref parent_template_arguments,
                    ref template_arguments,
                    ref item_groups,
                } => todo!(),
                HirEagerExprData::PropsStructField {
                    owner_hir_expr_idx,
                    ident,
                } => todo!(),
                HirEagerExprData::MemoizedField {
                    owner_hir_expr_idx,
                    ident,
                    path,
                } => todo!(),
                HirEagerExprData::MethodFnCall {
                    self_argument,
                    ident,
                    path,
                    ref template_arguments,
                    ref item_groups,
                } => todo!(),
                HirEagerExprData::NewTuple { ref items } => todo!(),
                HirEagerExprData::Index {
                    owner_hir_expr_idx,
                    ref items,
                } => todo!(),
                HirEagerExprData::NewList { ref items } => todo!(),
                HirEagerExprData::Block { .. } => (),
                HirEagerExprData::EmptyHtmlTag { .. } => (),
                HirEagerExprData::Todo => (),
                HirEagerExprData::Unreachable => (),
            }
        }
    }

    fn add_hir_lazy_expr_region(
        &mut self,
        hir_lazy_expr_region: HirLazyExprRegion,
        db: &::salsa::Db,
    ) {
        for data in hir_lazy_expr_region.hir_lazy_expr_arena(db) {
            match *data {
                HirLazyExprData::AssociatedFn { path } => todo!(),
                HirLazyExprData::PrincipalEntityPath(_) => todo!(),
                HirLazyExprData::Be { src, ref target } =>
                /* ad hoc */
                {
                    ()
                }
                HirLazyExprData::Prefix {
                    opr,
                    opd_hir_expr_idx,
                } => todo!(),
                HirLazyExprData::Suffix {
                    opd_hir_expr_idx,
                    opr,
                } => todo!(),
                HirLazyExprData::TypeConstructorFnCall {
                    path,
                    ref instantiation,
                    ref item_groups,
                } => todo!(),
                HirLazyExprData::TypeVariantConstructorFnCall {
                    path,
                    ref instantiation,
                    ref item_groups,
                } => todo!(),
                HirLazyExprData::FunctionFnItemCall {
                    path,
                    ref instantiation,
                    ref item_groups,
                } => todo!(),
                HirLazyExprData::FunctionGnItemCall {
                    path,
                    ref instantiation,
                    ref item_groups,
                } => todo!(),
                HirLazyExprData::AssociatedFunctionFnCall {
                    path,
                    ref instantiation,
                    ref item_groups,
                } => todo!(),
                HirLazyExprData::PropsStructField { owner, ident } => todo!(),
                HirLazyExprData::MemoizedField {
                    owner,
                    ident,
                    path,
                    ref indirections,
                    ref instantiation,
                } => todo!(),
                HirLazyExprData::MethodFnCall {
                    self_argument,
                    ident,
                    path,
                    ref indirections,
                    ref instantiation,
                    ref item_groups,
                } => todo!(),
                HirLazyExprData::NewTuple { ref items } => todo!(),
                HirLazyExprData::Index { owner, ref items } => todo!(),
                HirLazyExprData::NewList { ref items } => todo!(),
                HirLazyExprData::Block { stmts } => todo!(),
                _ => (),
            }
        }
    }
}

#[test]
fn item_valkyrie_rides_works() {
    DB::default().ast_expect_test_debug_with_db(
        |db, module_path| -> Vec<_> {
            module_item_paths(db, module_path)
                .iter()
                .map(|&item_path| item_valkyrie_rides(db, *item_path))
                .collect()
        },
        &AstTestConfig::new("item_valkyrie_rides"),
    )
}
