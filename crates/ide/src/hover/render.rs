//! Logic for rendering the different hover messages
use std::fmt::Display;

use either::Either;
use hir::{AsAssocItem, HirDisplay, Semantics};
use ide_db::{
    base_db::SourceDatabase,
    defs::Definition,
    helpers::{
        generated_lints::{CLIPPY_LINTS, DEFAULT_LINTS, FEATURES},
        FamousDefs,
    },
    RootDatabase,
};
use itertools::Itertools;
use stdx::format_to;
use syntax::{ast, Direction, SyntaxToken};

use crate::{
    doc_links::{remove_links, rewrite_links},
    hover::walk_and_push_ty,
    markdown_remove::remove_markdown,
    HoverAction, HoverConfig, HoverResult, Markup,
};

pub(super) fn type_info(
    sema: &Semantics<RootDatabase>,
    config: &HoverConfig,
    expr_or_pat: &Either<ast::Expr, ast::Pat>,
) -> Option<HoverResult> {
    todo!()
}

pub(super) fn try_expr(
    sema: &Semantics<RootDatabase>,
    config: &HoverConfig,
    try_expr: &ast::TryExpr,
) -> Option<HoverResult> {
    todo!()
}

pub(super) fn deref_expr(
    sema: &Semantics<RootDatabase>,
    config: &HoverConfig,
    deref_expr: &ast::PrefixExpr,
) -> Option<HoverResult> {
    todo!()
}

pub(super) fn keyword(
    sema: &Semantics<RootDatabase>,
    config: &HoverConfig,
    token: &SyntaxToken,
) -> Option<HoverResult> {
    todo!()
}

pub(super) fn try_for_lint(attr: &ast::Attr, token: &SyntaxToken) -> Option<HoverResult> {
    todo!()
}

pub(super) fn process_markup(
    db: &RootDatabase,
    def: Definition,
    markup: &Markup,
    config: &HoverConfig,
) -> Markup {
    let markup = markup.as_str();
    let markup = if !config.markdown() {
        remove_markdown(markup)
    } else if config.links_in_hover {
        rewrite_links(db, markup, def)
    } else {
        remove_links(markup)
    };
    Markup::from(markup)
}

fn definition_owner_name(db: &RootDatabase, def: &Definition) -> Option<String> {
    todo!()
}

pub(super) fn path(db: &RootDatabase, module: hir::Module, item_name: Option<String>) -> String {
    todo!()
}

pub(super) fn definition(
    db: &RootDatabase,
    def: Definition,
    famous_defs: Option<&FamousDefs>,
    config: &HoverConfig,
) -> Option<Markup> {
    todo!()
}

fn definition_mod_path(db: &RootDatabase, def: &Definition) -> Option<String> {
    if let Definition::GenericParam(_) = def {
        return None;
    }
    def.module(db)
        .map(|module| path(db, module, definition_owner_name(db, def)))
}

fn markup(docs: Option<String>, desc: String, mod_path: Option<String>) -> Option<Markup> {
    let mut buf = String::new();

    if let Some(mod_path) = mod_path {
        if !mod_path.is_empty() {
            format_to!(buf, "```rust\n{}\n```\n\n", mod_path);
        }
    }
    format_to!(buf, "```rust\n{}\n```", desc);

    if let Some(doc) = docs {
        format_to!(buf, "\n___\n\n{}", doc);
    }
    Some(buf.into())
}

fn builtin(famous_defs: &FamousDefs, builtin: hir::BuiltinType) -> Option<Markup> {
    todo!()
}

fn find_std_module(famous_defs: &FamousDefs, name: &str) -> Option<hir::Module> {
    todo!()
}

fn local(db: &RootDatabase, it: hir::Local) -> Option<Markup> {
    todo!()
}
