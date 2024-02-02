use super::*;

// todo: this should return a template
#[salsa::tracked(jar = DeclarativeTypeJar)]
pub fn ty_variant_path_declarative_ty(
    db: &::salsa::Db,
    path: TypeVariantPath,
) -> DeclarativeTypeResult<DeclarativeTerm> {
    // todo: GADT
    let _declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    let tmpl = match path.declarative_signature_template(db) {
        Ok(tmpl) => tmpl,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    match tmpl {
        TypeVariantDecTemplate::Props(tmpl) => {
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
                tmpl.parent_ty_template(db).template_parameters(db),
                tmpl.instance_constructor_ty(db),
            )
        }
        TypeVariantDecTemplate::Tuple(tmpl) => {
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
                tmpl.parent_ty_template(db).template_parameters(db),
                tmpl.instance_constructor_ty(db),
            )
        }
        TypeVariantDecTemplate::Unit(signature_template) => {
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
                signature_template.self_ty(db),
            )
        }
    }
}

#[test]
fn ty_variant_path_declarative_ty_works() {
    use husky_entity_tree::helpers::paths::module_item_paths;

    DB::ast_expect_test_debug_with_db(
        |db, module_path: husky_vfs::ModulePath| {
            use husky_entity_tree::HasTypeVariantPaths;
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
                    ItemPath::TypeVariant(_, _path) => {
                        todo!()
                        // Some((path, ty_variant_path_declarative_ty(db, path)))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
        },
        &AstTestConfig::new(
            "ty_variant_path_declarative_ty",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::KERNEL,
        ),
    );
}
