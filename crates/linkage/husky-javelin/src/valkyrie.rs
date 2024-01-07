use crate::{
    amazon::{package_amazon_javelins},
    instantiation::JavelinInstantiation,
    javelin::JavelinData,
    path::JavelinPath,
    template_argument::ty::JavelinType,
    *,
};
use fxhash::FxHashMap;
use husky_entity_path::{ItemPathId, MajorItemPath, PrincipalEntityPath};
use husky_entity_syn_tree::helpers::paths::module_item_paths;
use husky_hir_decl::{parameter::template::HirTemplateParameters, HasHirDecl};
use husky_hir_defn::HasHirDefn;
use husky_hir_eager_expr::{HirEagerExprData, HirEagerExprRegion};
use husky_hir_expr::HirExprRegion;
use husky_hir_lazy_expr::HirLazyStmtData;
use husky_hir_lazy_expr::{HirLazyExprData, HirLazyExprRegion};
use husky_hir_ty::{instantiation::HirInstantiation, HirType};
use husky_manifest::HasPackageManifest;
use husky_vfs::PackagePath;

use vec_like::VecSet;

/// a Valkyrie javelin is one with non empty instantiation
#[salsa::debug_with_db]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValkyrieJavelin(Javelin);

impl ValkyrieJavelin {
    fn new(javelin: Javelin, db: &::salsa::Db) -> Self {
        #[cfg(debug_assertions)]
        match javelin.data(db) {
            JavelinData::PathLeading { instantiation, .. } => {
                debug_assert!(!instantiation.is_univalent())
            }
            JavelinData::VecConstructor { element_ty: _ } => todo!(),
            JavelinData::TypeDefault { ty: _ } => todo!(),
        }
        Self(javelin)
    }
}

/// can be instantiated to a path leading javelin given JavelinInstantiation
#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum ValkyrieRide {
    PathLeading {
        javelin_item_path: JavelinPath,
        hir_instantiation: HirInstantiation,
    },
    VecConstructor {
        element_ty: HirType,
    },
    TypeDefault {
        ty: HirType,
    },
}

