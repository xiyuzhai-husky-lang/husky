//! Code for applying replacement templates for matches that have previously been found.
use common::*;

use crate::{resolving::ResolvedRule, Match, SsrMatches};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use syntax::ast;
use syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

use text_edit::TextEdit;

/// Returns a text edit that will replace each match in `matches` with its corresponding replacement
/// template. Placeholders in the template will have been substituted with whatever they matched to
/// in the original code.
pub(crate) fn matches_to_edit(
    matches: &SsrMatches,
    file_src: &str,
    rules: &[ResolvedRule],
) -> TextEdit {
    matches_to_edit_at_offset(matches, file_src, 0.into(), rules)
}

fn matches_to_edit_at_offset(
    matches: &SsrMatches,
    file_src: &str,
    relative_start: TextSize,
    rules: &[ResolvedRule],
) -> TextEdit {
    let mut edit_builder = TextEdit::builder();
    for m in &matches.matches {
        edit_builder.replace(
            m.range.range.checked_sub(relative_start).unwrap(),
            render_replace(m, file_src, rules),
        );
    }
    edit_builder.finish()
}

struct ReplacementRenderer<'a> {
    match_info: &'a Match,
    file_src: &'a str,
    rules: &'a [ResolvedRule],
    rule: &'a ResolvedRule,
    out: String,
    // Map from a range within `out` to a token in `template` that represents a placeholder. This is
    // used to validate that the generated source code doesn't split any placeholder expansions (see
    // below).
    placeholder_tokens_by_range: FxHashMap<TextRange, SyntaxToken>,
    // Which placeholder tokens need to be wrapped in parenthesis in order to ensure that when `out`
    // is parsed, placeholders don't get split. e.g. if a template of `$a.to_string()` results in `1
    // + 2.to_string()` then the placeholder value `1 + 2` was split and needs parenthesis.
    placeholder_tokens_requiring_parenthesis: FxHashSet<SyntaxToken>,
}

fn render_replace(match_info: &Match, file_src: &str, rules: &[ResolvedRule]) -> String {
    todo!()
}

impl ReplacementRenderer<'_> {
    fn render_node_children(&mut self, node: &SyntaxNode) {
        todo!()
    }

    fn render_node_or_token(&mut self, node_or_token: &SyntaxElement) {
        todo!()
    }

    fn render_node(&mut self, node: &SyntaxNode) {
        todo!()
    }

    fn render_token(&mut self, token: &SyntaxToken) {
        todo!()
    }

    // Checks if the resulting code, when parsed doesn't split any placeholders due to different
    // order of operations between the search pattern and the replacement template. If any do, then
    // we rerender the template and wrap the problematic placeholders with parenthesis.
    fn maybe_rerender_with_extra_parenthesis(&mut self, template: &SyntaxNode) {
        todo!()
    }

    fn remove_node_ranges(&mut self, node: SyntaxNode) {
        todo!()
    }
}

/// Returns whether token is the receiver of a method call. Note, being within the receiver of a
/// method call doesn't count. e.g. if the token is `$a`, then `$a.foo()` will return true, while
/// `($a + $b).foo()` or `x.foo($a)` will return false.
fn token_is_method_call_receiver(token: &SyntaxToken) -> bool {
    todo!()
}

fn parse_as_kind(code: &str, kind: SyntaxKind) -> Option<SyntaxNode> {
    todo!()
}
