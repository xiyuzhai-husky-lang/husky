//! This module is responsible for matching a search pattern against a node in the AST. In the
//! process of matching, placeholder values are recorded.

use crate::{
    parsing::{Constraint, NodeKind, Placeholder, Var},
    resolving::{ResolvedPattern, ResolvedRule, UfcsCallInfo},
    SsrMatches,
};
use hir::Semantics;
use ide_db::file_db::FileRange;
use rustc_hash::FxHashMap;
use std::{cell::Cell, iter::Peekable};
use syntax::SmolStr;
use syntax::{ast, SyntaxElement, SyntaxElementChildren, SyntaxKind, SyntaxNode, SyntaxToken};

// Creates a match error. If we're currently attempting to match some code that we thought we were
// going to match, as indicated by the --debug-snippet flag, then populate the reason field.
macro_rules! match_error {
    ($e:expr) => {{
            MatchFailed {
                reason: if recording_match_fail_reasons() {
                    Some(format!("{}", $e))
                } else {
                    None
                }
            }
    }};
    ($fmt:expr, $($arg:tt)+) => {{
        MatchFailed {
            reason: if recording_match_fail_reasons() {
                Some(format!($fmt, $($arg)+))
            } else {
                None
            }
        }
    }};
}

// Fails the current match attempt, recording the supplied reason if we're recording match fail reasons.
macro_rules! fail_match {
    ($($args:tt)*) => {return Err(match_error!($($args)*))};
}

/// Information about a match that was found.
#[derive(Debug)]
pub struct Match {
    pub(crate) range: FileRange,
    pub(crate) matched_node: SyntaxNode,
    pub(crate) placeholder_values: FxHashMap<Var, PlaceholderMatch>,
    pub(crate) ignored_comments: Vec<ast::Comment>,
    pub(crate) rule_index: usize,
    /// The depth of matched_node.
    pub(crate) depth: usize,
    // Each path in the template rendered for the module in which the match was found.
    pub(crate) rendered_template_paths: FxHashMap<SyntaxNode, hir::ModPath>,
}

/// Information about a placeholder bound in a match.
#[derive(Debug)]
pub(crate) struct PlaceholderMatch {
    pub(crate) range: FileRange,
    /// More matches, found within `node`.
    pub(crate) inner_matches: SsrMatches,
    /// How many times the code that the placeholder matched needed to be dereferenced. Will only be
    /// non-zero if the placeholder matched to the receiver of a method call.
    pub(crate) autoderef_count: usize,
    pub(crate) autoref_kind: ast::SelfParamKind,
}

#[derive(Debug)]
pub(crate) struct MatchFailureReason {
    pub(crate) reason: String,
}

/// An "error" indicating that matching failed. Use the fail_match! macro to create and return this.
#[derive(Clone)]
pub(crate) struct MatchFailed {
    /// The reason why we failed to match. Only present when debug_active true in call to
    /// `get_match`.
    pub(crate) reason: Option<String>,
}

/// Checks if `code` matches the search pattern found in `search_scope`, returning information about
/// the match, if it does. Since we only do matching in this module and searching is done by the
/// parent module, we don't populate nested matches.
pub(crate) fn get_match(
    debug_active: bool,
    rule: &ResolvedRule,
    code: &SyntaxNode,
    restrict_range: &Option<FileRange>,
    sema: &Semantics<ide_db::IdeDatabase>,
) -> Result<Match, MatchFailed> {
    record_match_fails_reasons_scope(debug_active, || {
        Matcher::try_match(rule, code, restrict_range, sema)
    })
}

/// Checks if our search pattern matches a particular node of the AST.
struct Matcher<'db, 'sema> {
    sema: &'sema Semantics<'db, ide_db::IdeDatabase>,
    /// If any placeholders come from anywhere outside of this range, then the match will be
    /// rejected.
    restrict_range: Option<FileRange>,
    rule: &'sema ResolvedRule,
}

/// Which phase of matching we're currently performing. We do two phases because most attempted
/// matches will fail and it means we can defer more expensive checks to the second phase.
enum Phase<'a> {
    /// On the first phase, we perform cheap checks. No state is mutated and nothing is recorded.
    First,
    /// On the second phase, we construct the `Match`. Things like what placeholders bind to is
    /// recorded.
    Second(&'a mut Match),
}

impl<'db, 'sema> Matcher<'db, 'sema> {
    fn try_match(
        rule: &ResolvedRule,
        code: &SyntaxNode,
        restrict_range: &Option<FileRange>,
        sema: &'sema Semantics<'db, ide_db::IdeDatabase>,
    ) -> Result<Match, MatchFailed> {
        todo!()
    }

    /// Checks that `range` is within the permitted range if any. This is applicable when we're
    /// processing a macro expansion and we want to fail the match if we're working with a node that
    /// didn't originate from the token tree of the macro call.
    fn validate_range(&self, range: &FileRange) -> Result<(), MatchFailed> {
        if let Some(restrict_range) = &self.restrict_range {
            if restrict_range.file_id != range.file_id
                || !restrict_range.range.contains_range(range.range)
            {
                fail_match!("Node originated from a macro");
            }
        }
        Ok(())
    }

    fn attempt_match_node(
        &self,
        phase: &mut Phase,
        pattern: &SyntaxNode,
        code: &SyntaxNode,
    ) -> Result<(), MatchFailed> {
        todo!()
    }