impl ValkyrieRide {
    fn to_javelin(
        &self,
        javelin_instantiation: &JavelinInstantiation,
        db: &::salsa::Db,
    ) -> ValkyrieJavelin {
        match *self {
            ValkyrieRide::PathLeading {
                javelin_item_path,
                ref hir_instantiation,
            } => {
                debug_assert!(!hir_instantiation.is_univalent_for_javelin());
                ValkyrieJavelin(Javelin::new(
                    db,
                    JavelinData::PathLeading {
                        path: javelin_item_path,
                        instantiation: JavelinInstantiation::from_hir(
                            hir_instantiation,
                            Some(javelin_instantiation),
                            db,
                        ),
                    },
                ))
            }
            ValkyrieRide::VecConstructor { element_ty } => ValkyrieJavelin(Javelin::new(
                db,
                JavelinData::VecConstructor {
                    element_ty: JavelinType::from_hir(element_ty, Some(javelin_instantiation), db),
                },
            )),
            ValkyrieRide::TypeDefault { ty } => ValkyrieJavelin(Javelin::new(
                db,
                JavelinData::TypeDefault {
                    ty: JavelinType::from_hir(ty, Some(javelin_instantiation), db),
                },
            )),
        }
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct ValkyrieRides {
    hir_template_parameters: Option<HirTemplateParameters>,
    rides: VecSet<ValkyrieRide>,
}

#[salsa::tracked(jar = JavelinJar, return_ref)]
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
        for entry in hir_eager_expr_region.expr_arena(db) {
            #[deprecated(note = "incomplete")]
            match entry.data {
                HirEagerExprData::AssociatedFn {
                    associated_item_path: _,
                } => (), // ad hoc
                HirEagerExprData::PrincipalEntityPath(path) => match path {
                    PrincipalEntityPath::Module(_) => unreachable!(),
                    PrincipalEntityPath::MajorItem(path) => match path {
                        MajorItemPath::Type(_) => (),
                        MajorItemPath::Trait(_) => (),
                        MajorItemPath::Fugitive(_) => (),
                    },
                    PrincipalEntityPath::TypeVariant(_path) => (),
                },
                HirEagerExprData::Be { src: _, target: _ } => (),
                HirEagerExprData::TypeConstructorFnCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_path_leading_ride(path.into(), instantiation),
                HirEagerExprData::TypeVariantConstructorCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_path_leading_ride(path.into(), instantiation),
                HirEagerExprData::FunctionFnCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_path_leading_ride(path.into(), instantiation),
                HirEagerExprData::AssociatedFunctionFnCall {
                    path,
                    ref instantiation,
                    ..
                } => {
                    if let Some(javelin_path) = JavelinPath::try_from_item_path(path.into(), db) {
                        self.try_add_path_leading_ride(javelin_path, instantiation)
                    }
                }
                HirEagerExprData::PropsStructField {
                    owner_hir_expr_idx: _,
                    ident: _,
                    field_ty: _,
                } => (),
                HirEagerExprData::MemoizedField {
                    owner_hir_expr_idx: _,
                    ident: _,
                    path: _,
                } =>
                /* ad hoc */
                {
                    ()
                }
                HirEagerExprData::MethodFnCall {
                    path,
                    ref instantiation,
                    ..
                } => {
                    if let Some(javelin_path) = JavelinPath::try_from_item_path(path.into(), db) {
                        self.try_add_path_leading_ride(javelin_path, instantiation)
                    }
                }
                HirEagerExprData::NewTuple { items: _ } => (),
                HirEagerExprData::Index {
                    owner_hir_expr_idx: _,
                    items: _,
                } => (),
                HirEagerExprData::NewList { element_ty, .. } => {
                    self.add_vec_constructor_ride(element_ty)
                }
                HirEagerExprData::Unveil {
                    unveil_associated_fn_path,
                    ref instantiation,
                    
                    ..
                } => {
                    if let Some(javelin_path) =
                        JavelinPath::try_from_item_path(unveil_associated_fn_path.into(), db)
                    {
                        self.try_add_path_leading_ride(javelin_path, instantiation)
                    } else {
                        todo!()
                    }
                }
                HirEagerExprData::Literal(_)
                | HirEagerExprData::ConstSymbol { .. }
                | HirEagerExprData::Variable(_)
                | HirEagerExprData::Binary { .. }
                | HirEagerExprData::Prefix { .. }
                | HirEagerExprData::Suffix { .. }
                | HirEagerExprData::As { .. }
                | HirEagerExprData::Block { .. }
                | HirEagerExprData::EmptyHtmlTag { .. }
                | HirEagerExprData::Todo
                | HirEagerExprData::Unreachable
                | HirEagerExprData::Unwrap { .. } => (),
            }
        }
        for _data in hir_eager_expr_region.stmt_arena(db) {
            // todo!()
            // match *data {
            //     HirEagerStmtData::Require { return_ty, .. } => todo!(),
            //     _ => todo!(),
            // }
        }
    }

