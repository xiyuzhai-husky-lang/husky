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

pub use db::*;
pub use error::*;
pub use expr::*;
pub use line_group::*;
pub use table::*;

use husky_toml_token::*;
use husky_vfs::*;
use husky_word::Word;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use parser::TomlAstParser;
use section::{TomlSection, TomlSectionIdx, TomlSectionKind, TomlSectionSheet};

#[salsa::jar(db = TomlAstDb)]
pub struct TomlAstJar(package_manifest_ast);

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = TomlAstDb)]
pub struct TomlAst {
    exprs: TomlExprArena,
    sections: TomlSectionSheet,
    line_groups: Vec<TomlGroup>,
    table: TomlTable,
}

#[salsa::tracked(jar = TomlAstJar, return_ref)]
fn package_manifest_ast(db: &dyn TomlAstDb, package: PackagePath) -> VfsResult<TomlAst> {
    Ok(TomlAst::new(
        db,
        db.package_manifest_toml_token_sheet(package).as_ref()?,
    ))
}

impl TomlAst {
    fn new(db: &dyn TomlAstDb, toml_token_text: &TomlTokenSheet) -> Self {
        let mut exprs = TomlExprArena::default();
        let line_groups: Vec<_> = toml_token_text
            .line_groups()
            .map(|tokens| TomlAstParser::new(db, tokens, &mut exprs).parse_line_group())
            .collect();
        let sections = TomlSectionSheet::new(&toml_token_text, &line_groups);
        let table = TomlTable::new(&sections);
        TomlAst {
            sections,
            exprs,
            line_groups,
            table,
        }
    }
}
