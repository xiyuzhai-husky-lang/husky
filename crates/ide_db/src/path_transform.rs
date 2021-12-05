//! See [`PathTransform`].

use crate::helpers::mod_path_to_ast;
use hir::SemanticsScope;
use rustc_hash::FxHashMap;
use syntax::{ast, SyntaxNode};

/// `PathTransform` substitutes path in SyntaxNodes in bulk.
///
/// This is mostly useful for IDE code generation. If you paste some existing
/// code into a new context (for example, to add method overrides to an `impl`
/// block), you generally want to appropriately qualify the names, and sometimes
/// you might want to substitute generic parameters as well:
///
/// ```
/// mod x {
///   pub struct A<V>;
///   pub trait T<U> { fn foo(&self, _: U) -> A<U>; }
/// }
///
/// mod y {
///   use x::T;
///
///   impl T<()> for () {
///      // If we invoke **Add Missing Members** here, we want to copy-paste `foo`.
///      // But we want a slightly-modified version of it:
///      fn foo(&self, _: ()) -> x::A<()> {}
///   }
/// }
/// ```
pub struct PathTransform<'a> {
    generic_def: hir::GenericDef,
    substs: Vec<ast::Type>,
    target_scope: &'a SemanticsScope<'a>,
    source_scope: &'a SemanticsScope<'a>,
}

impl<'a> PathTransform<'a> {
    pub fn trait_impl(
        target_scope: &'a SemanticsScope<'a>,
        source_scope: &'a SemanticsScope<'a>,
        trait_: hir::Trait,
        impl_: ast::Impl,
    ) -> PathTransform<'a> {
        todo!()
    }

    pub fn function_call(
        target_scope: &'a SemanticsScope<'a>,
        source_scope: &'a SemanticsScope<'a>,
        function: hir::Function,
        generic_arg_list: ast::GenericArgList,
    ) -> PathTransform<'a> {
        todo!()
    }

    pub fn apply(&self, syntax: &SyntaxNode) {
        if let Some(ctx) = self.build_ctx() {
            ctx.apply(syntax)
        }
    }

    fn build_ctx(&self) -> Option<Ctx<'a>> {
        todo!()
    }
}

struct Ctx<'a> {
    substs: FxHashMap<hir::TypeParam, ast::Type>,
    target_module: hir::Module,
    source_scope: &'a SemanticsScope<'a>,
}

impl<'a> Ctx<'a> {
    fn apply(&self, item: &SyntaxNode) {
        todo!()
    }
    fn transform_path(&self, path: ast::Path) -> Option<()> {
        todo!()
    }
}

// FIXME: It would probably be nicer if we could get this via HIR (i.e. get the
// trait ref, and then go from the types in the substs back to the syntax).
fn get_syntactic_substs(impl_def: ast::Impl) -> Option<Vec<ast::Type>> {
    todo!()
}

fn get_type_args_from_arg_list(generic_arg_list: ast::GenericArgList) -> Option<Vec<ast::Type>> {
    todo!()
}
