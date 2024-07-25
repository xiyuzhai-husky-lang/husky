use husky_entity_tree::node::ty_variant::HasTypeVariantPaths;
use husky_hir_decl::decl::TypeVariantHirDecl;

use super::*;

pub(super) fn ty_linkets_emancipated_by_javelin(
    path: TypePath,
    instantiation: &JavInstantiation,
    db: &::salsa::Db,
) -> SmallVec<[Linket; 4]> {
    match path.ty_kind(db) {
        TypeKind::Enum => enum_ty_linkets_emancipated_by_javelin(path, instantiation, db),
        TypeKind::Struct => struct_ty_linkets_emancipated_by_javelin(path, instantiation, db),
        TypeKind::Inductive | TypeKind::Record | TypeKind::Structure | TypeKind::Extern => {
            unreachable!()
        }
    }
}

pub(super) fn enum_ty_linkets_emancipated_by_javelin(
    path: TypePath,
    instantiation: &JavInstantiation,
    db: &::salsa::Db,
) -> SmallVec<[Linket; 4]> {
    let mut linkets: SmallVec<[Linket; 4]> = smallvec![];
    if enum_ty_has_only_unit_variants(db, path) {
        linkets.push(Linket::new_enum_index_presenter(path, db))
    }
    for instantiation in LinInstantiation::from_jav(instantiation, db) {
        let self_ty = LinTypePathLeading::from_path_instantiation(path, &instantiation, db);
        for &(_, path) in path.ty_variant_paths(db) {
            let hir_defn = path.hir_decl(db).unwrap();
            linkets.push(Linket::new(
                db,
                LinketData::EnumVariantConstructor {
                    self_ty,
                    path,
                    instantiation: instantiation.clone(),
                },
            ));
            linkets.push(Linket::new(
                db,
                LinketData::EnumVariantDiscriminator {
                    self_ty,
                    path,
                    instantiation: instantiation.clone(),
                },
            ));
            match hir_defn {
                TypeVariantHirDecl::Props(hir_defn) => {
                    linkets.push(Linket::new(
                        db,
                        LinketData::EnumVariantDestructor {
                            self_ty,
                            path,
                            instantiation: instantiation.clone(),
                        },
                    ));
                    for field in hir_defn.fields(db) {
                        linkets.push(Linket::new(
                            db,
                            LinketData::EnumVariantField {
                                path,
                                instantiation: instantiation.clone(),
                                field_ty_leash_class: field
                                    .ty()
                                    .lin_instantiate(&instantiation, db)
                                    .ty_leash_class(db),
                                field: LinField::Props {
                                    ident: field.ident(),
                                },
                            },
                        ));
                    }
                }
                TypeVariantHirDecl::Tuple(hir_defn) => {
                    linkets.push(Linket::new(
                        db,
                        LinketData::EnumVariantDestructor {
                            self_ty,
                            path,
                            instantiation: instantiation.clone(),
                        },
                    ));
                    for (index, field) in hir_defn.fields(db).iter().enumerate() {
                        linkets.push(Linket::new(
                            db,
                            LinketData::EnumVariantField {
                                path,
                                instantiation: instantiation.clone(),
                                field_ty_leash_class: field
                                    .ty()
                                    .lin_instantiate(&instantiation, db)
                                    .ty_leash_class(db),
                                field: LinField::Tuple {
                                    index: index.try_into().unwrap(),
                                },
                            },
                        ));
                    }
                }
                TypeVariantHirDecl::Unit(_) => (),
            }
        }
    }
    linkets
}

pub(super) fn struct_ty_linkets_emancipated_by_javelin(
    path: TypePath,
    instantiation: &JavInstantiation,
    db: &::salsa::Db,
) -> SmallVec<[Linket; 4]> {
    let mut linkets: SmallVec<[Linket; 4]> = smallvec![];
    for instantiation in LinInstantiation::from_jav(instantiation, db) {
        let fields: Vec<(LinLeashClass, LinField)> = match path.hir_decl(db).unwrap() {
            TypeHirDecl::PropsStruct(hir_decl) => hir_decl
                .fields(db)
                .iter()
                .map(|field| {
                    (
                        field
                            .ty()
                            .lin_instantiate(&instantiation, db)
                            .ty_leash_class(db),
                        LinField::Props {
                            ident: field.ident(),
                        },
                    )
                })
                .collect(),
            TypeHirDecl::UnitStruct(_) => vec![],
            TypeHirDecl::TupleStruct(_) => todo!(),
            TypeHirDecl::Union(_) => todo!(),
            _ => unreachable!(),
        };
        let self_ty = LinTypePathLeading::from_path_instantiation(path, &instantiation, db);
        linkets.push(Linket::new(
            db,
            LinketData::StructConstructor {
                path,
                instantiation: instantiation.clone(),
            },
        ));
        if !fields.is_empty() {
            linkets.push(Linket::new(db, LinketData::StructDestructor { self_ty }));
        }
        for (field_ty_leash_class, field) in fields {
            linkets.push(Linket::new(
                db,
                LinketData::StructField {
                    self_ty,
                    field_ty_leash_class,
                    field,
                },
            ))
        }
    }
    linkets
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinLeashClass {
    Copyable,
    Vec,
    Other,
}

impl LinType {
    fn ty_leash_class(self, db: &::salsa::Db) -> LinLeashClass {
        match self {
            LinType::PathLeading(slf) => {
                if slf.is_copyable(db) {
                    LinLeashClass::Copyable
                } else {
                    LinLeashClass::Other
                }
            }
            LinType::Ritchie(_) => LinLeashClass::Copyable,
        }
    }
}

impl LinLeashClass {
    pub fn code(self) -> &'static str {
        match self {
            LinLeashClass::Copyable => "copyable",
            LinLeashClass::Vec => "vec",
            LinLeashClass::Other => "other",
        }
    }
}
