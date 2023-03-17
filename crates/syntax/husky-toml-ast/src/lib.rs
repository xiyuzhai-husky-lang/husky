#![feature(const_trait_impl)]
#![feature(try_trait_v2)]
mod ast;
mod db;
mod error;
mod expr;
mod line_group;
mod parser;
mod section;
mod table;
#[cfg(test)]
mod tests;

pub use self::ast::*;
pub use self::db::*;
pub use self::error::*;
pub use self::expr::*;
pub use self::line_group::*;
pub use self::section::*;
pub use self::table::*;

use husky_toml_token::*;
use husky_vfs::*;
use husky_word::Word;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use parser::TomlAstParser;

#[salsa::jar(db = TomlAstDb)]
pub struct TomlAstJar(package_manifest_toml_ast, TomlSectionTitle);

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TomlAstDb)]
pub struct TomlAstSheet {
    expr_arena: TomlExprArena,
    section_arena: TomlSectionAstSheet,
    line_groups: Vec<TomlGroup>,
    table: TomlTable,
}

#[salsa::tracked(jar = TomlAstJar, return_ref)]
fn package_manifest_toml_ast(db: &dyn TomlAstDb, package: PackagePath) -> VfsResult<TomlAstSheet> {
    Ok(TomlAstSheet::new(
        db,
        db.package_manifest_toml_token_sheet(package).as_ref()?,
    ))
}

impl TomlAstSheet {
    fn new(db: &dyn TomlAstDb, toml_token_text: &TomlTokenSheet) -> Self {
        let mut exprs = TomlExprArena::default();
        let line_groups: Vec<_> = toml_token_text
            .line_groups()
            .map(|tokens| TomlAstParser::new(db, tokens, &mut exprs).parse_line_group())
            .collect();
        let sections = TomlSectionAstSheet::parse_collect(db, &toml_token_text, &line_groups);
        let table = TomlTable::new(db, &sections);
        TomlAstSheet {
            section_arena: sections,
            expr_arena: exprs,
            line_groups,
            table,
        }
    }
}