    fn add_hir_lazy_expr_region(
        &mut self,
        hir_lazy_expr_region: HirLazyExprRegion,
        db: &::salsa::Db,
    ) {
        for data in hir_lazy_expr_region.hir_lazy_expr_arena(db) {
            match *data {
                HirLazyExprData::AssociatedFn { path: _ } => (),
                HirLazyExprData::PrincipalEntityPath(_) => (),
                HirLazyExprData::Be {
                    src: _,
                    pattern: ref _target,
                } =>
                /* ad hoc */
                {
                    ()
                }
                HirLazyExprData::TypeConstructorFnCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_path_leading_ride(path.into(), instantiation),
                HirLazyExprData::TypeVariantConstructorFnCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_path_leading_ride(path.into(), instantiation),
                HirLazyExprData::FunctionFnItemCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_path_leading_ride(path.into(), instantiation),
                HirLazyExprData::FunctionGnItemCall {
                    path,
                    ref instantiation,
                    ..
                } => {
                    self.try_add_path_leading_ride(path.into(), instantiation);
                }
                HirLazyExprData::AssociatedFunctionFnCall {
                    path,
                    ref instantiation,
                    ..
                } => {
                    if let Some(javelin_path) = JavelinPath::try_from_item_path(path.into(), db) {
                        self.try_add_path_leading_ride(javelin_path, instantiation)
                    }
                }
                HirLazyExprData::PropsStructField { .. } => (),
                HirLazyExprData::MemoizedField {
                    owner: _,
                    ident: _,
                    path,
                    indirections: _,
                    ref instantiation,
                } => {
                    if let Some(javelin_path) = JavelinPath::try_from_item_path(path.into(), db) {
                        self.try_add_path_leading_ride(javelin_path, instantiation)
                    }
                }
                HirLazyExprData::MethodFnCall {
                    path,
                    ref instantiation,
                    ..
                } => {
                    if let Some(javelin_path) = JavelinPath::try_from_item_path(path.into(), db) {
                        self.try_add_path_leading_ride(javelin_path, instantiation)
                    }
                }
                HirLazyExprData::NewTuple { items: _ } => (),
                HirLazyExprData::Index { owner: _, items: _ } => (),
                HirLazyExprData::ConstructList { element_ty, .. } => {
                    self.add_vec_constructor_ride(element_ty)
                }
                HirLazyExprData::Unveil {
                    unveil_associated_fn_path: _,
                    instantiation: _,
                    opd_hir_expr_idx: _,
                } => (),
                HirLazyExprData::Literal(_)
                | HirLazyExprData::ConstSymbol(_)
                | HirLazyExprData::Variable(_)
                | HirLazyExprData::Binary { .. }
                | HirLazyExprData::Prefix { .. }
                | HirLazyExprData::Suffix { .. }
                | HirLazyExprData::As { .. }
                | HirLazyExprData::Block { .. }
                | HirLazyExprData::EmptyHtmlTag { .. }
                | HirLazyExprData::Todo
                | HirLazyExprData::Unreachable
                | HirLazyExprData::Unwrap { .. } => (),
            }
        }
        for data in hir_lazy_expr_region.hir_lazy_stmt_arena(db) {
            match *data {
                HirLazyStmtData::Require { return_ty, .. } => self.add_ty_default_ride(return_ty),
                _ => (),
            }
        }
    }

    fn try_add_path_leading_ride(
        &mut self,
        javelin_path: JavelinPath,
        instantiation: &HirInstantiation,
    ) {
        if !instantiation.is_univalent_for_javelin() {
            self.rides.insert_move(ValkyrieRide::PathLeading {
                javelin_item_path: javelin_path,
                hir_instantiation: instantiation.clone(),
            })
        }
    }

    fn add_vec_constructor_ride(&mut self, element_ty: HirType) {
        self.rides
            .insert_move(ValkyrieRide::VecConstructor { element_ty })
    }

    fn add_ty_default_ride(&mut self, ty: HirType) {
        self.rides.insert_move(ValkyrieRide::TypeDefault { ty })
    }
}

#[test]
fn item_valkyrie_rides_works() {
    DB::default().ast_expect_test_debug_with_db(
        |db, module_path| -> Vec<_> {
            module_item_paths(db, module_path)
                .iter()
                .map(|&item_path| (item_path, item_valkyrie_rides(db, *item_path)))
                .collect()
        },
        &AstTestConfig::new("item_valkyrie_rides"),
    )
}

#[salsa::tracked(jar = JavelinJar, return_ref)]
pub(crate) fn javelin_generated_valkyrie_javelins(
    db: &::salsa::Db,
    javelin: Javelin,
) -> VecSet<ValkyrieJavelin> {
    match *javelin.data(db) {
        JavelinData::PathLeading {
            path,
            ref instantiation,
        } => {
            let Some(valkyrie_rides) = item_valkyrie_rides(db, *path) else {
                return Default::default();
            };
            valkyrie_rides
                .rides
                .iter()
                .map(|ride| ride.to_javelin(instantiation, db))
                .collect()
        }
        JavelinData::VecConstructor { element_ty: _ } => Default::default(),
        JavelinData::TypeDefault { ty: _ } => Default::default(),
    }
}

