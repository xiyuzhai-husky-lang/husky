use crate::*;
use husky_accessibility::Accessibility;
use husky_ast::{Ast, AstIdx};
use husky_entity_path::EntityPath;
use husky_vfs::VfsResult;
use husky_word::Identifier;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
pub struct ModuleUsesSheet {
    module_level_uses: HashMap<EntityPath, VfsResult<Vec<EntityTree>>>,
    errors: Vec<(AstIdx, EntityTreeError)>,
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn module_use_sheet(db: &dyn EntityTreeDb, entity_path: EntityPath) -> ModuleUsesSheet {
    todo!()
}

pub struct ModuleUseBuilder<'a> {
    db: &'a dyn EntityTreeDb,
    root: EntityPath,
    aliases: HashMap<EntityPath, VfsResult<Vec<EntityTree>>>,
    modified_modules: HashSet<EntityPath>,
    errors: Vec<(AstIdx, EntityTreeError)>,
}

impl<'a> ModuleUseBuilder<'a> {
    fn build_all(mut self) -> ModuleUsesSheet {
        self.first_build();
        loop {
            self.incr_build();
            if self.modified_modules.len() == 0 {
                return ModuleUsesSheet {
                    module_level_uses: self.aliases,
                    errors: self.errors,
                };
            } else {
                self.modified_modules = Default::default()
            }
        }
    }

    fn first_build(&mut self) {
        todo!()
    }

    fn incr_build(&mut self) {
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
        .top_level_asts_indexed_iter()
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
