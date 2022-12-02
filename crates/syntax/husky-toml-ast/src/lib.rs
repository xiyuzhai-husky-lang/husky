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

use husky_source_path::SourcePath;
use husky_toml_token::*;
use husky_vfs::VfsResult;
use husky_word::Word;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use parser::TomlAstParser;
use section::{TomlSection, TomlSectionIdx, TomlSectionKind, TomlSectionSheet};

#[salsa::jar(db = TomlAstDb)]
pub struct TomlAstJar(toml_ast);

#[derive(Debug, PartialEq, Eq)]
pub struct TomlAst {
    exprs: TomlExprArena,
    sections: TomlSectionSheet,
    line_groups: Vec<TomlLineGroup>,
    table: TomlTable,
}

#[salsa::tracked(jar = TomlAstJar, return_ref)]
fn toml_ast(db: &dyn TomlAstDb, path: SourcePath) -> VfsResult<TomlAst> {
    Ok(TomlAst::new(db, db.toml_token_text(path).as_ref()?))
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
