//! Completion for derives
use hir::{HasAttrs, MacroKind};
use ide_db::helpers::{import_assets::ImportAssets, insert_use::ImportScope, FamousDefs};
use itertools::Itertools;
use rustc_hash::FxHashSet;
use syntax::{ast, SmolStr, SyntaxKind};

use crate::{
    completions::flyimport::compute_fuzzy_completion_order_key,
    context::CompletionContext,
    item::{CompletionItem, CompletionItemKind},
    Completions, ImportEdit,
};

pub(super) fn complete_derive(
    acc: &mut Completions,
    ctx: &CompletionContext,
    existing_derives: &[ast::Path],
) {
    todo!()
}

fn flyimport_attribute(acc: &mut Completions, ctx: &CompletionContext) -> Option<()> {
    todo!()
}

struct DeriveDependencies {
    label: &'static str,
    dependencies: &'static [&'static str],
}

/// Standard Rust derives that have dependencies
/// (the dependencies are needed so that the main derive don't break the compilation when added)
const DEFAULT_DERIVE_DEPENDENCIES: &[DeriveDependencies] = &[
    DeriveDependencies {
        label: "Copy",
        dependencies: &["Clone"],
    },
    DeriveDependencies {
        label: "Eq",
        dependencies: &["PartialEq"],
    },
    DeriveDependencies {
        label: "Ord",
        dependencies: &["PartialOrd", "Eq", "PartialEq"],
    },
    DeriveDependencies {
        label: "PartialOrd",
        dependencies: &["PartialEq"],
    },
];