    fn attempt_match_node_children(
        &self,
        phase: &mut Phase,
        pattern: &SyntaxNode,
        code: &SyntaxNode,
    ) -> Result<(), MatchFailed> {
        todo!()
    }

    fn attempt_match_sequences(
        &self,
        phase: &mut Phase,
        pattern_it: PatternIterator,
        mut code_it: SyntaxElementChildren,
    ) -> Result<(), MatchFailed> {
        todo!()
    }

    fn attempt_match_token(
        &self,
        phase: &mut Phase,
        pattern: &mut Peekable<PatternIterator>,
        code: &syntax::SyntaxToken,
    ) -> Result<(), MatchFailed> {
        todo!()
    }

    fn check_constraint(
        &self,
        constraint: &Constraint,
        code: &SyntaxNode,
    ) -> Result<(), MatchFailed> {
        todo!()
    }

    /// Paths are matched based on whether they refer to the same thing, even if they're written
    /// differently.
    fn attempt_match_path(
        &self,
        phase: &mut Phase,
        pattern: &SyntaxNode,
        code: &SyntaxNode,
    ) -> Result<(), MatchFailed> {
        todo!()
    }

    fn attempt_match_opt(
        &self,
        phase: &mut Phase,
        pattern: Option<SyntaxNode>,
        code: Option<SyntaxNode>,
    ) -> Result<(), MatchFailed> {
        todo!()
    }

    /// We want to allow the records to match in any order, so we have special matching logic for
    /// them.
    fn attempt_match_record_field_list(
        &self,
        phase: &mut Phase,
        pattern: &SyntaxNode,
        code: &SyntaxNode,
    ) -> Result<(), MatchFailed> {
        todo!()
    }

    /// Outside of token trees, a placeholder can only match a single AST node, whereas in a token
    /// tree it can match a sequence of tokens. Note, that this code will only be used when the
    /// pattern matches the macro invocation. For matches within the macro call, we'll already have
    /// expanded the macro.
    fn attempt_match_token_tree(
        &self,
        phase: &mut Phase,
        pattern: &SyntaxNode,
        code: &syntax::SyntaxNode,
    ) -> Result<(), MatchFailed> {
        todo!()
    }

    fn attempt_match_ufcs_to_method_call(
        &self,
        phase: &mut Phase,
        pattern_ufcs: &UfcsCallInfo,
        code: &ast::MethodCallExpr,
    ) -> Result<(), MatchFailed> {
        todo!()
    }

    fn attempt_match_ufcs_to_ufcs(
        &self,
        phase: &mut Phase,
        pattern_ufcs: &UfcsCallInfo,
        code: &ast::CallExpr,
    ) -> Result<(), MatchFailed> {
        todo!()
    }

    /// Verifies that `expr` matches `pattern_type`, possibly after dereferencing some number of
    /// times. Returns the number of times it needed to be dereferenced.
    fn check_expr_type(
        &self,
        pattern_type: &hir::Type,
        expr: &ast::Expr,
    ) -> Result<usize, MatchFailed> {
        todo!()
    }

    fn get_placeholder_for_node(&self, node: &SyntaxNode) -> Option<&Placeholder> {
        todo!()
    }

    fn get_placeholder(&self, element: &SyntaxElement) -> Option<&Placeholder> {
        todo!()
    }
}

impl Match {
    fn render_template_paths(
        &mut self,
        template: &ResolvedPattern,
        sema: &Semantics<ide_db::IdeDatabase>,
    ) -> Result<(), MatchFailed> {
        todo!()
    }
}

impl Phase<'_> {
    fn next_non_trivial(&mut self, code_it: &mut SyntaxElementChildren) -> Option<SyntaxElement> {
        todo!()
    }

    fn record_ignored_comments(&mut self, token: &SyntaxToken) {
        todo!()
    }
}

fn is_closing_token(kind: SyntaxKind) -> bool {
    todo!()
}

pub(crate) fn record_match_fails_reasons_scope<F, T>(debug_active: bool, f: F) -> T
where
    F: Fn() -> T,
{
    RECORDING_MATCH_FAIL_REASONS.with(|c| c.set(debug_active));
    let res = f();
    RECORDING_MATCH_FAIL_REASONS.with(|c| c.set(false));
    res
}

// For performance reasons, we don't want to record the reason why every match fails, only the bit
// of code that the user indicated they thought would match. We use a thread local to indicate when
// we are trying to match that bit of code. This saves us having to pass a boolean into all the bits
// of code that can make the decision to not match.
thread_local! {
    pub static RECORDING_MATCH_FAIL_REASONS: Cell<bool> = Cell::new(false);
}

fn recording_match_fail_reasons() -> bool {
    RECORDING_MATCH_FAIL_REASONS.with(|c| c.get())
}

impl PlaceholderMatch {
    fn from_range(range: FileRange) -> Self {
        todo!()
    }
}

impl NodeKind {
    fn matches(&self, node: &SyntaxNode) -> Result<(), MatchFailed> {
        todo!()
    }
}

// If `node` contains nothing but an ident then return it, otherwise return None.
fn only_ident(element: SyntaxElement) -> Option<SyntaxToken> {
    todo!()
}

struct PatternIterator {
    iter: SyntaxElementChildren,
}

impl Iterator for PatternIterator {
    type Item = SyntaxElement;

    fn next(&mut self) -> Option<SyntaxElement> {
        todo!()
    }
}

impl PatternIterator {
    fn new(parent: &SyntaxNode) -> Self {
        todo!()
    }
}
