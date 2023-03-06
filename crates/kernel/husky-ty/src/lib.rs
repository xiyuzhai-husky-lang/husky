// #![feature(trait_upcasting)]
// #![feature(const_trait_impl)]
// #![feature(const_default_impls)]
// mod db;
// mod entity_path;
// mod error;
// mod field;
// mod intrinsic_ty;
// mod method;
// mod term;
// #[cfg(test)]
// mod tests;

// pub use self::db::*;
// pub use self::entity_path::*;
// pub use self::error::*;
// pub use self::intrinsic_ty::*;
// pub use self::term::*;

// use self::field::*;
// use self::method::*;
// #[cfg(test)]
// use self::tests::*;
// use husky_entity_path::*;
// use husky_entity_taxonomy::*;
// use husky_signature::*;
// use husky_term::*;
// use husky_term_prelude::*;
// use husky_ty_expectation::*;
// use husky_vfs::Toolchain;
// use husky_word::*;

// #[salsa::jar(db=TermDb)]
// pub struct TermJar(
//     ty_ontology_path_ty_unchecked,
//     ty_constructor_path_ty_unchecked,
//     trai_path_ty_unchecked,
//     form_path_ty_unchecked,
//     application_expansion_salsa,
//     ApplicationArguments,
//     entity_ty_method_ty,
//     application_ty_method_ty,
//     entity_ty_field_ty,
//     application_ty_field_ty,
//     application_term_ty,
//     TermSymbols,
//     term_curry_symbols,
//     term_ritchie_symbols,
//     term_application_symbols,
// );
