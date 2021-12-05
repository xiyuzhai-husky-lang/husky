//! Converts a flat collection of matches into a nested form suitable for replacement. When there
//! are multiple matches for a node, or that overlap, priority is given to the earlier rule. Nested
//! matches are only permitted if the inner match is contained entirely within a placeholder of an
//! outer match.
//!
//! For example, if our search pattern is `foo(foo($a))` and the code had `foo(foo(foo(foo(42))))`,
//! then we'll get 3 matches, however only the outermost and innermost matches can be accepted. The
//! middle match would take the second `foo` from the outer match.

use crate::{Match, SsrMatches};
use rustc_hash::FxHashMap;
use syntax::SyntaxNode;

pub(crate) fn nest_and_remove_collisions(
    mut matches: Vec<Match>,
    sema: &hir::Semantics<ide_db::RootDatabase>,
) -> SsrMatches {
    // We sort the matches by depth then by rule index. Sorting by depth means that by the time we
    // see a match, any parent matches or conflicting matches will have already been seen. Sorting
    // by rule_index means that if there are two matches for the same node, the rule added first
    // will take precedence.
    matches.sort_by(|a, b| {
        a.depth
            .cmp(&b.depth)
            .then_with(|| a.rule_index.cmp(&b.rule_index))
    });
    let mut collector = MatchCollector::default();
    for m in matches {
        collector.add_match(m, sema);
    }
    collector.into()
}

#[derive(Default)]
struct MatchCollector {
    matches_by_node: FxHashMap<SyntaxNode, Match>,
}

impl MatchCollector {
    /// Attempts to add `m` to matches. If it conflicts with an existing match, it is discarded. If
    /// it is entirely within the a placeholder of an existing match, then it is added as a child
    /// match of the existing match.
    fn add_match(&mut self, m: Match, sema: &hir::Semantics<ide_db::RootDatabase>) {
        todo!()
    }
}

/// Attempts to add `m` as a sub-match of `existing`.
fn try_add_sub_match(m: Match, existing: &mut Match, sema: &hir::Semantics<ide_db::RootDatabase>) {
    todo!()
}

impl From<MatchCollector> for SsrMatches {
    fn from(mut match_collector: MatchCollector) -> Self {
        let mut matches = SsrMatches::default();
        for (_, m) in match_collector.matches_by_node.drain() {
            matches.matches.push(m);
        }
        matches.matches.sort_by(|a, b| {
            // Order matches by file_id then by start range. This should be sufficient since ranges
            // shouldn't be overlapping.
            a.range
                .file_id
                .cmp(&b.range.file_id)
                .then_with(|| a.range.range.start().cmp(&b.range.range.start()))
        });
        matches
    }
}
