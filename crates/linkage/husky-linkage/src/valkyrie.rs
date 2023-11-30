use crate::{instantiation::LinkageInstantiation, path::LinkageItemPath, *};
use husky_entity_path::{ItemPathId, MajorItemPath, PrincipalEntityPath};
use husky_entity_syn_tree::helpers::paths::module_item_paths;
use husky_hir_decl::{parameter::template::HirTemplateParameters, HasHirDecl};
use husky_hir_defn::HasHirDefn;
use husky_hir_eager_expr::{HirEagerExprData, HirEagerExprRegion};
use husky_hir_expr::HirExprRegion;
use husky_hir_lazy_expr::{HirLazyExprData, HirLazyExprRegion};
use husky_hir_ty::instantiation::HirInstantiation;
use husky_vfs::PackagePath;
use smallvec::ToSmallVec;
use vec_like::VecSet;

/// can be instantiated to a path leading linkage given LinkageInstantiation
#[derive(Debug, PartialEq, Eq)]
pub struct ValkyrieRide {
    linkage_item_path: LinkageItemPath,
    hir_instantiation: HirInstantiation,
}

impl ValkyrieRide {
    fn to_linkage(
        &self,
        linkage_instantiation: &LinkageInstantiation,
        db: &::salsa::Db,
    ) -> Linkage {
        Linkage::new(
            db,
            LinkageData::PathLeading {
                path: self.linkage_item_path,
                instantiation: LinkageInstantiation::from_hir(
                    &self.hir_instantiation,
                    Some(linkage_instantiation),
                    db,
                ),
            },
        )
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct ValkyrieRides {
    hir_template_parameters: Option<HirTemplateParameters>,
    rides: VecSet<ValkyrieRide>,
}

#[salsa::tracked(jar = LinkageJar, return_ref)]
fn item_valkyrie_rides(db: &::salsa::Db, id: ItemPathId) -> Option<ValkyrieRides> {
    let item_path = id.item_path(db);
    let hir_decl = item_path.hir_decl(db)?;
    let hir_template_parameters = hir_decl.template_parameters(db).map(Clone::clone);
    let mut valkyrie_ride = ValkyrieRides {
        hir_template_parameters,
        rides: Default::default(),
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
            #[deprecated(note = "incomplete")]
            match *data {
                HirEagerExprData::AssociatedFn {
                    associated_item_path,
                } => (), // ad hoc
                HirEagerExprData::PrincipalEntityPath(path) => match path {
                    PrincipalEntityPath::Module(_) => unreachable!(),
                    PrincipalEntityPath::MajorItem(path) => match path {
                        MajorItemPath::Type(_) => (),
                        MajorItemPath::Trait(_) => (),
                        MajorItemPath::Fugitive(_) => (),
                    },
                    PrincipalEntityPath::TypeVariant(path) => (),
                },
                HirEagerExprData::Be { src, ref target } => (),
                HirEagerExprData::TypeConstructorFnCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation),
                HirEagerExprData::TypeVariantConstructorCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation),
                HirEagerExprData::FunctionFnCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation),
                HirEagerExprData::AssociatedFunctionFnCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation),
                HirEagerExprData::PropsStructField {
                    owner_hir_expr_idx,
                    ident,
                } => (),
                HirEagerExprData::MemoizedField {
                    owner_hir_expr_idx,
                    ident,
                    path,
                } =>
                /* ad hoc */
                {
                    ()
                }
                HirEagerExprData::MethodFnCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation),
                HirEagerExprData::NewTuple { ref items } => (),
                HirEagerExprData::Index {
                    owner_hir_expr_idx,
                    ref items,
                } => (),
                HirEagerExprData::NewList { ref items } => (),
                _ => (),
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
                HirLazyExprData::AssociatedFn { path } => (),
                HirLazyExprData::PrincipalEntityPath(_) => (),
                HirLazyExprData::Be { src, ref target } =>
                /* ad hoc */
                {
                    ()
                }
                HirLazyExprData::TypeConstructorFnCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation),
                HirLazyExprData::TypeVariantConstructorFnCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation),
                HirLazyExprData::FunctionFnItemCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation),
                HirLazyExprData::FunctionGnItemCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation),
                HirLazyExprData::AssociatedFunctionFnCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation),
                HirLazyExprData::PropsStructField { owner, ident } => (), // ad hoc
                HirLazyExprData::MemoizedField {
                    owner,
                    ident,
                    path,
                    ref indirections,
                    ref instantiation,
                } => (), // ad hoc
                HirLazyExprData::MethodFnCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_ride(path.into(), instantiation), // todo: handle indirections
                HirLazyExprData::NewTuple { ref items } => (),
                HirLazyExprData::Index { owner, ref items } => (),
                HirLazyExprData::NewList { ref items } => (),
                _ => (),
            }
        }
    }

    fn try_add_ride(&mut self, linkage_path: LinkageItemPath, instantiation: &HirInstantiation) {
        if !instantiation.is_empty() {
            self.rides.insert_move(ValkyrieRide {
                linkage_item_path: linkage_path,
                hir_instantiation: instantiation.clone(),
            })
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

#[salsa::tracked(jar = LinkageJar, return_ref)]
pub(crate) fn linkage_valkyrie_linkages(db: &::salsa::Db, linkage: Linkage) -> VecSet<Linkage> {
    match *linkage.data(db) {
        LinkageData::Coersion {} => Default::default(),
        LinkageData::PathLeading {
            path,
            ref instantiation,
        } => {
            let Some(valkyrie_rides) = item_valkyrie_rides(db, *path) else {
                return Default::default();
            };
            valkyrie_rides
                .rides
                .iter()
                .map(|ride| ride.to_linkage(instantiation, db))
                .collect()
        }
        LinkageData::PropsStructField => todo!(),
        LinkageData::MemoizedField => todo!(),
        LinkageData::Index => todo!(),
        LinkageData::Method => todo!(),
    }
}

pub struct PackageValkyrieLinkagesBuilder<'db> {
    db: &'db ::salsa::Db,
    linkages: VecSet<Linkage>,
}

impl<'db> PackageValkyrieLinkagesBuilder<'db> {}
