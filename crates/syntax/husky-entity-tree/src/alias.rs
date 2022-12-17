use crate::*;
use husky_accessibility::Accessibility;
use husky_ast::{Ast, AstIdx};
use husky_entity_path::EntityPath;
use husky_vfs::VfsResult;
use husky_word::Identifier;
use std::collections::{HashMap, HashSet};

pub struct AliasSheet {
    aliases: HashMap<EntityPath, VfsResult<Vec<Alias>>>,
}

pub struct Alias {
    ident: Identifier,
    ast_idx: AstIdx,
}

fn alias_sheet(db: &dyn EntityTreeDb, crate_path: CratePath) -> AliasSheet {
    todo!()
}

pub struct AliasSheetBuilder<'a> {
    db: &'a dyn EntityTreeDb,
    root: EntityPath,
    aliases: HashMap<EntityPath, VfsResult<Vec<Alias>>>,
    modified_modules: HashSet<EntityPath>,
}

impl<'a> AliasSheetBuilder<'a> {
    fn build_all(mut self) -> AliasSheet {
        loop {
            self.build();
            if self.modified_modules.len() == 0 {
                return AliasSheet {
                    aliases: self.aliases,
                };
            } else {
                self.modified_modules = Default::default()
            }
        }
    }

    fn build(&mut self) {
        todo!()
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn module_level_use_alls(
    db: &dyn EntityTreeDb,
    module: EntityPath,
) -> VfsResult<Vec<UseAll>> {
    let ast_sheet = db.ast_sheet(module).as_ref()?;
    todo!()
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn module_level_use_ones(
    db: &dyn EntityTreeDb,
    module: EntityPath,
) -> VfsResult<Vec<UseOne>> {
    let ast_sheet = db.ast_sheet(module).as_ref()?;
    Ok(ast_sheet
        .top_level_asts()
        .filter_map(|(ast_idx, ast)| match ast {
            Ast::Use { .. } => todo!(),
            _ => None,
        })
        .collect())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseAll {
    pub ast_idx: AstIdx,
    pub parent: EntityPath,
    pub accessibility: Accessibility,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseOne {
    pub ast_idx: AstIdx,
    pub path: EntityPath,
    pub accessibility: Accessibility,
}
