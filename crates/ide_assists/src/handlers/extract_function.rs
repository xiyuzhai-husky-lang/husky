use std::iter;

use common::*;

use either::Either;
use hir::{HirDisplay, InFile, Local, ModuleDef, Semantics};
use husky_lang_db::{
    defs::{Definition, NameRefClass},
    helpers::{
        insert_use::{insert_use, ImportScope},
        mod_path_to_ast,
        node_ext::{walk_expr, walk_pat, walk_patterns_in_expr},
        FamousDefs,
    },
    search::{FileReference, ReferenceCategory, SearchScope},
    FxIndexSet, HuskyLangDatabase,
};
use itertools::Itertools;
use stdx::format_to;
use syntax::{
    ast::{self, edit::IndentLevel},
    SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken, TokenAtOffset,
};

use crate::{
    assist_context::{AssistContext, Assists, TreeMutator},
    AssistId,
};

pub(crate) fn extract_function(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

fn extraction_target(node: &SyntaxNode, selection_range: TextRange) -> Option<FunctionBody> {
    todo!()
}

#[derive(Debug)]
struct Function {
    name: ast::NameRef,
    self_param: Option<ast::SelfParam>,
    params: Vec<Param>,
    control_flow: ControlFlow,
    ret_ty: RetType,
    body: FunctionBody,
    outliving_locals: Vec<OutlivedLocal>,
    mods: ContainerInfo,
}

#[derive(Debug)]
struct Param {
    var: Local,
    ty: hir::Type,
    move_local: bool,
    requires_mut: bool,
    is_copy: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParamKind {
    Value,
    MutValue,
    SharedRef,
    MutRef,
}

#[derive(Debug, Eq, PartialEq)]
enum FunType {
    Unit,
    Single(hir::Type),
    Tuple(Vec<hir::Type>),
}

/// Where to put extracted function definition
#[derive(Debug)]
enum Anchor {
    /// Extract free function and put right after current top-level function
    Freestanding,
    /// Extract method and put right after current function in the impl-block
    Method,
}

// FIXME: ControlFlow and ContainerInfo both track some function modifiers, feels like these two should
// probably be merged somehow.
#[derive(Debug)]
struct ControlFlow {
    kind: Option<FlowKind>,
    is_async: bool,
    is_unsafe: bool,
}

/// The thing whose expression we are extracting from. Can be a function, const, static, const arg, ...
#[derive(Clone, Debug)]
struct ContainerInfo {
    is_const: bool,
    is_in_tail: bool,
    parent_loop: Option<SyntaxNode>,
    /// The function's return type, const's type etc.
    ret_type: Option<hir::Type>,
}

/// Control flow that is exported from extracted function
///
/// E.g.:
/// ```rust,no_run
/// loop {
///     $0
///     if 42 == 42 {
///         break;
///     }
///     $0
/// }
/// ```
#[derive(Debug, Clone)]
enum FlowKind {
    /// Return with value (`return $expr;`)
    Return(Option<ast::Expr>),
    Try {
        kind: TryKind,
    },
    /// Break with value (`break $expr;`)
    Break(Option<ast::Expr>),
    /// Continue
    Continue,
}

#[derive(Debug, Clone)]
enum TryKind {
    Option,
    Result { ty: hir::Type },
}

#[derive(Debug)]
enum RetType {
    Expr(hir::Type),
    Stmt,
}

impl RetType {
    fn is_unit(&self) -> bool {
        todo!()
    }
}

/// Semantically same as `ast::Expr`, but preserves identity when using only part of the Block
/// This is the future function body, the part that is being extracted.
#[derive(Debug)]
enum FunctionBody {
    Expr(ast::Expr),
    Span {
        parent: ast::StmtList,
        text_range: TextRange,
    },
}

#[derive(Debug)]
struct OutlivedLocal {
    local: Local,
    mut_usage_outside_body: bool,
}

/// Container of local variable usages
///
/// Semanticall same as `UsageSearchResult`, but provides more convenient interface
struct LocalUsages(husky_lang_db::search::UsageSearchResult);

impl LocalUsages {
    fn find_local_usages(ctx: &AssistContext, var: Local) -> Self {
        Self(
            Definition::Local(var)
                .usages(&ctx.sema)
                .in_scope(SearchScope::single_file(ctx.file_id()))
                .all(),
        )
    }

    fn iter(&self) -> impl Iterator<Item = &FileReference> + '_ {
        self.0.iter().flat_map(|(_, rs)| rs)
    }
}

impl Function {
    fn return_type(&self, ctx: &AssistContext) -> FunType {
        todo!()
    }
}

impl ParamKind {
    fn is_ref(&self) -> bool {
        matches!(self, ParamKind::SharedRef | ParamKind::MutRef)
    }
}

impl Param {
    fn kind(&self) -> ParamKind {
        match (self.move_local, self.requires_mut, self.is_copy) {
            (false, true, _) => ParamKind::MutRef,
            (false, false, false) => ParamKind::SharedRef,
            (true, true, _) => ParamKind::MutValue,
            (_, false, _) => ParamKind::Value,
        }
    }

    fn to_arg(&self, ctx: &AssistContext) -> ast::Expr {
        todo!()
    }

    fn to_param(&self, ctx: &AssistContext, module: hir::Module) -> ast::Param {
        todo!()
    }
}

impl TryKind {
    fn of_ty(ty: hir::Type, ctx: &AssistContext) -> Option<TryKind> {
        todo!()
    }
}

impl FlowKind {
    fn make_result_handler(&self, expr: Option<ast::Expr>) -> ast::Expr {
        todo!()
    }

    fn expr_ty(&self, ctx: &AssistContext) -> Option<hir::Type> {
        todo!()
    }
}

impl FunctionBody {
    fn parent(&self) -> Option<SyntaxNode> {
        todo!()
    }

    fn from_expr(expr: ast::Expr) -> Option<Self> {
        todo!()
    }

    fn from_range(parent: ast::StmtList, selected: TextRange) -> FunctionBody {
        todo!()
    }

    fn indent_level(&self) -> IndentLevel {
        todo!()
    }

    fn tail_expr(&self) -> Option<ast::Expr> {
        todo!()
    }

    fn walk_expr(&self, cb: &mut dyn FnMut(ast::Expr)) {
        todo!()
    }

    fn preorder_expr(&self, cb: &mut dyn FnMut(WalkEvent<ast::Expr>) -> bool) {
        todo!()
    }

    fn walk_pat(&self, cb: &mut dyn FnMut(ast::Pat)) {
        todo!()
    }

    fn text_range(&self) -> TextRange {
        todo!()
    }

    fn contains_range(&self, range: TextRange) -> bool {
        self.text_range().contains_range(range)
    }

    fn precedes_range(&self, range: TextRange) -> bool {
        self.text_range().end() <= range.start()
    }

    fn contains_node(&self, node: &SyntaxNode) -> bool {
        todo!()
    }
}

impl FunctionBody {
    /// Analyzes a function body, returning the used local variables that are referenced in it as well as
    /// whether it contains an await expression.
    fn analyze(
        &self,
        sema: &Semantics<HuskyLangDatabase>,
    ) -> (FxIndexSet<Local>, Option<ast::SelfParam>) {
        todo!()
    }

    fn analyze_container(&self, sema: &Semantics<HuskyLangDatabase>) -> Option<ContainerInfo> {
        todo!()
    }

    fn return_ty(&self, ctx: &AssistContext) -> Option<RetType> {
        todo!()
    }

    /// Local variables defined inside `body` that are accessed outside of it
    fn ret_values<'a>(
        &self,
        ctx: &'a AssistContext,
        parent: &SyntaxNode,
    ) -> impl Iterator<Item = OutlivedLocal> + 'a {
        let parent = parent.clone();
        let range = self.text_range();
        locals_defined_in_body(&ctx.sema, self)
            .into_iter()
            .filter_map(move |local| local_outlives_body(ctx, range, local, &parent))
    }

    /// Analyses the function body for external control flow.
    fn external_control_flow(
        &self,
        ctx: &AssistContext,
        container_info: &ContainerInfo,
    ) -> Option<ControlFlow> {
        todo!()
    }

    /// find variables that should be extracted as params
    ///
    /// Computes additional info that affects param type and mutability
    fn extracted_function_params(
        &self,
        ctx: &AssistContext,
        container_info: &ContainerInfo,
        locals: impl Iterator<Item = Local>,
    ) -> Vec<Param> {
        todo!()
    }

    fn has_usages_after_body(&self, usages: &LocalUsages) -> bool {
        usages
            .iter()
            .any(|reference| self.precedes_range(reference.range))
    }
}

