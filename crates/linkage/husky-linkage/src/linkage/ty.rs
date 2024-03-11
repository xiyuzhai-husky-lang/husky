use husky_entity_tree::HasTypeVariantPaths;

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
            linkages.push(Linkage::new(
                db,
                LinkageData::EnumTypeVariantConstructor {
                    path,
                    instantiation: instantiation.clone(),
                },
            ));
            linkages.push(Linkage::new(
                db,
                LinkageData::EnumTypeVariantDiscriminator {
                    path,
                    instantiation: instantiation.clone(),
                },
            ));
            for &qual in LinQual::ALL {
                linkages.push(Linkage::new(
                    db,
                    LinkageData::EnumTypeVariantDestructor {
                        path,
                        instantiation: instantiation.clone(),
                        qual,
                    },
                ))
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
            LinkageData::StructTypeConstructor {
                path,
                instantiation: instantiation.clone(),
            },
        ));
        for &qual in LinQual::ALL {
            linkages.push(Linkage::new(
                db,
                LinkageData::StructTypeDestructor {
                    path,
                    instantiation: instantiation.clone(),
                    qual,
                },
            ))
        }
        for &field in &fields {
            linkages.push(Linkage::new(
                db,
                LinkageData::StructField { self_ty, field },
            ))
        }
    }
    linkages
}
