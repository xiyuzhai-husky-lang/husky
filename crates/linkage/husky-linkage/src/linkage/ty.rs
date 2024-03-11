use husky_entity_tree::HasTypeVariantPaths;
use husky_hir_decl::decl::TypeVariantHirDecl;

use super::*;

pub(super) fn ty_linkages_emancipated_by_javelin(
    path: TypePath,
    instantiation: &JavInstantiation,
    db: &::salsa::Db,
) -> SmallVec<[Linkage; 4]> {
    match path.ty_kind(db) {
        TypeKind::Enum => enum_ty_linkages_emancipated_by_javelin(path, instantiation, db),
        TypeKind::Struct => struct_ty_linkages_emancipated_by_javelin(path, instantiation, db),
        TypeKind::Inductive | TypeKind::Record | TypeKind::Structure | TypeKind::Extern => {
            unreachable!()
        }
    }
}

pub(super) fn enum_ty_linkages_emancipated_by_javelin(
    path: TypePath,
    instantiation: &JavInstantiation,
    db: &::salsa::Db,
) -> SmallVec<[Linkage; 4]> {
    let mut linkages: SmallVec<[Linkage; 4]> = smallvec![];
    if enum_ty_has_only_unit_variants(db, path) {
        linkages.push(Linkage::new_enum_u8_presenter(path, db))
    }
    for instantiation in LinInstantiation::from_javelin(instantiation, db) {
        for &(_, path) in path.ty_variant_paths(db) {
            let hir_defn = path.hir_decl(db).unwrap();
            linkages.push(Linkage::new(
                db,
                LinkageData::EnumVariantConstructor {
                    path,
                    instantiation: instantiation.clone(),
                },
            ));
            linkages.push(Linkage::new(
                db,
                LinkageData::EnumVariantDiscriminator {
                    path,
                    instantiation: instantiation.clone(),
                },
            ));
            match hir_defn {
                TypeVariantHirDecl::Props(hir_defn) => {
                    linkages.push(Linkage::new(
                        db,
                        LinkageData::EnumVariantDestructor {
                            path,
                            instantiation: instantiation.clone(),
                        },
                    ));
                    for field in hir_defn.fields(db) {
                        todo!()
                    }
                }
                TypeVariantHirDecl::Tuple(hir_defn) => {
                    linkages.push(Linkage::new(
                        db,
                        LinkageData::EnumVariantDestructor {
                            path,
                            instantiation: instantiation.clone(),
                        },
                    ));
                    for (index, field) in hir_defn.fields(db).iter().enumerate() {
                        linkages.push(Linkage::new(
                            db,
                            LinkageData::EnumVariantField {
                                path,
                                instantiation: instantiation.clone(),
                                field: LinkageField::Tuple {
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
    linkages
}

pub(super) fn struct_ty_linkages_emancipated_by_javelin(
    path: TypePath,
    instantiation: &JavInstantiation,
    db: &::salsa::Db,
) -> SmallVec<[Linkage; 4]> {
    let mut linkages: SmallVec<[Linkage; 4]> = smallvec![];
    let fields: Vec<LinkageField> = match path.hir_decl(db).unwrap() {
        TypeHirDecl::PropsStruct(hir_decl) => hir_decl
            .fields(db)
            .iter()
            .map(|field| LinkageField::Props {
                ident: field.ident(),
            })
            .collect(),
        TypeHirDecl::UnitStruct(_) => vec![],
        TypeHirDecl::TupleStruct(_) => todo!(),
        TypeHirDecl::Union(_) => todo!(),
        _ => unreachable!(),
    };
    for instantiation in LinInstantiation::from_javelin(instantiation, db) {
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
        linkages.push(Linkage::new(
            db,
            LinkageData::StructConstructor {
                path,
                instantiation: instantiation.clone(),
            },
        ));
        linkages.push(Linkage::new(
            db,
            LinkageData::StructDestructor {
                path,
                instantiation: instantiation.clone(),
            },
        ));
        for &field in &fields {
            linkages.push(Linkage::new(
                db,
                LinkageData::StructField { self_ty, field },
            ))
        }
    }
    linkages
}
