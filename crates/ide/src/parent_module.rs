use hir::Semantics;
use husky_lang_db::{
    vfs::{SourceFileId, SourceFilePosition},
    HuskyLangDatabase,
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
pub(crate) fn parent_module(
    db: &HuskyLangDatabase,
    position: SourceFilePosition,
) -> Vec<NavigationTarget> {
    todo!()
}
