use crate::*;

#[salsa::jar(db = DeclarativeTypeDb)]
pub struct DeclarativeTypeJar(
    ty_ontology_path_declarative_ty,
    ty_instance_constructor_path_declarative_ty,
    trai_path_declarative_ty,
    // fugitive_path_declarative_ty,
    ty_path_variances,
    ty_template_parameter_variance_reprs,
    declarative_ty_item_variance_crate_dependencies,
    trai_item_variances,
    trai_item_variance_reprs,
    trai_item_variance_crate_dependencies,
    fugitive_path_variances,
    form_item_variance_reprs,
    form_item_variance_crate_dependencies,
    ty_item_path_variances,
    ty_item_item_variance_reprs,
    application_expansion_salsa,
    EtherealApplicationArguments,
    ty_path_ty_method_declarative_ty,
    ty_path_field_declarative_ty,
    application_declarative_term_declarative_ty,
    crate::principal_item_path::ty_variant::ty_variant_path_declarative_ty,
);
