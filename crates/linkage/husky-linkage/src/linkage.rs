use crate::{template_argument::ty::LinkageType, *};
use either::*;
use husky_coword::Ident;
use husky_entity_kind::{FugitiveKind, TraitItemKind, TypeItemKind, TypeKind};
use husky_entity_path::{
    AssociatedItemPath, FugitivePath, PreludeTraitPath, TypeItemPath, TypeVariantPath,
};
use husky_entity_path::{TraitForTypeItemPath, TypePath};
use husky_hir_decl::parameter::template::item_hir_template_parameter_stats;
use husky_hir_defn::{FugitiveHirDefn, HasHirDefn, HirDefn, MajorItemHirDefn};
use husky_hir_expr::HirExprIdx;
use husky_hir_ty::{
    instantiation::HirInstantiation, HirTemplateArgument, HirTemplateArguments, HirType,
};
use husky_javelin::{
    javelin::{package_javelins, Javelin, JavelinData},
    path::JavelinPath,
};
use husky_print_utils::p;
use husky_vfs::PackagePath;
use salsa::DebugWithDb;
use smallvec::{smallvec, SmallVec};

#[salsa::interned(jar = LinkageJar, constructor = pub(crate) new)]
pub struct Linkage {
    #[return_ref]
    pub data: LinkageData,
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LinkageData {
    FunctionFnItem {
        path: FugitivePath,
        instantiation: LinkageInstantiation,
    },
    ValItem {
        path: FugitivePath,
        instantiation: LinkageInstantiation,
    },
    MemoizedField {
        path: AssociatedItemPath,
        instantiation: LinkageInstantiation,
    },
    MethodFn {
        path: AssociatedItemPath,
        instantiation: LinkageInstantiation,
    },
    AssociatedFunctionFn {
        path: AssociatedItemPath,
        instantiation: LinkageInstantiation,
    },
    UnveilAssociatedFunctionFn {
        path: TraitForTypeItemPath,
        instantiation: LinkageInstantiation,
    },
    TypeConstructor {
        path: TypePath,
        instantiation: LinkageInstantiation,
    },
    TypeVariantConstructor {
        path: TypeVariantPath,
        instantiation: LinkageInstantiation,
    },
    PropsStructField {
        self_ty: LinkageType,
        ident: Ident,
    },
    Index,
    FunctionGnItem {
        path: FugitivePath,
        instantiation: LinkageInstantiation,
    },
}

impl Linkage {
    /// gives a linkage if the item is eagerly defined or extern
    pub fn new_val_item(path: FugitivePath, db: &::salsa::Db) -> Option<Self> {
        let FugitiveHirDefn::Val(hir_defn) = path.hir_defn(db).unwrap() else {
            unreachable!()
        };
        match hir_defn.body_with_hir_expr_region(db) {
            Some((HirExprIdx::Lazy(_), _)) => None,
            Some((HirExprIdx::Eager(_), _)) | None => Some(Self::new(
                db,
                LinkageData::ValItem {
                    path,
                    // ad hoc
                    instantiation: LinkageInstantiation::new_empty(false),
                },
            )),
        }
    }

    // todo: linkage_instantiation
    // todo: change to `JavelinType`
    pub fn new_props_struct_field(
        self_ty: HirType,
        ident: Ident,
        linkage_instantiation: &LinkageInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        let self_ty = LinkageType::from_hir(self_ty, Some(linkage_instantiation), db);
        let data = LinkageData::PropsStructField { self_ty, ident };
        Self::new(db, data)
    }

    // todo: linkage_instantiation
    pub fn new_memoized_field(
        path: AssociatedItemPath,
        hir_instantiation: &HirInstantiation,
        linkage_instantiation: &LinkageInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            LinkageData::MemoizedField {
                path,
                instantiation: LinkageInstantiation::from_hir(
                    hir_instantiation,
                    linkage_instantiation,
                    db,
                ),
            },
        )
    }

