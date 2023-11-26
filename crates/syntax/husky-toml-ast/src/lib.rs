#![feature(const_trait_impl)]
#![feature(try_trait_v2)]
mod db;
mod error;
mod expr;
mod line_group;
mod parser;
mod section;
mod table;
#[cfg(test)]
mod tests;
mod transformer;

pub use self::db::*;
pub use self::error::*;
pub use self::expr::*;
pub use self::line_group::*;
pub use self::section::*;
pub use self::table::*;
pub use self::transformer::*;

use husky_coword::Coword;
use husky_toml_token::*;
use husky_vfs::{error::VfsResult, *};
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use parser::TomlAstParser;

#[salsa::jar(db = TomlAstDb)]
pub struct TomlAstJar(toml_ast_sheet_aux);

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = TomlAstDb, jar = TomlAstJar)]
pub struct TomlAstSheet {
    expr_arena: TomlExprArena,
    section_sheet: TomlSectionSheet,
    line_groups: Vec<TomlLineGroup>,
    table: TomlTable,
}

fn toml_ast_sheet(db: &::salsa::Db, path: VirtualPath) -> VfsResult<Option<&TomlAstSheet>> {
    match toml_ast_sheet_aux(db, path) {
        Ok(Some(ast_sheet)) => Ok(Some(ast_sheet)),
        Ok(None) => Ok(None),
        Err(e) => Err(e.clone()),
    }
}

#[salsa::tracked(jar = TomlAstJar, return_ref)]
fn toml_ast_sheet_aux(db: &::salsa::Db, path: VirtualPath) -> VfsResult<Option<TomlAstSheet>> {
    Ok(db
        .toml_token_sheet(path)
        .as_ref()?
        .map(|token_sheet| TomlAstSheet::new(db, token_sheet)))
}

impl TomlAstSheet {
    fn new(db: &::salsa::Db, toml_token_text: &TomlTokenSheet) -> Self {
        let mut exprs = TomlExprArena::default();
        let line_groups: Vec<_> = toml_token_text
            .line_groups()
            .map(|tokens| TomlAstParser::new(db, tokens, &mut exprs).parse_line_group())
            .collect();
        let sections = TomlSectionSheet::parse_collect(db, &toml_token_text, &line_groups);
        let table = TomlTable::new(db, &sections);
        TomlAstSheet {
            section_sheet: sections,
            expr_arena: exprs,
            line_groups,
            table,
        }
    }
}
