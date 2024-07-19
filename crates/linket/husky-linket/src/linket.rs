mod ty;
pub mod virtual_linket_impl;

use crate::{
    linket::ty::ty_linkets_emancipated_by_javelin,
    template_argument::ty::{LinType, LinTypePathLeading},
    *,
};
use either::*;
use husky_coword::Ident;
use husky_entity_kind::{MajorFormKind, TraitItemKind, TypeItemKind, TypeKind};
use husky_entity_path::path::{
    assoc_item::{trai_for_ty_item::TraitForTypeItemPath, AssocItemPath},
    major_item::{form::MajorFormPath, trai::PreludeTraitPath, ty::TypePath},
    ty_variant::TypeVariantPath,
};
use husky_hir_decl::{
    decl::{HasHirDecl, TypeHirDecl},
    helpers::enum_ty_has_only_unit_variants,
};
use husky_hir_defn::defn::{major_item::form::MajorFormHirDefn, HasHirDefn};
use husky_hir_expr::HirExprIdx;
use husky_hir_ty::{instantiation::HirInstantiation, HirType};
use husky_javelin::{
    instantiation::JavInstantiation,
    javelin::{package_javelins, Javelin, JavelinData},
    path::JavPath,
};
use husky_vfs::path::package_path::PackagePath;
use smallvec::{smallvec, SmallVec};

#[salsa::interned(constructor = pub(crate) new)]
pub struct Linket {
    #[return_ref]
    pub data: LinketData,
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LinketData {
    MajorFunctionRitchie {
        path: MajorFormPath,
        instantiation: LinInstantiation,
    },
    MajorStaticVar {
        path: MajorFormPath,
        instantiation: LinInstantiation,
    },
    MajorVal {
        path: MajorFormPath,
        instantiation: LinInstantiation,
    },
    MemoizedField {
        path: AssocItemPath,
        instantiation: LinInstantiation,
    },
    MethodRitchie {
        path: AssocItemPath,
        instantiation: LinInstantiation,
    },
    AssocRitchie {
        path: AssocItemPath,
        instantiation: LinInstantiation,
    },
    UnveilAssocRitchie {
        path: TraitForTypeItemPath,
        instantiation: LinInstantiation,
    },
    StructConstructor {
        path: TypePath,
        instantiation: LinInstantiation,
    },
    StructDestructor {
        self_ty: LinTypePathLeading,
    },
    EnumVariantConstructor {
        self_ty: LinTypePathLeading,
        path: TypeVariantPath,
        instantiation: LinInstantiation,
    },
    /// tells if a value is of a certain variant, returns bool
    EnumVariantDiscriminator {
        self_ty: LinTypePathLeading,
        path: TypeVariantPath,
        instantiation: LinInstantiation,
    },
    /// destruct a value with `qual` assuming it is of a certain variant,
    /// panic otherwise
    EnumVariantDestructor {
        self_ty: LinTypePathLeading,
        path: TypeVariantPath,
        instantiation: LinInstantiation,
    },
    StructField {
        self_ty: LinTypePathLeading,
        field: LinketField,
    },
    EnumVariantField {
        path: TypeVariantPath,
        instantiation: LinInstantiation,
        field: LinketField,
    },
    Index,
    VecConstructor {
        element_ty: LinType,
    },
    TypeDefault {
        ty: LinType,
    },
    EnumUnitToJsonValue {
        ty_path: TypePath,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LinketField {
    Tuple { index: usize },
    Props { ident: Ident },
}

impl Linket {
    /// gives a linket if the item is eagerly defined or extern
    pub fn new_val(path: MajorFormPath, db: &::salsa::Db) -> Option<Self> {
        let MajorFormHirDefn::Val(hir_defn) = path.hir_defn(db).unwrap() else {
            unreachable!()
        };
        match hir_defn.hir_expr_body_and_region(db) {
            Some((HirExprIdx::Lazy(_), _)) => None,
            Some((HirExprIdx::Eager(_), _)) | None => Some(Self::new(
                db,
                LinketData::MajorVal {
                    path,
                    // ad hoc
                    instantiation: LinInstantiation::new_empty(false),
                },
            )),
        }
    }

    pub fn new_static_var(path: MajorFormPath, db: &::salsa::Db) -> Self {
        Self::new(
            db,
            LinketData::MajorStaticVar {
                path,
                instantiation: LinInstantiation::new_empty(false),
            },
        )
    }

    // todo: lin_instantiation
    // todo: change to `JavType`
    pub fn new_props_struct_field(
        self_ty: HirType,
        ident: Ident,
        instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        let LinType::PathLeading(self_ty) = LinType::from_hir(self_ty, instantiation, db) else {
            unreachable!()
        };
        let data = LinketData::StructField {
            self_ty,
            field: LinketField::Props { ident },
        };
        Self::new(db, data)
    }

    // todo: lin_instantiation
    pub fn new_memo_field(
        path: AssocItemPath,
        hir_instantiation: &HirInstantiation,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            LinketData::MemoizedField {
                path,
                instantiation: LinInstantiation::from_hir(hir_instantiation, lin_instantiation, db),
            },
        )
    }

    pub fn new_method(
        path: AssocItemPath,
        hir_instantiation: &HirInstantiation,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            LinketData::MethodRitchie {
                path,
                instantiation: LinInstantiation::from_hir(hir_instantiation, lin_instantiation, db),
            },
        )
    }

    pub fn new_index(db: &::salsa::Db) -> Self {
        Self::new(db, LinketData::Index)
    }

    pub fn new_ty_constructor_fn(
        path: TypePath,
        hir_instantiation: &HirInstantiation,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            LinketData::StructConstructor {
                path,
                instantiation: LinInstantiation::from_hir(hir_instantiation, lin_instantiation, db),
            },
        )
    }

