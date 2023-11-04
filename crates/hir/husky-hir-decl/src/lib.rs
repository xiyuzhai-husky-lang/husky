#![feature(trait_upcasting)]
pub mod db;
mod decl;
// ad hoc
mod ritchie_parameter;
mod template_parameter;
#[cfg(test)]
mod tests;

pub use crate::decl::*;
pub use crate::ritchie_parameter::*;
pub use crate::template_parameter::*;

use crate::db::*;
use husky_coword::*;
use husky_entity_path::*;
use husky_ethereal_signature::*;
use husky_ethereal_term::*;
use husky_hir_eager_expr::*;
use husky_hir_expr::*;
use husky_hir_lazy_expr::*;
use husky_hir_ty::{
    template_parameter::{HirTemplateParameter, HirTemplateParameters},
    *,
};
use husky_vfs::*;
use smallvec::*;
