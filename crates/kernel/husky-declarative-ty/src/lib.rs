#![feature(trait_upcasting)]
#![feature(const_trait_impl)]
mod db;
mod error;
mod field;
mod method;
mod principal_entity_path;
mod term;
#[cfg(test)]
mod tests;
mod variance;

pub use self::db::*;
pub use self::error::*;
pub use self::field::*;
pub use self::method::*;
pub use self::principal_entity_path::*;
pub use self::term::*;

#[cfg(test)]
use self::tests::*;
use self::variance::*;
use husky_decl::HasDecl;
use husky_declarative_signature::*;
use husky_declarative_term::*;
use husky_entity_path::*;
use husky_term_prelude::*;
use husky_ty_expectation::TypePathDisambiguation;
use husky_word::*;
use map_collect::*;

#[salsa::jar(db = DeclarativeTypeDb)]
pub struct DeclarativeTypeJar(
    ty_ontology_path_declarative_ty,
    ty_instance_constructor_path_declarative_ty,
    trai_path_declarative_ty,
    form_path_declarative_ty,
    ty_implicit_parameter_variances,
    ty_implicit_parameter_variance_reprs,
    declarative_ty_entity_variance_crate_dependencies,
    trai_entity_variances,
    trai_entity_variance_reprs,
    trai_entity_variance_crate_dependencies,
    form_entity_variances,
    form_entity_variance_reprs,
    form_entity_variance_crate_dependencies,
    ty_item_entity_variances,
    ty_item_entity_variance_reprs,
    application_expansion_salsa,
    EtherealApplicationArguments,
    ty_path_ty_method_declarative_ty,
    ty_path_field_declarative_ty,
    application_declarative_term_declarative_ty,
    ty_variant_path_declarative_ty,
);

pub trait HasDeclarativeType: Copy {
    fn declarative_ty(self, db: &dyn DeclarativeTypeDb) -> DeclarativeTypeResult<DeclarativeTerm>;
}