    pub fn new_ty_variant_constructor_fn(
        path: TypeVariantPath,
        hir_instantiation: &HirInstantiation,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        let instantiation = LinInstantiation::from_hir(hir_instantiation, lin_instantiation, db);
        let self_ty = LinTypePathLeading::from_path_instantiation(
            path.parent_ty_path(db),
            &instantiation,
            db,
        );
        Self::new(
            db,
            LinketData::EnumVariantConstructor {
                path,
                instantiation,
                self_ty,
            },
        )
    }

    pub fn new_vec_constructor(
        element_ty: HirType,
        instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            LinketData::VecConstructor {
                element_ty: LinType::from_hir(element_ty, instantiation, db),
            },
        )
    }

    pub fn new_major_function_ritchie_item(
        path: MajorFormPath,
        hir_instantiation: &HirInstantiation,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            LinketData::MajorFunctionRitchie {
                path,
                instantiation: LinInstantiation::from_hir(hir_instantiation, lin_instantiation, db),
            },
        )
    }

    pub fn new_assoc_function_ritchie_item(
        path: AssocItemPath,
        hir_instantiation: &HirInstantiation,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            LinketData::AssocRitchie {
                path,
                instantiation: LinInstantiation::from_hir(hir_instantiation, lin_instantiation, db),
            },
        )
    }

    pub fn new_unveil_assoc_fn(
        path: TraitForTypeItemPath,
        hir_instantiation: &HirInstantiation,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            LinketData::UnveilAssocRitchie {
                path,
                instantiation: LinInstantiation::from_hir(hir_instantiation, lin_instantiation, db),
            },
        )
    }

    pub fn new_ty_default(ty: HirType, instantiation: &LinInstantiation, db: &::salsa::Db) -> Self {
        Self::new(
            db,
            LinketData::TypeDefault {
                ty: LinType::from_hir(ty, instantiation, db),
            },
        )
    }

    pub fn new_enum_index_presenter(ty_path: TypePath, db: &::salsa::Db) -> Self {
        Self::new(db, LinketData::EnumUnitToJsonValue { ty_path })
    }
}

