#![feature(trait_upcasting)]
mod db;
mod error;
mod menu;
mod sections;
#[cfg(test)]
mod tests;
mod transformer;

pub use self::db::*;
pub use self::error::*;
pub use self::sections::*;

use self::menu::*;
#[cfg(test)]
use self::tests::*;
use self::transformer::*;
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
    let mut errors = vec![];
    let Some(transformer) = CorgiConfigAstTransformer::new_root(
        db,
        path,
        corgi_config_ast_menu(db),
        &mut errors,
    )? else {
        return Ok(None)
    };
    Ok(Some(transform_corgi_config_ast_sheet(transformer)))
}

// todo: change this to trait implementation??
fn transform_corgi_config_ast_sheet<'a, 'b>(
    mut transformer: CorgiConfigAstTransformer<'a, 'b, TomlTable>,
) -> CorgiConfigAstSheet {
    CorgiConfigAstSheet {
        registry_section: transformer.transform_normal_section(),
    }
}
