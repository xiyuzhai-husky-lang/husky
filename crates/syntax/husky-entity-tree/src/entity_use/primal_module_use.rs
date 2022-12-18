/// primal module use doesn't include those entities defined in implementations
use crate::*;
use husky_accessibility::Accessibility;
use husky_ast::{Ast, AstIdx};
use husky_entity_path::EntityPath;
use husky_vfs::VfsResult;
use husky_word::Identifier;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
pub struct PrimalModuleUseSheet {
    module_uses_map: HashMap<EntityPath, VfsResult<Vec<EntityTree>>>,
    errors: Vec<(AstIdx, EntityTreeError)>,
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn primal_module_use_sheet(
    db: &dyn EntityTreeDb,
    crate_path: CratePath,
) -> VfsResult<PrimalModuleUseSheet> {
    Ok(PrimalModuleUseBuilder::new(db, crate_path)?.build_all())
}

#[test]
fn primal_module_use_sheet_works() {
    DB::expect_test_crates("primal_module_use_sheet", |db, crate_path| {
        primal_module_use_sheet(db, crate_path)
    })
}

pub struct PrimalModuleUseBuilder<'a> {
    db: &'a dyn EntityTreeDb,
    root: EntityPath,
    module_uses_map: HashMap<EntityPath, VfsResult<Vec<EntityTree>>>,
    modified_modules: HashSet<EntityPath>,
    errors: Vec<(AstIdx, EntityTreeError)>,
}

impl<'a> PrimalModuleUseBuilder<'a> {
    fn new(db: &'a dyn EntityTreeDb, crate_path: CratePath) -> VfsResult<Self> {
        fn collect_initial_module_uses(
            db: &dyn EntityTreeDb,
            crate_path: CratePath,
        ) -> VfsResult<HashMap<EntityPath, VfsResult<Vec<EntityTree>>>> {
            let all_modules = db.all_modules_within_crate(crate_path).as_ref()?;
            todo!()
        }

        let root = db.it_entity_path(EntityPathData::CrateRoot(crate_path));
        Ok(PrimalModuleUseBuilder {
            db,
            root,
            module_uses_map: collect_initial_module_uses(db, crate_path)?,
            modified_modules: Default::default(),
            errors: Default::default(),
        })
    }

    fn build_all(mut self) -> PrimalModuleUseSheet {
        self.first_build();
        loop {
            self.incr_build();
            if self.modified_modules.len() == 0 {
                return PrimalModuleUseSheet {
                    module_uses_map: self.module_uses_map,
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
pub(crate) fn module_use_alls(db: &dyn EntityTreeDb, module: EntityPath) -> VfsResult<Vec<UseAll>> {
    let ast_sheet = db.ast_sheet(module).as_ref()?;
    Ok(ast_sheet
        .top_level_asts_indexed_iter()
        .filter_map(|(ast_idx, ast)| match ast {
            Ast::Use { token_group } => todo!(),
            _ => None,
        })
        .collect())
}

#[test]
fn module_use_alls_works() {
    DB::expect_test_modules("module_use_alls", |db, entity_path| {
        module_use_alls(db, entity_path)
    })
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn module_use_ones(db: &dyn EntityTreeDb, module: EntityPath) -> VfsResult<Vec<UseOne>> {
    let ast_sheet = db.ast_sheet(module).as_ref()?;
    Ok(ast_sheet
        .top_level_asts_indexed_iter()
        .filter_map(|(ast_idx, ast)| match ast {
            Ast::Use { .. } => todo!(),
            _ => None,
        })
        .collect())
}

#[test]
fn module_use_ones_works() {
    DB::expect_test_modules("module_use_ones", |db, entity_path| {
        module_use_ones(db, entity_path)
    })
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
