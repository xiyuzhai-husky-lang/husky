mod entity_path;
mod field;
mod method;

pub use self::entity_path::*;
pub use self::field::*;
pub use self::method::*;

use crate::*;
use husky_raw_ty::{trai_path_raw_ty, ty_constructor_path_raw_ty, ty_ontology_path_raw_ty};
use husky_scope::*;

// impl EtherealTerm {
//     pub fn ty(self, db: &dyn EtherealTypeDb, toolchain: Toolchain) -> TermResult<EtherealTerm> {
//         match self.raw_ty(db)? {
//             Left(raw_ty) => EtherealTerm::ty_from_raw(db, raw_ty),
//             Right(_) => todo!(),
//         }
//     }
// }
