use hir::Semantics;
use ide_db::{
    base_db::{CrateId, FileID, FilePosition},
    RootDatabase,
};
use itertools::Itertools;
use syntax::ast;

use crate::NavigationTarget;

// Feature: Parent Module
//
// Navigates to the parent module of the current module.
//
// |===
// | Editor  | Action Name
//
// | VS Code | **Rust Analyzer: Locate parent module**
// |===
//
// image::https://user-images.githubusercontent.com/48062697/113065580-04c21800-91b1-11eb-9a32-00086161c0bd.gif[]

/// This returns `Vec` because a module may be included from several places.
pub(crate) fn parent_module(db: &RootDatabase, position: FilePosition) -> Vec<NavigationTarget> {
    todo!()
}

/// Returns `Vec` for the same reason as `parent_module`
pub(crate) fn crate_for(db: &RootDatabase, file_id: FileID) -> Vec<CrateId> {
    todo!()
}
