//! This module contains functions to suggest names for expressions, functions and other items

use hir::Semantics;
use ide_db::RootDatabase;
use itertools::Itertools;
use stdx::to_lower_snake_case;
use syntax::{ast, SmolStr};

/// Trait names, that will be ignored when in `impl Trait` and `dyn Trait`
const USELESS_TRAITS: &[&str] = &["Send", "Sync", "Copy", "Clone", "Eq", "PartialEq"];

/// Identifier names that won't be suggested, ever
///
/// **NOTE**: they all must be snake lower case
const USELESS_NAMES: &[&str] = &[
    "new", "default", "option", "some", "none", "ok", "err", "str", "string",
];

/// Generic types replaced by their first argument
///
/// # Examples
/// `Option<Name>` -> `Name`
/// `Result<User, Error>` -> `User`
const WRAPPER_TYPES: &[&str] = &["Box", "Option", "Result"];

/// Prefixes to strip from methods names
///
/// # Examples
/// `vec.as_slice()` -> `slice`
/// `args.into_config()` -> `config`
/// `bytes.to_vec()` -> `vec`
const USELESS_METHOD_PREFIXES: &[&str] = &["into_", "as_", "to_"];

/// Useless methods that are stripped from expression
///
/// # Examples
/// `var.name().to_string()` -> `var.name()`
const USELESS_METHODS: &[&str] = &[
    "to_string",
    "as_str",
    "to_owned",
    "as_ref",
    "clone",
    "cloned",
    "expect",
    "expect_none",
    "unwrap",
    "unwrap_none",
    "unwrap_or",
    "unwrap_or_default",
    "unwrap_or_else",
    "unwrap_unchecked",
    "iter",
    "into_iter",
    "iter_mut",
];

pub(crate) fn for_generic_parameter(ty: &ast::ImplTraitType) -> SmolStr {
    todo!()
}

pub(crate) fn for_variable(expr: &ast::Expr, sema: &Semantics<'_, RootDatabase>) -> String {
    todo!()
}

fn normalize(name: &str) -> Option<String> {
    let name = to_lower_snake_case(name);

    if USELESS_NAMES.contains(&name.as_str()) {
        return None;
    }

    if !is_valid_name(&name) {
        return None;
    }

    Some(name)
}

fn is_valid_name(name: &str) -> bool {
    todo!()
}

fn is_useless_method(method: &ast::MethodCallExpr) -> bool {
    todo!()
}

fn from_call(expr: &ast::Expr) -> Option<String> {
    from_func_call(expr).or_else(|| from_method_call(expr))
}

fn from_func_call(expr: &ast::Expr) -> Option<String> {
    todo!()
}

fn from_method_call(expr: &ast::Expr) -> Option<String> {
    todo!()
}

fn from_param(expr: &ast::Expr, sema: &Semantics<'_, RootDatabase>) -> Option<String> {
    todo!()
}

fn var_name_from_pat(pat: &ast::Pat) -> Option<ast::Name> {
    todo!()
}

fn from_type(expr: &ast::Expr, sema: &Semantics<'_, RootDatabase>) -> Option<String> {
    todo!()
}

fn name_of_type(ty: &hir::Type, db: &RootDatabase) -> Option<String> {
    todo!()
}

fn trait_name(trait_: &hir::Trait, db: &RootDatabase) -> Option<String> {
    todo!()
}
