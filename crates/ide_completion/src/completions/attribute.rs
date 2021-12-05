//! Completion for attributes
//!
//! This module uses a bit of static metadata to provide completions
//! for built-in attributes.

use ide_db::helpers::generated_lints::{CLIPPY_LINTS, DEFAULT_LINTS, FEATURES, RUSTDOC_LINTS};
use itertools::Itertools;
use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;
use syntax::{ast, Direction, SyntaxKind};

use crate::{
    context::CompletionContext,
    item::{CompletionItem, CompletionItemKind},
    Completions,
};

pub(crate) fn complete_attribute(acc: &mut Completions, ctx: &CompletionContext) -> Option<()> {
    todo!()
}

fn complete_new_attribute(acc: &mut Completions, ctx: &CompletionContext, attribute: &ast::Attr) {
    todo!()
}

struct AttrCompletion {
    label: &'static str,
    lookup: Option<&'static str>,
    snippet: Option<&'static str>,
    prefer_inner: bool,
}

impl AttrCompletion {
    fn key(&self) -> &'static str {
        self.lookup.unwrap_or(self.label)
    }

    const fn prefer_inner(self) -> AttrCompletion {
        AttrCompletion {
            prefer_inner: true,
            ..self
        }
    }
}

const fn attr(
    label: &'static str,
    lookup: Option<&'static str>,
    snippet: Option<&'static str>,
) -> AttrCompletion {
    AttrCompletion {
        label,
        lookup,
        snippet,
        prefer_inner: false,
    }
}

fn parse_comma_sep_paths(input: ast::TokenTree) -> Option<Vec<ast::Path>> {
    todo!()
}

fn parse_comma_sep_expr(input: ast::TokenTree) -> Option<Vec<ast::Expr>> {
    todo!()
}