pub struct PackageValkyrieJavelinsBuilder<'db> {
    db: &'db ::salsa::Db,
    javelins: VecSet<Javelin>,
}

impl<'db> PackageValkyrieJavelinsBuilder<'db> {}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct ValkyrieJavelinPantheon {
    package_path: PackagePath,
    // map each javelin to a package where it's instantiated
    instantiation_map: FxHashMap<ValkyrieJavelin, PackagePath>,
    package_valkyrie_javelins: Vec<ValkyrieJavelin>,
}

#[salsa::tracked(jar = JavelinJar, return_ref)]
fn package_valkyrie_javelin_pantheon(
    db: &::salsa::Db,
    package_path: PackagePath,
) -> ValkyrieJavelinPantheon {
    let mut pantheon = ValkyrieJavelinPantheon {
        package_path,
        instantiation_map: Default::default(),
        package_valkyrie_javelins: Default::default(),
    };
    for dep in package_path
        .dependencies(db)
        .expect("no error at this stage")
    {
        pantheon.add_valkyrie_javelins_instantiated_by_package(package_valkyrie_javelin_pantheon(
            db,
            dep.package_path(),
        ))
    }
    let mut bar = 0;
    for &javelin in package_amazon_javelins(db, package_path) {
        pantheon.try_add_valkyrie_javelins_instantiated_by_javelin(*javelin, db)
    }
    while bar < pantheon.package_valkyrie_javelins.len() {
        let new_bar = pantheon.package_valkyrie_javelins.len();
        for i in bar..pantheon.package_valkyrie_javelins.len() {
            pantheon.try_add_valkyrie_javelins_instantiated_by_javelin(
                *pantheon.package_valkyrie_javelins[i],
                db,
            )
        }
        bar = new_bar
    }
    pantheon
}

pub(crate) fn package_valkyrie_javelins<'db>(
    db: &'db ::salsa::Db,
    package_path: PackagePath,
) -> &'db [ValkyrieJavelin] {
    &package_valkyrie_javelin_pantheon(db, package_path).package_valkyrie_javelins
}

impl ValkyrieJavelinPantheon {
    pub fn new_valkyrie_javelins<'a>(&'a self) -> impl Iterator<Item = ValkyrieJavelin> + 'a {
        self.instantiation_map
            .iter()
            .filter_map(|(&javelin, &package_path)| {
                (package_path == self.package_path).then_some(javelin)
            })
    }

    fn add_valkyrie_javelins_instantiated_by_package(
        &mut self,
        other_package_javelin_pantheon: &Self,
    ) {
        for (&javelin, &package_path) in other_package_javelin_pantheon.instantiation_map.iter() {
            self.instantiation_map
                .entry(javelin)
                .or_insert(package_path);
        }
    }

    // do nothing if already instantiated
    fn try_add_valkyrie_javelins_instantiated_by_javelin(
        &mut self,
        javelin: Javelin,
        db: &::salsa::Db,
    ) {
        for &valkyrie_javelin in javelin_generated_valkyrie_javelins(db, javelin) {
            match self.instantiation_map.get(&valkyrie_javelin) {
                Some(_) => (),
                None => self.add_new_valkyrie_javelin(valkyrie_javelin),
            }
        }
    }

    fn add_new_valkyrie_javelin(&mut self, valkyrie_javelin: ValkyrieJavelin) {
        self.instantiation_map
            .insert(valkyrie_javelin, self.package_path);
        self.package_valkyrie_javelins.push(valkyrie_javelin)
    }
}

#[test]
fn package_javelin_pantheon_works() {
    let mut db = DB::default();
    db.ast_expect_test_debug_with_db(
        |db, package_path| package_valkyrie_javelin_pantheon(db, package_path),
        &AstTestConfig::new("package_javelin_pantheon"),
    )
}
