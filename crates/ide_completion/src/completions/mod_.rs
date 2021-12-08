//! Completes mod declarations.

use std::iter;

use hir::{Module, ModuleSource};
use husky_lang_db::{HuskyLangDatabase, SymbolKind};
use rustc_hash::FxHashSet;

use crate::CompletionItem;

use crate::{context::CompletionContext, Completions};

/// Complete mod declaration, i.e. `mod $0 ;`
pub(crate) fn complete_mod(acc: &mut Completions, ctx: &CompletionContext) -> Option<()> {
    todo!()
}

// fn directory_to_look_for_submodules(
//     module: Module,
//     db: &HuskyLangDatabase,
//     module_file_path: &VfsPath,
// ) -> Option<VfsPath> {
//     todo!()
// }

fn module_chain_to_containing_module_file(
    current_module: Module,
    db: &HuskyLangDatabase,
) -> Vec<Module> {
    todo!()
}
