use crate::{
    template_argument::ty::{LinType, LinTypePathLeading},
    *,
};
use either::*;
use husky_coword::Ident;
use husky_entity_kind::{MajorFugitiveKind, TraitItemKind, TypeItemKind, TypeKind};
use husky_entity_path::{AssocItemPath, FugitivePath, PreludeTraitPath, TypeVariantPath};
use husky_entity_path::{TraitForTypeItemPath, TypePath};
use husky_hir_decl::decl::{HasHirDecl, TypeHirDecl};
use husky_hir_decl::helpers::enum_ty_has_only_unit_variants;
use husky_hir_defn::{FugitiveHirDefn, HasHirDefn};
use husky_hir_expr::HirExprIdx;
use husky_hir_ty::{instantiation::HirInstantiation, HirType};
use husky_javelin::{
    instantiation::JavInstantiation,
    javelin::{package_javelins, Javelin, JavelinData},
    path::JavPath,
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
    MajorRitchieEager {
        path: FugitivePath,
        instantiation: LinInstantiation,
    },
    MajorRitchieLazy {
        path: FugitivePath,
        instantiation: LinInstantiation,
    },
    MajorVal {
        path: FugitivePath,
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
    UnveilAssocFn {
        path: TraitForTypeItemPath,
        instantiation: LinInstantiation,
    },
    TypeConstructor {
        path: TypePath,
        instantiation: LinInstantiation,
    },
    TypeVariantConstructor {
        path: TypeVariantPath,
        instantiation: LinInstantiation,
    },
    StructField {
        self_ty: LinTypePathLeading,
        field: LinkageStructField,
    },
    Index,
    VecConstructor {
        element_ty: LinType,
    },
    TypeDefault {
        ty: LinType,
    },
    EnumU8ToJsonValue {
        ty_path: TypePath,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LinkageStructField {
    Tuple,
    Props { ident: Ident },
}

impl Linkage {
    /// gives a linkage if the item is eagerly defined or extern
    pub fn new_val_item(path: FugitivePath, db: &::salsa::Db) -> Option<Self> {
        let FugitiveHirDefn::Ki(hir_defn) = path.hir_defn(db).unwrap() else {
            unreachable!()
        };
        match hir_defn.body_with_hir_expr_region(db) {
            Some((HirExprIdx::Lazy(_), _)) => None,
            Some((HirExprIdx::Eager(_), _)) | None => Some(Self::new(
                db,
                LinkageData::MajorVal {
                    path,
                    // ad hoc
                    instantiation: LinInstantiation::new_empty(false),
                },
            )),
        }
    }

    // todo: lin_instantiation
    // todo: change to `JavelinType`
    pub fn new_props_struct_field(
        self_ty: HirType,
        ident: Ident,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        let LinType::PathLeading(self_ty) = LinType::from_hir(self_ty, Some(lin_instantiation), db)
        else {
            unreachable!()
        };
        let data = LinkageData::StructField {
            self_ty,
            field: LinkageStructField::Props { ident },
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
            LinkageData::MemoizedField {
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
            LinkageData::MethodRitchie {
                path,
                instantiation: LinInstantiation::from_hir(hir_instantiation, lin_instantiation, db),
            },
        )
    }

    pub fn new_index(db: &::salsa::Db) -> Self {
        Self::new(db, LinkageData::Index)
    }

    pub fn new_ty_constructor_fn(
        path: TypePath,
        hir_instantiation: &HirInstantiation,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            LinkageData::TypeConstructor {
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
        Self::new(
            db,
            LinkageData::TypeVariantConstructor {
                path,
                instantiation: LinInstantiation::from_hir(hir_instantiation, lin_instantiation, db),
            },
        )
    }

    pub fn new_vec_constructor(
        element_ty: HirType,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            LinkageData::VecConstructor {
                element_ty: LinType::from_hir(element_ty, Some(lin_instantiation), db),
            },
        )
    }

    pub fn new_function_fn_item(
        path: FugitivePath,
        hir_instantiation: &HirInstantiation,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        debug_assert_eq!(path.major_fugitive_kind(db), MajorFugitiveKind::FN);
        Self::new(
            db,
            LinkageData::MajorRitchieEager {
                path,
                instantiation: LinInstantiation::from_hir(hir_instantiation, lin_instantiation, db),
            },
        )
    }

    pub fn new_function_gn_item(
        path: FugitivePath,
        hir_instantiation: &HirInstantiation,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        debug_assert_eq!(path.major_fugitive_kind(db), MajorFugitiveKind::GN);
        Self::new(
            db,
            LinkageData::MajorRitchieLazy {
                path,
                instantiation: LinInstantiation::from_hir(hir_instantiation, lin_instantiation, db),
            },
        )
    }

    pub fn new_assoc_function_fn_item(
        path: AssocItemPath,
        hir_instantiation: &HirInstantiation,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            LinkageData::AssocRitchie {
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
            LinkageData::UnveilAssocFn {
                path,
                instantiation: LinInstantiation::from_hir(hir_instantiation, lin_instantiation, db),
            },
        )
    }

    pub fn new_ty_default(
        ty: HirType,
        lin_instantiation: &LinInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        Self::new(
            db,
            LinkageData::TypeDefault {
                ty: LinType::from_hir(ty, Some(lin_instantiation), db),
            },
        )
    }

    pub fn new_enum_u8_presenter(ty_path: TypePath, db: &::salsa::Db) -> Self {
        Self::new(db, LinkageData::EnumU8ToJsonValue { ty_path })
    }
}

#[salsa::tracked(jar = LinkageJar, return_ref)]
fn linkages_emancipated_by_javelin(db: &::salsa::Db, javelin: Javelin) -> SmallVec<[Linkage; 4]> {
    match *javelin.data(db) {
        JavelinData::PathLeading {
            path,
            ref instantiation,
        } => {
            fn build(
                instantiation: &JavInstantiation,
                f: impl Fn(LinInstantiation) -> Linkage,
                db: &::salsa::Db,
            ) -> SmallVec<[Linkage; 4]> {
                LinInstantiation::from_javelin(instantiation, db)
                    .into_iter()
                    .map(f)
                    .collect()
            }
            match path {
                JavPath::Fugitive(path) => match path.major_fugitive_kind(db) {
                    MajorFugitiveKind::Ritchie(ritchie_item_kind) => {
                        match ritchie_item_kind.is_lazy() {
                            true => {
                                let Some(hir_defn) = path.hir_defn(db) else {
                                    unreachable!()
                                };
                                match hir_defn.hir_expr_region(db) {
                                    Some(_) => smallvec![],
                                    None => build(
                                        instantiation,
                                        |instantiation| {
                                            Linkage::new(
                                                db,
                                                LinkageData::MajorRitchieLazy {
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
                                    Linkage::new(
                                        db,
                                        LinkageData::MajorRitchieEager {
                                            path,
                                            instantiation,
                                        },
                                    )
                                },
                                db,
                            ),
                        }
                    }
                    MajorFugitiveKind::Val => {
                        smallvec![Linkage::new(
                            db,
                            LinkageData::MajorVal {
                                path,
                                instantiation: LinInstantiation::new_empty(false),
                            }
                        )]
                    }
                    MajorFugitiveKind::Const => todo!(),
                    MajorFugitiveKind::TypeAlias | MajorFugitiveKind::Formal => unreachable!(),
                },
                JavPath::TypeItem(path) => match path.item_kind(db) {
                    TypeItemKind::AssocRitchie(_) => build(
                        instantiation,
                        |instantiation| {
                            Linkage::new(
                                db,
                                LinkageData::AssocRitchie {
                                    path: path.into(),
                                    instantiation,
                                },
                            )
                        },
                        db,
                    ),
                    TypeItemKind::AssocVal => todo!(),
                    TypeItemKind::AssocType => smallvec![],
                    TypeItemKind::AssocFormal => todo!(),
                    TypeItemKind::AssocConst => todo!(),
                    TypeItemKind::MemoizedField => build(
                        instantiation,
                        |instantiation| {
                            Linkage::new(
                                db,
                                LinkageData::MemoizedField {
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
                            Linkage::new(
                                db,
                                LinkageData::MethodRitchie {
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
                        match ritchie_item_kind.is_lazy() {
                            true => {
                                todo!()
                            }
                            false => build(
                                instantiation,
                                |instantiation| {
                                    Linkage::new(
                                        db,
                                        LinkageData::MethodRitchie {
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
                                LinInstantiation::from_javelin(instantiation, db)
                                    .into_iter()
                                    .flat_map(|instantiation| {
                                        [
                                            Linkage::new(
                                                db,
                                                LinkageData::AssocRitchie {
                                                    path: path.into(),
                                                    instantiation: instantiation.clone(),
                                                },
                                            ),
                                            Linkage::new(
                                                db,
                                                LinkageData::UnveilAssocFn {
                                                    path: path.into(),
                                                    instantiation,
                                                },
                                            ),
                                        ]
                                    })
                                    .collect()
                            }
                            _ => match ritchie_item_kind.is_lazy() {
                                true => todo!(),
                                false => build(
                                    instantiation,
                                    |instantiation| {
                                        Linkage::new(
                                            db,
                                            LinkageData::AssocRitchie {
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
                    TraitItemKind::AssocFormal => todo!(),
                    TraitItemKind::AssocConst => todo!(),
                },
                JavPath::TypeConstructor(path) => {
                    match path.ty_kind(db) {
                        TypeKind::Enum => {
                            if enum_ty_has_only_unit_variants(db, path) {
                                smallvec![Linkage::new_enum_u8_presenter(path, db)]
                            } else {
                                smallvec![]
                            }
                        }
                        TypeKind::Inductive => unreachable!(),
                        TypeKind::Record => unreachable!(),
                        TypeKind::Struct => {
                            let fields: Vec<LinkageStructField> = match path.hir_decl(db).unwrap() {
                                TypeHirDecl::PropsStruct(hir_decl) => hir_decl
                                    .fields(db)
                                    .iter()
                                    .map(|field| LinkageStructField::Props {
                                        ident: field.ident(),
                                    })
                                    .collect(),
                                TypeHirDecl::UnitStruct(_) => vec![],
                                TypeHirDecl::TupleStruct(_) => todo!(),
                                TypeHirDecl::Union(_) => todo!(),
                                _ => unreachable!(),
                            };
                            LinInstantiation::from_javelin(instantiation, db)
                                .into_iter()
                                .flat_map(|instantiation| {
                                    let self_ty = LinTypePathLeading::new(
                                        db,
                                        path,
                                        instantiation
                                            .symbol_resolutions()
                                            .iter()
                                            .map(|(_, res)| match *res {
                                                LinTermSymbolResolution::Explicit(arg) => arg,
                                                LinTermSymbolResolution::SelfLifetime => todo!(),
                                                LinTermSymbolResolution::SelfQuary(_) => todo!(),
                                            })
                                            .collect(),
                                    );
                                    [Linkage::new(
                                        db,
                                        LinkageData::TypeConstructor {
                                            path,
                                            instantiation,
                                        },
                                    )]
                                    .into_iter()
                                    .chain(fields.iter().map(move |&field| {
                                        Linkage::new(
                                            db,
                                            LinkageData::StructField { self_ty, field },
                                        )
                                    }))
                                })
                                .collect()
                        }
                        TypeKind::Structure => unreachable!(),
                        TypeKind::Extern => {
                            p!(path.debug(db));
                            unreachable!()
                        }
                    }
                }
                JavPath::TypeVariantConstructor(path) => {
                    LinInstantiation::from_javelin(instantiation, db)
                        .into_iter()
                        .flat_map(|instantiation| {
                            [Linkage::new(
                                db,
                                LinkageData::TypeVariantConstructor {
                                    path,
                                    instantiation,
                                },
                            )]
                            .into_iter()
                            // todo: chain with pattern matcher
                            // .chain(fields.iter().map(move |&field| {
                            //     Linkage::new(db, LinkageData::StructField { self_ty, field })
                            // }))
                        })
                        .collect()
                }
            }
        }
        JavelinData::VecConstructor { element_ty } => smallvec![Linkage::new(
            db,
            LinkageData::VecConstructor {
                element_ty: LinType::from_javelin(
                    element_ty,
                    // ad hoc
                    &LinInstantiation::new_empty(false),
                    db
                )
            },
        )],
        JavelinData::TypeDefault { ty } => smallvec![Linkage::new(
            db,
            LinkageData::TypeDefault {
                ty: LinType::from_javelin(
                    ty,
                    // ad hoc
                    &LinInstantiation::new_empty(false),
                    db
                )
            },
        )],
    }
}

#[salsa::tracked(jar = LinkageJar, return_ref)]
pub fn package_linkages(db: &::salsa::Db, package_path: PackagePath) -> Vec<Linkage> {
    package_javelins(db, package_path)
        .flat_map(|javelin| linkages_emancipated_by_javelin(db, javelin).iter().copied())
        .collect()
}

#[test]
fn package_linkages_works() {
    DB::ast_expect_test_debug_with_db(
        package_linkages,
        &AstTestConfig::new(
            "package_linkages",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::LINKAGE,
        ),
    )
}