    pub fn new_method(
        path: AssociatedItemPath,
        hir_instantiation: &HirInstantiation,
        linkage_instantiation: &LinkageInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            LinkageData::MethodFn {
                path,
                instantiation: LinkageInstantiation::from_hir(
                    hir_instantiation,
                    linkage_instantiation,
                    db,
                ),
            },
        )
    }

    pub fn new_index(db: &::salsa::Db) -> Self {
        Self::new(db, LinkageData::Index)
    }

    pub fn new_ty_constructor_fn(
        path: TypePath,
        hir_instantiation: &HirInstantiation,
        linkage_instantiation: &LinkageInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            LinkageData::TypeConstructor {
                path,
                instantiation: LinkageInstantiation::from_hir(
                    hir_instantiation,
                    linkage_instantiation,
                    db,
                ),
            },
        )
    }

    pub fn new_ty_variant_constructor_fn(
        path: TypeVariantPath,
        hir_instantiation: &HirInstantiation,
        linkage_instantiation: &LinkageInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            LinkageData::TypeVariantConstructor {
                path,
                instantiation: LinkageInstantiation::from_hir(
                    hir_instantiation,
                    linkage_instantiation,
                    db,
                ),
            },
        )
    }

    pub fn new_function_fn_item(
        path: FugitivePath,
        hir_instantiation: &HirInstantiation,
        linkage_instantiation: &LinkageInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        debug_assert_eq!(path.fugitive_kind(db), FugitiveKind::FunctionFn);
        Self::new(
            db,
            LinkageData::FunctionFnItem {
                path,
                instantiation: LinkageInstantiation::from_hir(
                    hir_instantiation,
                    linkage_instantiation,
                    db,
                ),
            },
        )
    }

    pub fn new_function_gn_item(
        path: FugitivePath,
        hir_instantiation: &HirInstantiation,
        linkage_instantiation: &LinkageInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        debug_assert_eq!(path.fugitive_kind(db), FugitiveKind::FunctionGn);
        Self::new(
            db,
            LinkageData::FunctionGnItem {
                path,
                instantiation: LinkageInstantiation::from_hir(
                    hir_instantiation,
                    linkage_instantiation,
                    db,
                ),
            },
        )
    }

    pub fn new_associated_function_fn_item(
        path: AssociatedItemPath,
        hir_instantiation: &HirInstantiation,
        linkage_instantiation: &LinkageInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            LinkageData::AssociatedFunctionFn {
                path,
                instantiation: LinkageInstantiation::from_hir(
                    hir_instantiation,
                    linkage_instantiation,
                    db,
                ),
            },
        )
    }

    pub fn new_unveil_associated_fn(
        path: TraitForTypeItemPath,
        hir_instantiation: &HirInstantiation,
        linkage_instantiation: &LinkageInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            LinkageData::UnveilAssociatedFunctionFn {
                path,
                instantiation: LinkageInstantiation::from_hir(
                    hir_instantiation,
                    linkage_instantiation,
                    db,
                ),
            },
        )
    }
}

