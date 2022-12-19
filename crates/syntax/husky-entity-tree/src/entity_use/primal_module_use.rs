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
pub(crate) fn module_use_atoms(
    db: &dyn EntityTreeDb,
    module_path: EntityPath,
) -> VfsResult<Vec<EntityUseAtom>> {
    let module_use_expr_sheet = module_use_exprs(db, module_path).as_ref()?;
    let mut use_alls: Vec<EntityUseAtom> = vec![];
    for (ast_idx, accessibility, use_expr) in module_use_expr_sheet.use_exprs() {
        UseAllCollector::new(
            db,
            module_use_expr_sheet,
            *ast_idx,
            *use_expr,
            *accessibility,
            &mut use_alls,
        )
        .collect_all()
    }
    Ok(use_alls)
}

#[test]
fn module_use_atoms_works() {
    DB::expect_test_modules("module_use_ones", |db, entity_path| {
        module_use_atoms(db, entity_path)
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityUseAtom {
    All {
        ast_idx: AstIdx,
        parent: Vec<Identifier>,
        accessibility: Accessibility,
    },
    One {
        ast_idx: AstIdx,
        path: Vec<Identifier>,
        accessibility: Accessibility,
    },
}

pub struct UseAllCollector<'a> {
    db: &'a dyn EntityTreeDb,
    module_use_expr_sheet: &'a EntityUseExprSheet,
    path: Vec<Identifier>,
    use_atoms: &'a mut Vec<EntityUseAtom>,
    ast_idx: AstIdx,
    root: EntityUseExprIdx,
    accessibility: Accessibility,
}

impl<'a> UseAllCollector<'a> {
    fn new(
        db: &'a dyn EntityTreeDb,
        module_use_expr_sheet: &'a EntityUseExprSheet,
        ast_idx: AstIdx,
        root: EntityUseExprIdx,
        accessibility: Accessibility,
        use_atoms: &'a mut Vec<EntityUseAtom>,
    ) -> Self {
        Self {
            db,
            module_use_expr_sheet,
            use_atoms,
            ast_idx,
            root,
            path: vec![],
            accessibility,
        }
    }
    fn collect_all(mut self) {
        self.collect(self.root);
    }

    fn collect(&mut self, use_expr_idx: EntityUseExprIdx) {
        match &self.module_use_expr_sheet[use_expr_idx] {
            EntityUseExpr::All {} => self.use_atoms.push(EntityUseAtom::All {
                ast_idx: self.ast_idx,
                parent: self.path.clone(),
                accessibility: self.accessibility,
            }),
            EntityUseExpr::One { ident } => todo!(),
            EntityUseExpr::ScopeResolution { parent, child } => {
                self.path.push(*parent);
                self.collect(*child)
            }
            EntityUseExpr::Multiple { exprs } => todo!(),
            EntityUseExpr::Err(_) => todo!(),
        }
    }
}
