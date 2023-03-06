// #![doc = include_str!("../README.md")]
// #![feature(trait_upcasting)]
// #![feature(let_chains)]
// // #![deny(unsafe_code, missing_docs, clippy::unwrap_used)]

// mod context;
// mod db;
// mod error;
// mod rewrite;
// mod term;
// #[cfg(test)]
// mod tests;

// pub use self::context::*;
// pub use self::db::*;
// pub use self::error::*;
// pub use self::rewrite::*;
// pub use self::term::*;

// use either::*;
// use husky_entity_path::EntityPath;
// use husky_print_utils::p;
// use husky_raw_term::*;
// use husky_raw_ty::*;
// use husky_term_prelude::*;
// use husky_vfs::*;
// use husky_word::Identifier;

// #[salsa::jar(db = RawTermDb)]
// pub struct RawTermJar(
//     RawTermSymbol,
//     RawTermCurry,
//     RawTermRitchie,
//     RawTermAbstraction,
//     RawTermApplication,
//     precise_term_application_from_raw,
//     precise_term_application_raw_ty,
//     RawTermSubentity,
//     RawTermAsTraitSubentity,
//     RawTermTraitConstraint,
//     // curry
//     precise_term_curry_from_raw,
// );