#[deprecated(note = "ad hoc implementation")]
#[salsa::tracked(jar = LinkageJar, return_ref)]
fn linkages_emancipated_by_javelin(db: &::salsa::Db, javelin: Javelin) -> SmallVec<[Linkage; 4]> {
    match *javelin.data(db) {
        JavelinData::PathLeading {
            path,
            ref instantiation,
        } => match path {
            JavelinPath::Fugitive(path) => match path.fugitive_kind(db) {
                FugitiveKind::FunctionFn => LinkageInstantiation::from_javelin(instantiation, db)
                    .into_iter()
                    .map(|instantiation| {
                        Linkage::new(
                            db,
                            LinkageData::FunctionFnItem {
                                path,
                                instantiation,
                            },
                        )
                    })
                    .collect(),
                FugitiveKind::FunctionGn => {
                    let Some(FugitiveHirDefn::FunctionGn(hir_defn)) = path.hir_defn(db) else {
                        unreachable!()
                    };
                    match hir_defn.hir_lazy_expr_region(db) {
                        Some(_) => smallvec![],
                        None => LinkageInstantiation::from_javelin(instantiation, db)
                            .into_iter()
                            .map(|instantiation| {
                                Linkage::new(
                                    db,
                                    LinkageData::FunctionGnItem {
                                        path,
                                        instantiation,
                                    },
                                )
                            })
                            .collect(),
                    }
                }
                FugitiveKind::AliasType => smallvec![],
                FugitiveKind::Val => {
                    smallvec![Linkage::new(
                        db,
                        LinkageData::ValItem {
                            path,
                            instantiation: LinkageInstantiation::new_empty(false),
                        }
                    )]
                }
            },
            JavelinPath::TypeItem(path) => match path.item_kind(db) {
                TypeItemKind::MethodFn => LinkageInstantiation::from_javelin(instantiation, db)
                    .into_iter()
                    .map(|instantiation| {
                        Linkage::new(
                            db,
                            LinkageData::MethodFn {
                                path: path.into(),
                                instantiation,
                            },
                        )
                    })
                    .collect(),
                TypeItemKind::AssociatedFunctionFn => {
                    LinkageInstantiation::from_javelin(instantiation, db)
                        .into_iter()
                        .map(|instantiation| {
                            Linkage::new(
                                db,
                                LinkageData::AssociatedFunctionFn {
                                    path: path.into(),
                                    instantiation,
                                },
                            )
                        })
                        .collect()
                }
                TypeItemKind::AssociatedVal => todo!(),
                TypeItemKind::AssociatedType => smallvec![],
                TypeItemKind::MemoizedField => {
                    LinkageInstantiation::from_javelin(instantiation, db)
                        .into_iter()
                        .map(|instantiation| {
                            Linkage::new(
                                db,
                                LinkageData::MemoizedField {
                                    path: path.into(),
                                    instantiation,
                                },
                            )
                        })
                        .collect()
                }
            },
            JavelinPath::TraitItem(_) => todo!(),
            JavelinPath::TraitForTypeItem(path) => match path.item_kind(db) {
                TraitItemKind::MethodFn => LinkageInstantiation::from_javelin(instantiation, db)
                    .into_iter()
                    .map(|instantiation| {
                        Linkage::new(
                            db,
                            LinkageData::MethodFn {
                                path: path.into(),
                                instantiation,
                            },
                        )
                    })
                    .collect(),
                TraitItemKind::AssociatedType => smallvec![],
                TraitItemKind::AssociatedVal => todo!(),
                TraitItemKind::AssociatedFunctionFn => {
                    match path.impl_block(db).trai_path(db).refine(db) {
                        Left(PreludeTraitPath::UNVEIL) => {
                            LinkageInstantiation::from_javelin(instantiation, db)
                                .into_iter()
                                .map(|instantiation| {
                                    [
                                        Linkage::new(
                                            db,
                                            LinkageData::AssociatedFunctionFn {
                                                path: path.into(),
                                                instantiation: instantiation.clone(),
                                            },
                                        ),
                                        Linkage::new(
                                            db,
                                            LinkageData::UnveilAssociatedFunctionFn {
                                                path: path.into(),
                                                instantiation,
                                            },
                                        ),
                                    ]
                                })
                                .flatten()
                                .collect()
                        }
                        _ => LinkageInstantiation::from_javelin(instantiation, db)
                            .into_iter()
                            .map(|instantiation| {
                                Linkage::new(
                                    db,
                                    LinkageData::AssociatedFunctionFn {
                                        path: path.into(),
                                        instantiation,
                                    },
                                )
                            })
                            .collect(),
                    }
                }
            },
            JavelinPath::TypeConstructor(path) => match path.ty_kind(db) {
                TypeKind::Enum => smallvec![],
                TypeKind::Inductive => unreachable!(),
                TypeKind::Record => unreachable!(),
                TypeKind::Struct => LinkageInstantiation::from_javelin(instantiation, db)
                    .into_iter()
                    .map(|instantiation| {
                        Linkage::new(
                            db,
                            LinkageData::TypeConstructor {
                                path,
                                instantiation,
                            },
                        )
                    })
                    .collect(),
                TypeKind::Structure => unreachable!(),
                TypeKind::Extern => {
                    p!(path.debug(db));
                    unreachable!()
                }
            },
            JavelinPath::TypeVariantConstructor(_) => todo!(),
        },
    }
}

#[salsa::tracked(jar = LinkageJar, return_ref)]
pub fn package_linkages(db: &::salsa::Db, package_path: PackagePath) -> Vec<Linkage> {
    package_javelins(db, package_path)
        .map(|javelin| linkages_emancipated_by_javelin(db, javelin).iter().copied())
        .flatten()
        .collect()
}

#[test]
fn package_linkages_works() {
    DB::default().ast_expect_test_debug_with_db(
        package_linkages,
        &AstTestConfig::new("package_linkages")
            .with_vfs_test_domains_config(VfsTestDomainsConfig::ExcludeLibrary),
    )
}
