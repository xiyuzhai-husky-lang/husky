#![feature(trait_upcasting)]
mod db;
mod error;
mod menu;
mod parser;
mod sections;

pub use self::db::*;
pub use self::error::*;
pub use self::sections::*;

use self::menu::*;
use self::parser::*;
use husky_toml_ast::*;
use husky_vfs::{DiffPath, VfsResult};

#[salsa::jar(db = CorgiConfigAstDb)]
pub struct CorgiConfigAstJar(corgi_config_ast_sheet, corgi_config_ast_menu);

#[derive(Debug, PartialEq, Eq)]
pub struct CorgiConfigAstSheet {
    registry_section: Option<CorgiConfigAstResult<CorgiConfigRegistrySectionAst>>,
}

impl CorgiConfigAstSheet {
    pub fn registry_section(
        &self,
    ) -> Option<CorgiConfigAstResultRef<&CorgiConfigRegistrySectionAst>> {
        self.registry_section.as_ref().map(|s| s.as_ref())
    }
}

#[salsa::tracked(jar = CorgiConfigAstJar, return_ref)]
pub(crate) fn corgi_config_ast_sheet(
    db: &dyn CorgiConfigAstDb,
    path: DiffPath,
) -> VfsResult<Option<CorgiConfigAstSheet>> {
    let Some(toml_ast_sheet) = db.toml_ast_sheet(path)? else {
        return Ok(None)
    };
    Ok(Some(build_corgi_config_ast_sheet(db, toml_ast_sheet)))
}

fn build_corgi_config_ast_sheet<'a>(
    db: &'a dyn CorgiConfigAstDb,
    toml_ast_sheet: &'a TomlAstSheet,
) -> CorgiConfigAstSheet {
    let mut errors = vec![];
    let mut parser =
        CorgiConfigAstParser::new_root(db, toml_ast_sheet, corgi_config_ast_menu(db), &mut errors);
    CorgiConfigAstSheet {
        registry_section: parser.parse_normal_section(),
    }
}
