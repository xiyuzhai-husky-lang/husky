mod render;

use std::iter;

use common::*;

use either::Either;
use hir::{HasSource, Semantics};
use husky_lang_db::{
    defs::Definition,
    helpers::{pick_best_token, FamousDefs},
    vfs::FileRange,
    FxIndexSet, HuskyLangDatabase,
};
use itertools::Itertools;
use syntax::{ast, SyntaxKind::*, SyntaxNode, SyntaxToken};

use crate::{
    doc_links::token_as_doc_comment, markup::Markup, FileId, FilePosition, NavigationTarget,
    RangeInfo, TryToNav,
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoverConfig {
    pub links_in_hover: bool,
    pub documentation: Option<HoverDocFormat>,
}

impl HoverConfig {
    fn markdown(&self) -> bool {
        matches!(self.documentation, Some(HoverDocFormat::Markdown))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HoverDocFormat {
    Markdown,
    PlainText,
}

#[derive(Debug, Clone)]
pub enum HoverAction {
    Implementation(FilePosition),
    Reference(FilePosition),
    GoToType(Vec<HoverGotoTypeData>),
}

impl HoverAction {
    fn goto_type_from_targets(db: &HuskyLangDatabase, targets: Vec<hir::ModuleDef>) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct HoverGotoTypeData {
    pub mod_path: String,
    pub nav: NavigationTarget,
}

/// Contains the results when hovering over an item
#[derive(Debug, Default)]
pub struct HoverResult {
    pub markup: Markup,
    pub actions: Vec<HoverAction>,
}

// Feature: Hover
//
// Shows additional information, like the type of an expression or the documentation for a definition when "focusing" code.
// Focusing is usually hovering with a mouse, but can also be triggered with a shortcut.
//
// image::https://user-images.githubusercontent.com/48062697/113020658-b5f98b80-917a-11eb-9f88-3dbc27320c95.gif[]
pub(crate) fn hover(
    db: &HuskyLangDatabase,
    FileRange { file_id, range }: FileRange,
    config: &HoverConfig,
) -> Option<RangeInfo<HoverResult>> {
    todo!()
}

pub(crate) fn hover_for_definition(
    sema: &Semantics<HuskyLangDatabase>,
    file_id: FileId,
    definition: Definition,
    node: &SyntaxNode,
    config: &HoverConfig,
) -> Option<HoverResult> {
    todo!()
}

fn hover_ranged(
    file: &SyntaxNode,
    range: TextRange,
    sema: &Semantics<HuskyLangDatabase>,
    config: &HoverConfig,
) -> Option<RangeInfo<HoverResult>> {
    todo!()
}

fn hover_type_fallback(
    sema: &Semantics<HuskyLangDatabase>,
    config: &HoverConfig,
    token: &SyntaxToken,
) -> Option<RangeInfo<HoverResult>> {
    todo!()
}

fn show_implementations_action(db: &HuskyLangDatabase, def: Definition) -> Option<HoverAction> {
    todo!()
}

fn show_fn_references_action(db: &HuskyLangDatabase, def: Definition) -> Option<HoverAction> {
    todo!()
}

fn runnable_action(
    sema: &hir::Semantics<HuskyLangDatabase>,
    def: Definition,
    file_id: FileId,
) -> Option<HoverAction> {
    todo!()
}

fn goto_type_action_for_def(db: &HuskyLangDatabase, def: Definition) -> Option<HoverAction> {
    todo!()
}

fn walk_and_push_ty(
    db: &HuskyLangDatabase,
    ty: &hir::Type,
    push_new_def: &mut dyn FnMut(hir::ModuleDef),
) {
    todo!()
}

fn dedupe_or_merge_hover_actions(actions: Vec<HoverAction>) -> Vec<HoverAction> {
    let mut deduped_actions = Vec::with_capacity(actions.len());
    let mut go_to_type_targets = FxIndexSet::default();

    let mut seen_implementation = false;
    let mut seen_reference = false;
    let mut seen_runnable = false;
    for action in actions {
        match action {
            HoverAction::GoToType(targets) => {
                go_to_type_targets.extend(targets);
            }
            HoverAction::Implementation(..) => {
                if !seen_implementation {
                    seen_implementation = true;
                    deduped_actions.push(action);
                }
            }
            HoverAction::Reference(..) => {
                if !seen_reference {
                    seen_reference = true;
                    deduped_actions.push(action);
                }
            }
        };
    }

    if !go_to_type_targets.is_empty() {
        deduped_actions.push(HoverAction::GoToType(
            go_to_type_targets.into_iter().collect(),
        ));
    }

    deduped_actions
}
