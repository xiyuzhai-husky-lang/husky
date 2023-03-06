// #![feature(trait_upcasting)]
// mod db;
// mod entity_path;
// mod error;

// pub use self::db::*;
// pub use self::entity_path::*;
// pub use self::error::*;

// use husky_entity_path::*;
// use husky_precise_term::*;
// use husky_ty_expectation::*;
// use husky_vfs::*;

// #[salsa::jar(db = PreciseTermDb)]
// pub struct PreciseTypeJar(
//     ty_ontology_path_ty,
//     ty_constructor_path_ty,
//     form_path_ty,
//     trai_path_ty,
// );

// pub trait HasPreciseType<Db: ?Sized + PreciseTermDb>: Copy {
//     fn ty(self, db: &Db) -> RawTerm;
// }

// impl<Db: ?Sized + PreciseTermDb> HasPreciseType<Db> for RawTerm {
//     fn ty(self, db: &Db) -> RawTerm {
//         match self {
//             RawTerm::Literal(_) => todo!(),
//             RawTerm::Symbol(_) => todo!(),
//             RawTerm::EntityPath(_) => todo!(),
//             RawTerm::Category(_) => todo!(),
//             RawTerm::Universe(_) => todo!(),
//             RawTerm::Curry(_) => todo!(),
//             RawTerm::Ritchie(_) => todo!(),
//             RawTerm::Abstraction(_) => todo!(),
//             RawTerm::Application(_) => todo!(),
//             RawTerm::Subentity(_) => todo!(),
//             RawTerm::AsTraitSubentity(_) => todo!(),
//             RawTerm::TraitConstraint(_) => todo!(),
//         }
//     }
// }
