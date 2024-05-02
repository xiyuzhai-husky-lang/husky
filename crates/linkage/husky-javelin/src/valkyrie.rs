use crate::{
    amazon::package_amazon_javelins, instantiation::JavInstantiation, javelin::JavelinData,
    path::JavPath, template_argument::ty::JavelinType, *,
};
use fxhash::FxHashMap;
use husky_entity_path::{ItemPathId, MajorItemPath, PrincipalEntityPath};
use husky_hir_decl::{decl::HasHirDecl, parameter::template::HirTemplateParameters};
use husky_hir_defn::defn::HasHirDefn;
use husky_hir_eager_expr::{HirEagerExprData, HirEagerExprRegion};
use husky_hir_expr::HirExprRegion;
use husky_hir_lazy_expr::HirLazyStmtData;
use husky_hir_lazy_expr::{HirLazyExprData, HirLazyExprRegion};
use husky_hir_ty::{instantiation::HirInstantiation, HirType};
use husky_manifest::HasPackageManifest;
use husky_vfs::PackagePath;
use vec_like::VecSet;

/// a Valkyrie javelin is one with non empty instantiation
#[salsa::derive_debug_with_db]
#[salsa::deref_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValkyrieJavelin(Javelin);

impl ValkyrieJavelin {
    fn new(javelin: Javelin, db: &::salsa::Db) -> Self {
        #[cfg(test)]
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

/// can be instantiated to a path leading javelin given JavInstantiation
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum ValkyrieRide {
    PathLeading {
        path: JavPath,
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
        javelin_instantiation: &JavInstantiation,
        db: &::salsa::Db,
    ) -> ValkyrieJavelin {
        match *self {
            ValkyrieRide::PathLeading {
                path,
                ref hir_instantiation,
            } => {
                debug_assert!(!hir_instantiation.is_univalent_for_javelin());
                let instantiation =
                    JavInstantiation::from_hir(hir_instantiation, javelin_instantiation, db);
                ValkyrieJavelin(Javelin::new(
                    db,
                    JavelinData::PathLeading {
                        path,
                        instantiation,
                    },
                ))
            }
            ValkyrieRide::VecConstructor { element_ty } => ValkyrieJavelin(Javelin::new(
                db,
                JavelinData::VecConstructor {
                    element_ty: JavelinType::from_hir(element_ty, javelin_instantiation, db),
                },
            )),
            ValkyrieRide::TypeDefault { ty } => ValkyrieJavelin(Javelin::new(
                db,
                JavelinData::TypeDefault {
                    ty: JavelinType::from_hir(ty, javelin_instantiation, db),
                },
            )),
        }
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct ValkyrieRides {
    hir_template_parameters: Option<HirTemplateParameters>,
    rides: VecSet<ValkyrieRide>,
}

#[salsa::tracked(return_ref)]
fn item_valkyrie_rides(db: &::salsa::Db, id: ItemPathId) -> Option<ValkyrieRides> {
    let item_path = id.item_path(db);
    let hir_decl = item_path.hir_decl(db)?;
    let hir_template_parameters = hir_decl.template_parameters(db).map(Clone::clone);
    let mut builder = ValkyrieRidesBuilder {
        db,
        hir_template_parameters,
        rides: Default::default(),
    };
    builder.add_hir_expr_region(hir_decl.hir_expr_region(db)?, db);
    if let Some(hir_expr_region) = item_path.hir_defn(db)?.hir_expr_region(db) {
        builder.add_hir_expr_region(hir_expr_region, db);
    }
    Some(builder.finish())
}

struct ValkyrieRidesBuilder<'db> {
    db: &'db ::salsa::Db,
    hir_template_parameters: Option<HirTemplateParameters>,
    rides: VecSet<ValkyrieRide>,
}

impl<'db> ValkyrieRidesBuilder<'db> {
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
            match *entry.data() {
                HirEagerExprData::AssocRitchie { assoc_item_path: _ } => (), // ad hoc
                HirEagerExprData::PrincipalEntityPath(path) => match path {
                    PrincipalEntityPath::Module(_) => unreachable!(),
                    PrincipalEntityPath::MajorItem(path) => match path {
                        MajorItemPath::Type(_) => (),
                        MajorItemPath::Trait(_) => (),
                        MajorItemPath::Form(_) => (),
                    },
                    PrincipalEntityPath::TypeVariant(_path) => (),
                },
                HirEagerExprData::Be { src: _, pattern: _ } => (),
                HirEagerExprData::TypeConstructorCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_path_leading_ride(path.into(), instantiation),
                HirEagerExprData::TypeVariantConstructorCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_path_leading_ride(path.parent_ty_path(db).into(), instantiation),
                HirEagerExprData::FunctionRitchieCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_path_leading_ride(path.into(), instantiation),
                HirEagerExprData::AssocFunctionRitchieCall {
                    path,
                    ref instantiation,
                    ..
                } => {
                    if let Some(javelin_path) = JavPath::try_from_item_path(path.into(), db) {
                        self.try_add_path_leading_ride(javelin_path, instantiation)
                    }
                }
                HirEagerExprData::PropsStructField { .. } => (),
                HirEagerExprData::MemoizedField { .. } =>
                /* ad hoc */
                {
                    ()
                }
                HirEagerExprData::MethodRitchieCall {
                    path,
                    ref instantiation,
                    ..
                } => {
                    if let Some(javelin_path) = JavPath::try_from_item_path(path.into(), db) {
                        self.try_add_path_leading_ride(javelin_path, instantiation)
                    }
                }
                HirEagerExprData::NewTuple { items: _ } => (),
                HirEagerExprData::Index { owner: _, items: _ } => (),
                HirEagerExprData::NewList { element_ty, .. } => {
                    self.add_vec_constructor_ride(element_ty)
                }
                HirEagerExprData::Unveil {
                    unveil_assoc_fn_path,
                    ref instantiation,
                    ..
                } => {
                    if let Some(javelin_path) =
                        JavPath::try_from_item_path(unveil_assoc_fn_path.into(), db)
                    {
                        self.try_add_path_leading_ride(javelin_path, instantiation)
                    } else {
                        todo!()
                    }
                }
                HirEagerExprData::Literal(_)
                | HirEagerExprData::ConstVariable { .. }
                | HirEagerExprData::Variable(_)
                | HirEagerExprData::Binary { .. }
                | HirEagerExprData::Prefix { .. }
                | HirEagerExprData::Suffix { .. }
                | HirEagerExprData::As { .. }
                | HirEagerExprData::Block { .. }
                | HirEagerExprData::EmptyHtmlTag { .. }
                | HirEagerExprData::Todo
                | HirEagerExprData::Unreachable
                | HirEagerExprData::Unwrap { .. }
                | HirEagerExprData::Closure { .. } => (),
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
                HirLazyExprData::AssocRitchie { path: _ } => (),
                HirLazyExprData::PrincipalEntityPath(_) => (),
                HirLazyExprData::Be {
                    src: _,
                    pattern: ref _target,
                } =>
                /* ad hoc */
                {
                    ()
                }
                HirLazyExprData::TypeConstructorCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_path_leading_ride(path.into(), instantiation),
                HirLazyExprData::TypeVariantConstructorCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_path_leading_ride(path.parent_ty_path(db).into(), instantiation),
                HirLazyExprData::FunctionRitchieItemCall {
                    path,
                    ref instantiation,
                    ..
                } => self.try_add_path_leading_ride(path.into(), instantiation),
                HirLazyExprData::AssocFunctionRitchieCall {
                    path,
                    ref instantiation,
                    ..
                } => {
                    if let Some(javelin_path) = JavPath::try_from_item_path(path.into(), db) {
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
                    if let Some(javelin_path) = JavPath::try_from_item_path(path.into(), db) {
                        self.try_add_path_leading_ride(javelin_path, instantiation)
                    }
                }
                HirLazyExprData::MethodRitchieCall {
                    path,
                    ref instantiation,
                    ..
                } => {
                    if let Some(javelin_path) = JavPath::try_from_item_path(path.into(), db) {
                        self.try_add_path_leading_ride(javelin_path, instantiation)
                    }
                }
                HirLazyExprData::NewTuple { items: _ } => (),
                HirLazyExprData::Index { owner: _, items: _ } => (),
                HirLazyExprData::ConstructList { element_ty, .. } => {
                    self.add_vec_constructor_ride(element_ty)
                }
                HirLazyExprData::Unveil {
                    unveil_assoc_fn_path: _,
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
        jav_path: JavPath,
        hir_instantiation: &HirInstantiation,
    ) {
        if !hir_instantiation.is_univalent_for_javelin() {
            self.rides.insert_move(ValkyrieRide::PathLeading {
                path: jav_path,
                hir_instantiation: hir_instantiation.clone(),
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

    fn finish(self) -> ValkyrieRides {
        ValkyrieRides {
            hir_template_parameters: self.hir_template_parameters,
            rides: self.rides,
        }
    }
}

#[test]
fn item_valkyrie_rides_works() {
    use husky_entity_tree::helpers::paths::module_item_paths;

    DB::ast_expect_test_debug_with_db(
        |db, module_path| -> Vec<_> {
            module_item_paths(db, module_path)
                .iter()
                .map(|&item_path| (item_path, item_valkyrie_rides(db, *item_path)))
                .collect()
        },
        &AstTestConfig::new(
            "item_valkyrie_rides",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::LINKAGE,
        ),
    )
}

#[salsa::tracked(return_ref)]
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

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct ValkyrieJavelinPantheon {
    package_path: PackagePath,
    // map each javelin to a package where it's instantiated
    instantiation_map: FxHashMap<ValkyrieJavelin, PackagePath>,
    package_valkyrie_javelins: Vec<ValkyrieJavelin>,
}

#[salsa::tracked(return_ref)]
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
    DB::ast_expect_test_debug_with_db(
        |db, package_path| package_valkyrie_javelin_pantheon(db, package_path),
        &AstTestConfig::new(
            "package_javelin_pantheon",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::LINKAGE,
        ),
    )
}
