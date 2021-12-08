use either::Either;
use hir::{AssocItem, HasVisibility, Module, ModuleDef, Name, PathResolution, ScopeDef};
use husky_lang_db::{
    defs::{Definition, NameRefClass},
    search::SearchScope,
};
use stdx::never;
use syntax::{ast, Direction, SyntaxNode, SyntaxToken};

use crate::{
    assist_context::{AssistContext, Assists},
    AssistId, AssistKind,
};

pub(crate) fn expand_glob_import(acc: &mut Assists, ctx: &AssistContext) -> Option<()> {
    todo!()
}

fn find_parent_and_path(
    star: &SyntaxToken,
) -> Option<(Either<ast::UseTree, ast::UseTreeList>, ast::Path)> {
    todo!()
}

fn def_is_referenced_in(def: Definition, ctx: &AssistContext) -> bool {
    let search_scope = SearchScope::single_file(ctx.file_id());
    def.usages(&ctx.sema).in_scope(search_scope).at_least_one()
}

#[derive(Debug, Clone)]
struct Ref {
    // could be alias
    visible_name: Name,
    def: Definition,
}

impl Ref {
    fn from_scope_def(name: Name, scope_def: ScopeDef) -> Option<Self> {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct Refs(Vec<Ref>);

impl Refs {
    fn used_refs(&self, ctx: &AssistContext) -> Refs {
        todo!()
    }

    fn filter_out_by_defs(&self, defs: Vec<Definition>) -> Refs {
        Refs(
            self.0
                .clone()
                .into_iter()
                .filter(|r| !defs.contains(&r.def))
                .collect(),
        )
    }
}

fn find_refs_in_mod(
    ctx: &AssistContext,
    module: Module,
    visible_from: Option<Module>,
) -> Option<Refs> {
    todo!()
}

fn is_mod_visible_from(ctx: &AssistContext, module: Module, from: Module) -> bool {
    todo!()
}

// looks for name refs in parent use block's siblings
//
// mod bar {
//     mod qux {
//         struct Qux;
//     }
//
//     pub use qux::Qux;
// }
//
// ↓ ---------------
// use foo::*$0;
// use baz::Baz;
// ↑ ---------------
fn find_imported_defs(ctx: &AssistContext, star: SyntaxToken) -> Option<Vec<Definition>> {
    todo!()
}

fn find_names_to_import(
    ctx: &AssistContext,
    refs_in_target: Refs,
    imported_defs: Vec<Definition>,
) -> Vec<Name> {
    let used_refs = refs_in_target
        .used_refs(ctx)
        .filter_out_by_defs(imported_defs);
    used_refs.0.iter().map(|r| r.visible_name.clone()).collect()
}