/// checks if relevant var is used with `&mut` access inside body
fn has_exclusive_usages(ctx: &AssistContext, usages: &LocalUsages, body: &FunctionBody) -> bool {
    usages
        .iter()
        .filter(|reference| body.contains_range(reference.range))
        .any(|reference| reference_is_exclusive(reference, body, ctx))
}

/// checks if this reference requires `&mut` access inside node
fn reference_is_exclusive(
    reference: &FileReference,
    node: &dyn HasTokenAtOffset,
    ctx: &AssistContext,
) -> bool {
    // we directly modify variable with set: `n = 0`, `n += 1`
    if reference.category == Some(ReferenceCategory::Write) {
        return true;
    }

    // we take `&mut` reference to variable: `&mut v`
    let path = match path_element_of_reference(node, reference) {
        Some(path) => path,
        None => return false,
    };

    expr_require_exclusive_access(ctx, &path).unwrap_or(false)
}

/// checks if this expr requires `&mut` access, recurses on field access
fn expr_require_exclusive_access(ctx: &AssistContext, expr: &ast::Expr) -> Option<bool> {
    todo!()
}

trait HasTokenAtOffset {
    fn token_at_offset(&self, offset: TextSize) -> TokenAtOffset<SyntaxToken>;
}

impl HasTokenAtOffset for SyntaxNode {
    fn token_at_offset(&self, offset: TextSize) -> TokenAtOffset<SyntaxToken> {
        todo!()
    }
}

