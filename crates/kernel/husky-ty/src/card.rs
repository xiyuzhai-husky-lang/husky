mod entity_path;
mod field;
mod method;

pub use self::entity_path::*;
pub use self::field::*;
pub use self::method::*;

use crate::*;
use husky_raw_ty::{trai_path_raw_ty, ty_constructor_path_raw_ty, ty_ontology_path_raw_ty};
use husky_scope::*;

// impl Term {
//     pub fn ty(self, db: &dyn TypeDb, toolchain: Toolchain) -> TermResult<Term> {
//         match self.raw_ty(db)? {
//             Left(raw_ty) => Term::ty_from_raw(db, raw_ty),
//             Right(_) => todo!(),
//         }
//     }
// }
