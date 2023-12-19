use super::*;

// todo: this should return a template
#[deprecated(note = "it's better to use signature directly instead of invoking this function")]
#[salsa::tracked(jar = DeclarativeTypeJar)]
pub fn ty_variant_path_declarative_ty(
    db: &::salsa::Db,
    path: TypeVariantPath,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    // todo: GADT
    let _declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    let signature_template = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    match signature_template {
        TypeVariantDeclarativeSignatureTemplate::Props(_) => todo!(),
        TypeVariantDeclarativeSignatureTemplate::Unit(signature_template) => {
            let Ok(parent_ty_template_parameter_variances) =
                ty_path_variances(db, path.parent_ty_path(db))
            else {
                todo!()
            };
            // todo: variant implicit parameters
            curry_from_template_parameters(
                db,
                path.toolchain(db),
                CurryKind::Implicit,
                parent_ty_template_parameter_variances,
                signature_template
                    .parent_ty_template(db)
                    .template_parameters(db),
                signature_template.ty(db),
            )
        }
        TypeVariantDeclarativeSignatureTemplate::Tuple(signature_template) => {
            let Ok(parent_ty_template_parameter_variances) =
                ty_path_variances(db, path.parent_ty_path(db))
            else {
                todo!()
            };
            // todo: variant implicit parameters
            curry_from_template_parameters(
                db,
                path.toolchain(db),
                CurryKind::Implicit,
                parent_ty_template_parameter_variances,
                signature_template
                    .parent_ty_template(db)
                    .template_parameters(db),
                signature_template.instance_constructor_ty(db),
            )
        }
    }
}

#[test]
fn ty_variant_path_declarative_ty_works() {
    DB::default().ast_expect_test_debug_with_db(
        |db, module_path: ModulePath| {
            module_item_paths(db, module_path)
                .iter()
                .filter_map(|&module_item_path| match module_item_path {
                    // ad hoc, because module_item_path doesn't include type variant path
                    ItemPath::MajorItem(MajorItemPath::Type(ty_path)) => Some((
                        ty_path,
                        ty_path
                            .ty_variant_paths(db)
                            .iter()
                            .map(|&(_, ty_variant_path)| {
                                (
                                    ty_variant_path,
                                    ty_variant_path_declarative_ty(db, ty_variant_path),
                                )
                            })
                            .collect::<Vec<_>>(),
                    )),
                    ItemPath::TypeVariant(_, path) => {
                        todo!()
                        // Some((path, ty_variant_path_declarative_ty(db, path)))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
        },
        &AstTestConfig::new("ty_variant_path_declarative_ty"),
    );
}
