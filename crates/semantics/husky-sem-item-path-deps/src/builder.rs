use crate::*;
use husky_entity_path::{
    path::{ItemPath, PrincipalEntityPath},
    region::RegionPath,
};
use husky_fly_term::instantiation::FlyInstantiation;
use husky_sem_expr::{
    helpers::region::sem_expr_region_from_region_path, SemExprData, SemExprRegion, SemStmtData,
};
use vec_like::VecSet;

pub(crate) struct SemItemPathDepsBuilder<'db> {
    db: &'db ::salsa::Db,
    item_path: ItemPath,
    item_path_deps: VecSet<ItemPath>,
}

/// # constructor
impl<'db> SemItemPathDepsBuilder<'db> {
    pub(crate) fn new(db: &'db ::salsa::Db, item_path: ItemPath) -> Self {
        Self {
            db,
            item_path,
            item_path_deps: Default::default(),
        }
    }
}

/// # getters

/// # actions
impl<'db> SemItemPathDepsBuilder<'db> {
    pub(crate) fn add_region(&mut self, region_path: RegionPath) -> SemItemPathDepsResult<()> {
        if let Some(expr_region) = sem_expr_region_from_region_path(region_path, self.db) {
            self.add_expr_region(expr_region)?;
        }
        Ok(())
    }

    fn add_expr_region(&mut self, expr_region: SemExprRegion) -> SemItemPathDepsResult<()> {
        let data = expr_region.data(self.db);
        for entry in data.sem_expr_arena().iter() {
            let data = entry.data_result()?;
            self.add_expr(data);
        }
        for entry in data.sem_stmt_arena().iter() {
            let data = entry.data_result()?;
            self.add_stmt(data);
        }
        Ok(())
    }

    fn add_expr(&mut self, expr: &SemExprData) {
        match *expr {
            SemExprData::Literal(_, _) => (),
            SemExprData::Unit { .. } => (),
            SemExprData::PrincipalEntityPath {
                path_expr_idx,
                path,
                ty_path_disambiguation,
                ref instantiation,
            } => {
                match path {
                    PrincipalEntityPath::Module(_) => todo!(),
                    PrincipalEntityPath::MajorItem(path) => self.add_item_path(path),
                    PrincipalEntityPath::TypeVariant(path) => self.add_item_path(path),
                };
                if let Some(instantiation) = instantiation {
                    self.add_instantiation(instantiation)
                }
            }
            SemExprData::MajorItemPathAssocItem {
                ref ontology_dispatch,
                ..
            } => {
                self.add_item_path(ontology_dispatch.path());
                self.add_instantiation(ontology_dispatch.instantiation());
            }
            SemExprData::TypeAsTraitItem {
                ref ontology_dispatch,
                ..
            } => {
                self.add_item_path(ontology_dispatch.path());
                self.add_instantiation(ontology_dispatch.instantiation());
            }
            SemExprData::AssocItem {
                ref ontology_dispatch,
                ..
            } => {
                self.add_item_path(ontology_dispatch.path());
                self.add_instantiation(ontology_dispatch.instantiation());
            }
            SemExprData::InheritedSynSymbol { .. } => (),
            SemExprData::CurrentSynSymbol { .. } => (),
            SemExprData::FrameVarDecl { .. } => (),
            SemExprData::SelfType(_) => (),
            SemExprData::SelfValue(_) => (),
            // ad hoc
            SemExprData::Binary { ref dispatch, .. } => (),
            SemExprData::Be { .. } => (),
            // ad hoc
            SemExprData::Prefix { .. } => (),
            // ad hoc
            SemExprData::Suffix { .. } => (),
            SemExprData::Unveil {
                unveil_assoc_fn_path,
                ..
            } => self.add_item_path(unveil_assoc_fn_path),
            // ad hoc
            SemExprData::Unwrap { .. } => (),
            SemExprData::FunctionApplication { function, argument } => (),
            SemExprData::FunctionRitchieCall { .. } => (),
            SemExprData::Ritchie { .. } => (),
            // ad hoc
            SemExprData::Field { ref dispatch, .. } => (),
            SemExprData::MethodApplication {
                self_argument,
                dot_regional_token_idx,
                ident_token,
                ref template_arguments,
                lpar_regional_token_idx,
                ref items,
                rpar_regional_token_idx,
            } => (),
            SemExprData::MethodRitchieCall {
                ref instance_dispatch,
                ref template_arguments,
                ..
            } => {
                if let Some(template_arguments) = template_arguments {
                    todo!()
                }
                self.add_item_path(instance_dispatch.signature().path());
                self.add_instantiation(instance_dispatch.signature().instantiation());
            }
            SemExprData::TemplateInstantiation {
                template,
                ref template_arguments,
            } => todo!(),
            SemExprData::At { .. } => (),
            SemExprData::Delimitered { .. } => (),
            SemExprData::NewTuple { .. } => (),
            // ad hoc
            SemExprData::Index {
                owner,
                ref index_sem_list_items,
                ref index_dynamic_dispatch,
                ..
            } => (),
            SemExprData::CompositionWithList { .. } => (),
            SemExprData::NewList { .. } => (),
            SemExprData::BoxColonList { .. } => (),
            SemExprData::VecFunctor { .. } => (),
            SemExprData::ArrayFunctor { .. } => (),
            SemExprData::Block { .. } => (),
            SemExprData::EmptyHtmlTag { .. } => (),
            SemExprData::Closure { .. } => (),
            SemExprData::Sorry { .. } => (),
            SemExprData::Todo { .. } => (),
            SemExprData::Unreachable { .. } => (),
            SemExprData::NestedBlock { .. } => (),
        }
    }

    fn add_stmt(&mut self, stmt: &SemStmtData) {
        // ad hoc
        match *stmt {
            SemStmtData::Let { .. } => (),
            SemStmtData::Return { .. } => (),
            SemStmtData::Require { .. } => (),
            SemStmtData::Assert { .. } => (),
            SemStmtData::Break { .. } => (),
            SemStmtData::Eval { .. } => (),
            SemStmtData::ForBetween { .. } => (),
            SemStmtData::ForIn { .. } => (),
            SemStmtData::Forext { .. } => (),
            SemStmtData::While { .. } => (),
            SemStmtData::DoWhile { .. } => (),
            SemStmtData::IfElse { .. } => (),
            SemStmtData::Match { .. } => (),
            SemStmtData::Narrate { .. } => (),
        }
    }

    fn add_item_path(&mut self, path: impl Into<ItemPath>) {
        let path = path.into();
        if path != self.item_path {
            self.item_path_deps.insert(path);
        }
    }

    fn add_instantiation(&mut self, instantiation: &FlyInstantiation) {
        // ad hoc, I'm lazy here. And most likely it wouldn't cause much difference.
        // todo!()
    }

    pub(crate) fn finish(self) -> VecSet<ItemPath> {
        self.item_path_deps
    }
}