#[salsa::tracked(return_ref)]
fn linkets_emancipated_by_javelin(db: &::salsa::Db, javelin: Javelin) -> SmallVec<[Linket; 4]> {
    match *javelin.data(db) {
        JavelinData::PathLeading {
            path,
            ref instantiation,
        } => {
            fn build(
                instantiation: &JavInstantiation,
                f: impl Fn(LinInstantiation) -> Linket,
                db: &::salsa::Db,
            ) -> SmallVec<[Linket; 4]> {
                LinInstantiation::from_jav(instantiation, db)
                    .into_iter()
                    .map(f)
                    .collect()
            }
            match path {
                JavPath::Form(path) => match path.kind(db) {
                    MajorFormKind::Ritchie(ritchie_item_kind) => {
                        match ritchie_item_kind.requires_lazy_to_use() {
                            true => {
                                let Some(hir_defn) = path.hir_defn(db) else {
                                    unreachable!()
                                };
                                match hir_defn.hir_expr_region(db) {
                                    Some(_) => smallvec![],
                                    None => build(
                                        instantiation,
                                        |instantiation| {
                                            Linket::new(
                                                db,
                                                LinketData::MajorFunctionRitchie {
                                                    path,
                                                    instantiation,
                                                },
                                            )
                                        },
                                        db,
                                    ),
                                }
                            }
                            false => build(
                                instantiation,
                                |instantiation| {
                                    Linket::new(
                                        db,
                                        LinketData::MajorFunctionRitchie {
                                            path,
                                            instantiation,
                                        },
                                    )
                                },
                                db,
                            ),
                        }
                    }
                    MajorFormKind::Val => {
                        smallvec![Linket::new(
                            db,
                            LinketData::MajorVal {
                                path,
                                instantiation: LinInstantiation::new_empty(false),
                            }
                        )]
                    }
                    MajorFormKind::Compterm => todo!(),
                    // ad hoc
                    MajorFormKind::StaticMut => {
                        smallvec![Linket::new(
                            db,
                            LinketData::MajorStaticVar {
                                path,
                                instantiation: LinInstantiation::new_empty(false),
                            }
                        )]
                    }
                    // ad hoc
                    MajorFormKind::StaticVar => {
                        smallvec![Linket::new(
                            db,
                            LinketData::MajorStaticVar {
                                path,
                                instantiation: LinInstantiation::new_empty(false),
                            }
                        )]
                    }
                    MajorFormKind::TypeAlias
                    | MajorFormKind::TypeVar
                    | MajorFormKind::Conceptual => unreachable!(),
                },
                JavPath::TypeItem(path) => match path.item_kind(db) {
                    TypeItemKind::AssocRitchie(_) => build(
                        instantiation,
                        |instantiation| {
                            Linket::new(
                                db,
                                LinketData::AssocRitchie {
                                    path: path.into(),
                                    instantiation,
                                },
                            )
                        },
                        db,
                    ),
                    TypeItemKind::AssocVal => todo!(),
                    TypeItemKind::AssocType => smallvec![],
                    TypeItemKind::AssocConceptual => todo!(),
                    TypeItemKind::AssocStaticMut => todo!(),
                    TypeItemKind::AssocStaticVar => todo!(),
                    TypeItemKind::AssocCompterm => todo!(),
                    TypeItemKind::MemoizedField => build(
                        instantiation,
                        |instantiation| {
                            Linket::new(
                                db,
                                LinketData::MemoizedField {
                                    path: path.into(),
                                    instantiation,
                                },
                            )
                        },
                        db,
                    ),
                    TypeItemKind::MethodRitchie(_) => build(
                        instantiation,
                        |instantiation| {
                            Linket::new(
                                db,
                                LinketData::MethodRitchie {
                                    path: path.into(),
                                    instantiation,
                                },
                            )
                        },
                        db,
                    ),
                },
                JavPath::TraitItem(_) => todo!(),
                JavPath::TraitForTypeItem(path) => match path.item_kind(db) {
                    TraitItemKind::MemoizedField => todo!(),
                    TraitItemKind::MethodRitchie(ritchie_item_kind) => {
                        match ritchie_item_kind.requires_lazy_to_use() {
                            true => {
                                todo!()
                            }
                            false => build(
                                instantiation,
                                |instantiation| {
                                    Linket::new(
                                        db,
                                        LinketData::MethodRitchie {
                                            path: path.into(),
                                            instantiation,
                                        },
                                    )
                                },
                                db,
                            ),
                        }
                    }
                    TraitItemKind::AssocType => smallvec![],
                    TraitItemKind::AssocVal => todo!(),
                    TraitItemKind::AssocRitchie(ritchie_item_kind) => {
                        match path.impl_block(db).trai_path(db).refine(db) {
                            Left(PreludeTraitPath::UNVEIL) => {
                                LinInstantiation::from_jav(instantiation, db)
                                    .into_iter()
                                    .flat_map(|instantiation| {
                                        [
                                            Linket::new(
                                                db,
                                                LinketData::AssocRitchie {
                                                    path: path.into(),
                                                    instantiation: instantiation.clone(),
                                                },
                                            ),
                                            Linket::new(
                                                db,
                                                LinketData::UnveilAssocRitchie {
                                                    path: path.into(),
                                                    instantiation,
                                                },
                                            ),
                                        ]
                                    })
                                    .collect()
                            }
                            _ => match ritchie_item_kind.requires_lazy_to_use() {
                                true => todo!(),
                                false => build(
                                    instantiation,
                                    |instantiation| {
                                        Linket::new(
                                            db,
                                            LinketData::AssocRitchie {
                                                path: path.into(),
                                                instantiation,
                                            },
                                        )
                                    },
                                    db,
                                ),
                            },
                        }
                    }
                    TraitItemKind::AssocConceptual => todo!(),
                    TraitItemKind::AssocStaticMut => todo!(),
                    TraitItemKind::AssocStaticVar => todo!(),
                    TraitItemKind::AssocCompterm => todo!(),
                },
                JavPath::Type(path) => ty_linkets_emancipated_by_javelin(path, instantiation, db),
            }
        }
        JavelinData::VecConstructor { element_ty } => smallvec![Linket::new(
            db,
            LinketData::VecConstructor {
                element_ty: LinType::from_jav(
                    element_ty,
                    // ad hoc
                    &LinInstantiation::new_empty(false),
                    db
                )
            },
        )],
        JavelinData::TypeDefault { ty } => smallvec![Linket::new(
            db,
            LinketData::TypeDefault {
                ty: LinType::from_jav(
                    ty,
                    // ad hoc
                    &LinInstantiation::new_empty(false),
                    db
                )
            },
        )],
    }
}

#[salsa::tracked(return_ref)]
pub fn package_linkets(db: &::salsa::Db, package_path: PackagePath) -> Vec<Linket> {
    package_javelins(db, package_path)
        .flat_map(|javelin| linkets_emancipated_by_javelin(db, javelin).iter().copied())
        .collect()
}

#[test]
fn package_linkets_works() {
    DB::ast_rich_test_debug_with_db(
        package_linkets,
        &AstTestConfig::new(
            "package_linkets",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::LINKET,
        ),
    )
}