impl HasTokenAtOffset for FunctionBody {
    fn token_at_offset(&self, offset: TextSize) -> TokenAtOffset<SyntaxToken> {
        todo!()
    }
}

/// find relevant `ast::Expr` for reference
///
/// # Preconditions
///
/// `node` must cover `reference`, that is `node.text_range().contains_range(reference.range)`
fn path_element_of_reference(
    node: &dyn HasTokenAtOffset,
    reference: &FileReference,
) -> Option<ast::Expr> {
    todo!()
}

/// list local variables defined inside `body`
fn locals_defined_in_body(
    sema: &Semantics<HuskyLangDatabase>,
    body: &FunctionBody,
) -> FxIndexSet<Local> {
    todo!()
}

/// Returns usage details if local variable is used after(outside of) body
fn local_outlives_body(
    ctx: &AssistContext,
    body_range: TextRange,
    local: Local,
    parent: &SyntaxNode,
) -> Option<OutlivedLocal> {
    let usages = LocalUsages::find_local_usages(ctx, local);
    let mut has_mut_usages = false;
    let mut any_outlives = false;
    for usage in usages.iter() {
        if body_range.end() <= usage.range.start() {
            has_mut_usages |= reference_is_exclusive(usage, parent, ctx);
            any_outlives |= true;
            if has_mut_usages {
                break; // no need to check more elements we have all the info we wanted
            }
        }
    }
    if !any_outlives {
        return None;
    }
    Some(OutlivedLocal {
        local,
        mut_usage_outside_body: has_mut_usages,
    })
}

/// checks if the relevant local was defined before(outside of) body
fn is_defined_outside_of_body(
    ctx: &AssistContext,
    body: &FunctionBody,
    src: &hir::InFile<Either<ast::IdentPat, ast::SelfParam>>,
) -> bool {
    todo!()
}

fn either_syntax(value: &Either<ast::IdentPat, ast::SelfParam>) -> &SyntaxNode {
    todo!()
}

/// find where to put extracted function definition
///
/// Function should be put right after returned node
fn node_to_insert_after(body: &FunctionBody, anchor: Anchor) -> Option<SyntaxNode> {
    todo!()
}

fn make_call(ctx: &AssistContext, fun: &Function, indent: IndentLevel) -> String {
    todo!()
}

enum FlowHandler {
    None,
    Try { kind: TryKind },
    If { action: FlowKind },
    IfOption { action: FlowKind },
    MatchOption { none: FlowKind },
    MatchResult { err: FlowKind },
}

impl FlowHandler {
    fn from_ret_ty(fun: &Function, ret_ty: &FunType) -> FlowHandler {
        todo!()
    }

    fn make_call_expr(&self, call_expr: ast::Expr) -> ast::Expr {
        todo!()
    }
}

fn path_expr_from_local(ctx: &AssistContext, var: Local) -> ast::Expr {
    todo!()
}

fn format_function(
    ctx: &AssistContext,
    module: hir::Module,
    fun: &Function,
    old_indent: IndentLevel,
    new_indent: IndentLevel,
) -> String {
    todo!()
}

impl Function {
    fn make_param_list(&self, ctx: &AssistContext, module: hir::Module) -> ast::ParamList {
        todo!()
    }

    fn make_ret_ty(&self, ctx: &AssistContext, module: hir::Module) -> Option<ast::RetType> {
        todo!()
    }
}

impl FunType {
    fn make_ty(&self, ctx: &AssistContext, module: hir::Module) -> ast::Type {
        todo!()
    }
}

fn make_body(
    ctx: &AssistContext,
    old_indent: IndentLevel,
    new_indent: IndentLevel,
    fun: &Function,
) -> ast::BlockExpr {
    todo!()
}

fn map_tail_expr(block: ast::BlockExpr, f: impl FnOnce(ast::Expr) -> ast::Expr) -> ast::BlockExpr {
    todo!()
}

fn with_default_tail_expr(block: ast::BlockExpr, tail_expr: ast::Expr) -> ast::BlockExpr {
    todo!()
}

fn with_tail_expr(block: ast::BlockExpr, tail_expr: ast::Expr) -> ast::BlockExpr {
    todo!()
}

fn format_type(ty: &hir::Type, ctx: &AssistContext, module: hir::Module) -> String {
    todo!()
}

fn make_ty(ty: &hir::Type, ctx: &AssistContext, module: hir::Module) -> ast::Type {
    todo!()
}

fn rewrite_body_segment(
    ctx: &AssistContext,
    params: &[Param],
    handler: &FlowHandler,
    syntax: &SyntaxNode,
) -> SyntaxNode {
    let syntax = fix_param_usages(ctx, params, syntax);
    update_external_control_flow(handler, &syntax);
    syntax
}

/// change all usages to account for added `&`/`&mut` for some params
fn fix_param_usages(ctx: &AssistContext, params: &[Param], syntax: &SyntaxNode) -> SyntaxNode {
    todo!()
}

fn update_external_control_flow(handler: &FlowHandler, syntax: &SyntaxNode) {
    todo!()
}

fn make_rewritten_flow(handler: &FlowHandler, arg_expr: Option<ast::Expr>) -> Option<ast::Expr> {
    todo!()
}
