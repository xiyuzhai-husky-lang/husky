//! This module is responsible for resolving paths within rules.

use crate::errors::error;
use crate::{parsing, SsrError};
use ide_db::file_db::FilePosition;
use parsing::Placeholder;
use rustc_hash::FxHashMap;
use syntax::{ast, SmolStr, SyntaxKind, SyntaxNode, SyntaxToken};

pub(crate) struct ResolutionScope<'db> {
    scope: hir::SemanticsScope<'db>,
    node: SyntaxNode,
}

pub(crate) struct ResolvedRule {
    pub(crate) pattern: ResolvedPattern,
    pub(crate) template: Option<ResolvedPattern>,
    pub(crate) index: usize,
}

pub(crate) struct ResolvedPattern {
    pub(crate) placeholders_by_stand_in: FxHashMap<SmolStr, parsing::Placeholder>,
    pub(crate) node: SyntaxNode,
    // Paths in `node` that we've resolved.
    pub(crate) resolved_paths: FxHashMap<SyntaxNode, ResolvedPath>,
    pub(crate) ufcs_function_calls: FxHashMap<SyntaxNode, UfcsCallInfo>,
    pub(crate) contains_self: bool,
}

pub(crate) struct ResolvedPath {
    pub(crate) resolution: hir::PathResolution,
    /// The depth of the ast::Path that was resolved within the pattern.
    pub(crate) depth: u32,
}

pub(crate) struct UfcsCallInfo {
    pub(crate) call_expr: ast::CallExpr,
    pub(crate) function: hir::Function,
    pub(crate) qualifier_type: Option<hir::Type>,
}

impl ResolvedRule {
    pub(crate) fn new(
        rule: parsing::ParsedRule,
        resolution_scope: &ResolutionScope,
        index: usize,
    ) -> Result<ResolvedRule, SsrError> {
        let resolver = Resolver {
            resolution_scope,
            placeholders_by_stand_in: rule.placeholders_by_stand_in,
        };
        let resolved_template = match rule.template {
            Some(template) => Some(resolver.resolve_pattern_tree(template)?),
            None => None,
        };
        Ok(ResolvedRule {
            pattern: resolver.resolve_pattern_tree(rule.pattern)?,
            template: resolved_template,
            index,
        })
    }

    pub(crate) fn get_placeholder(&self, token: &SyntaxToken) -> Option<&Placeholder> {
        todo!()
    }
}

struct Resolver<'a, 'db> {
    resolution_scope: &'a ResolutionScope<'db>,
    placeholders_by_stand_in: FxHashMap<SmolStr, parsing::Placeholder>,
}

impl Resolver<'_, '_> {
    fn resolve_pattern_tree(&self, pattern: SyntaxNode) -> Result<ResolvedPattern, SsrError> {
        todo!()
    }

    fn resolve(
        &self,
        node: SyntaxNode,
        depth: u32,
        resolved_paths: &mut FxHashMap<SyntaxNode, ResolvedPath>,
    ) -> Result<(), SsrError> {
        todo!()
    }

    /// Returns whether `path` contains a placeholder, but ignores any placeholders within type
    /// arguments.
    fn path_contains_placeholder(&self, path: &ast::Path) -> bool {
        todo!()
    }

    fn ok_to_use_path_resolution(&self, resolution: &hir::PathResolution) -> bool {
        todo!()
    }
}

impl<'db> ResolutionScope<'db> {
    pub(crate) fn new(
        sema: &hir::Semantics<'db, ide_db::IdeDatabase>,
        resolve_context: FilePosition,
    ) -> ResolutionScope<'db> {
        todo!()
    }

    /// Returns the function in which SSR was invoked, if any.
    pub(crate) fn current_function(&self) -> Option<SyntaxNode> {
        todo!()
    }

    fn resolve_path(&self, path: &ast::Path) -> Option<hir::PathResolution> {
        todo!()
    }

    fn qualifier_type(&self, path: &SyntaxNode) -> Option<hir::Type> {
        todo!()
    }
}

fn is_self(path: &ast::Path) -> bool {
    todo!()
}

/// Returns a suitable node for resolving paths in the current scope. If we create a scope based on
/// a statement node, then we can't resolve local variables that were defined in the current scope
/// (only in parent scopes). So we find another node, ideally a child of the statement where local
/// variable resolution is permitted.
fn pick_node_for_resolution(node: SyntaxNode) -> SyntaxNode {
    todo!()
}

/// Returns whether `path` or any of its qualifiers contains type arguments.
fn path_contains_type_arguments(path: Option<ast::Path>) -> bool {
    todo!()
}
