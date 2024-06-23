use crate::{region::SemStaticMutDepsRegion, static_mut_deps::SemStaticMutDeps};
use husky_entity_path::{path::ItemPath, region::RegionPath};
use husky_sem_expr::{
    helpers::{region::sem_expr_region_from_region_path, visitor::VisitSemExpr},
    stmt::condition::SemCondition,
    SemExprData, SemExprIdx, SemExprMap, SemExprRegionData, SemStmtIdx, SemStmtIdxRange,
    SemStmtMap,
};
use husky_syn_expr::region::SynExprRegionData;
use husky_syn_expr::variable::VariableMap;
use vec_like::OrderedSmallVecSet;

pub(crate) struct SemStaticMutDepsBuilder<'db, 'a, F>
where
    F: Fn(ItemPath) -> &'a SemStaticMutDeps,
{
    db: &'db ::salsa::Db,
    syn_expr_region_data: &'db SynExprRegionData,
    sem_expr_region_data: &'db SemExprRegionData,
    expr_static_mut_deps_table: SemExprMap<SemStaticMutDeps>,
    stmt_static_mut_deps_table: SemStmtMap<SemStaticMutDeps>,
    variable_static_mut_deps_table: VariableMap<SemStaticMutDeps>,
    effective_number_of_merges: usize,
    f: F,
}

/// # constructor
impl<'db, 'a, F> SemStaticMutDepsBuilder<'db, 'a, F>
where
    F: Fn(ItemPath) -> &'a SemStaticMutDeps,
{
    pub(crate) fn new(db: &'db ::salsa::Db, region_path: RegionPath, f: F) -> Option<Self> {
        let sem_expr_region = sem_expr_region_from_region_path(region_path, db)?;
        let syn_expr_region = sem_expr_region.syn_expr_region(db);
        let syn_expr_region_data = syn_expr_region.data(db);
        let sem_expr_region_data = sem_expr_region.data(db);
        Some(Self {
            db,
            syn_expr_region_data,
            sem_expr_region_data,
            expr_static_mut_deps_table: SemExprMap::new(sem_expr_region_data.sem_expr_arena()),
            stmt_static_mut_deps_table: SemStmtMap::new(sem_expr_region_data.sem_stmt_arena()),
            variable_static_mut_deps_table: VariableMap::new_initialized(
                syn_expr_region_data.variable_region(),
                |_, _| Some(Default::default()),
                |_, _| None,
            ),
            f,
            effective_number_of_merges: 0,
        })
    }
}

/// # getters
impl<'db, 'a, F> SemStaticMutDepsBuilder<'db, 'a, F> where F: Fn(ItemPath) -> &'a SemStaticMutDeps {}

/// # actions
impl<'db, 'a, F> SemStaticMutDepsBuilder<'db, 'a, F>
where
    F: Fn(ItemPath) -> &'a SemStaticMutDeps,
{
    pub(crate) fn calc_root(&mut self) -> SemStaticMutDeps {
        let root_body = self.sem_expr_region_data().root_body();
        self.visit_root_body();
        self.expr_static_mut_deps_table[root_body].clone()
    }

    fn calc_expr(&mut self, expr: SemExprIdx) -> SemStaticMutDeps {
        match *expr.data(self.sem_expr_region_data.sem_expr_arena()) {
            SemExprData::Literal(_, _) => todo!(),
            SemExprData::Unit {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
            } => todo!(),
            SemExprData::PrincipalEntityPath {
                path_expr_idx,
                path,
                ty_path_disambiguation,
                ref instantiation,
            } => todo!(),
            SemExprData::MajorItemPathAssocItem {
                parent_expr_idx,
                parent_path,
                colon_colon_regional_token,
                ident_token,
                ref ontology_dispatch,
            } => todo!(),
            SemExprData::TypeAsTraitItem {
                lpar_regional_token_idx,
                ty,
                as_region_token_idx,
                trai,
                rpar_regional_token_idx,
                colon_colon_regional_token_idx,
                ident,
                ident_regional_token_idx,
                ref ontology_dispatch,
            } => todo!(),
            SemExprData::AssocItem {
                parent_expr_idx,
                colon_colon_regional_token_idx,
                ident,
                ident_regional_token_idx,
                ref ontology_dispatch,
            } => todo!(),
            SemExprData::InheritedVariable {
                inherited_variable_idx,
                ..
            } => self.variable_static_mut_deps_table[inherited_variable_idx].clone(),
            SemExprData::CurrentVariable {
                ident,
                regional_token_idx,
                current_variable_idx,
                current_variable_kind,
            } => todo!(),
            SemExprData::FrameVarDecl {
                regional_token_idx,
                ident,
                frame_variable_idx,
                current_variable_kind,
            } => todo!(),
            SemExprData::SelfType(_) => todo!(),
            SemExprData::SelfValue(_) => todo!(),
            SemExprData::Binary {
                lopd,
                opr,
                ref dispatch,
                opr_regional_token_idx,
                ropd,
            } => todo!(),
            SemExprData::Be {
                src,
                be_regional_token_idx,
                target,
            } => todo!(),
            SemExprData::Prefix {
                opr,
                opr_regional_token_idx,
                opd,
            } => todo!(),
            SemExprData::Suffix {
                opd,
                opr,
                opr_regional_token_idx,
            } => todo!(),
            SemExprData::Unveil {
                opd,
                opr_regional_token_idx,
                ref unveil_output_ty_signature,
                unveil_assoc_fn_path,
                return_ty,
            } => todo!(),
            SemExprData::Unwrap {
                opd,
                opr_regional_token_idx,
            } => todo!(),
            SemExprData::FunctionApplication { function, argument } => todo!(),
            SemExprData::FunctionRitchieCall {
                function,
                ritchie_ty_kind,
                ref template_arguments,
                lpar_regional_token_idx,
                ref ritchie_parameter_argument_matches,
                rpar_regional_token_idx,
            } => todo!(),
            SemExprData::Ritchie {
                ritchie_kind_regional_token_idx,
                ritchie_kind,
                lpar_token,
                ref parameter_ty_items,
                rpar_regional_token_idx,
                light_arrow_token,
                return_ty,
            } => todo!(),
            SemExprData::Field {
                self_argument,
                self_ty,
                dot_regional_token_idx,
                ident_token,
                ref dispatch,
            } => todo!(),
            SemExprData::MethodApplication {
                self_argument,
                dot_regional_token_idx,
                ident_token,
                ref template_arguments,
                lpar_regional_token_idx,
                ref items,
                rpar_regional_token_idx,
            } => todo!(),
            SemExprData::MethodRitchieCall {
                self_argument,
                ref instance_dispatch,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let mut deps = self.expr_static_mut_deps_table[self_argument].clone();
                todo!()
            }
            SemExprData::TemplateInstantiation {
                template,
                ref template_arguments,
            } => todo!(),
            SemExprData::At {
                at_regional_token_idx,
                place_label_regional_token,
            } => todo!(),
            SemExprData::Delimitered {
                lpar_regional_token_idx,
                item,
                rpar_regional_token_idx,
            } => todo!(),
            SemExprData::NewTuple {
                lpar_regional_token_idx,
                ref items,
                rpar_regional_token_idx,
            } => todo!(),
            SemExprData::Index {
                owner,
                lbox_regional_token_idx,
                ref index_sem_list_items,
                rbox_regional_token_idx,
                ref index_dynamic_dispatch,
            } => todo!(),
            SemExprData::CompositionWithList {
                owner,
                lbox_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => todo!(),
            SemExprData::NewList {
                lbox_regional_token_idx,
                ref items,
                element_ty,
                rbox_regional_token_idx,
            } => todo!(),
            SemExprData::BoxColonList {
                lbox_regional_token_idx,
                colon_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => todo!(),
            SemExprData::VecFunctor {
                lbox_regional_token_idx,
                rbox_regional_token_idx,
            } => todo!(),
            SemExprData::ArrayFunctor {
                lbox_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => todo!(),
            SemExprData::Block { stmts } => todo!(),
            SemExprData::EmptyHtmlTag {
                empty_html_bra_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => todo!(),
            SemExprData::Closure {
                closure_kind_regional_token_idx,
                lvert_regional_token_idx,
                ref parameter_obelisks,
                rvert_regional_token,
                return_ty,
                body,
            } => todo!(),
            SemExprData::Sorry { regional_token_idx } => todo!(),
            SemExprData::Todo { regional_token_idx } => todo!(),
            SemExprData::Unreachable { regional_token_idx } => todo!(),
            SemExprData::NestedBlock {
                lcurl_regional_token_idx,
                stmts,
                rcurl_regional_token,
            } => todo!(),
        }
    }

    fn finish(self) -> SemStaticMutDepsRegion {
        todo!()
    }
}

impl<'db, 'a, F> VisitSemExpr<'db> for SemStaticMutDepsBuilder<'db, 'a, F>
where
    F: Fn(ItemPath) -> &'a SemStaticMutDeps,
{
    fn db(&self) -> &'db salsa::Db {
        self.db
    }

    fn sem_expr_region_data(&self) -> &'db SemExprRegionData {
        self.sem_expr_region_data
    }

    fn visit_expr(&mut self, expr: SemExprIdx, f: impl FnOnce(&mut Self)) {
        f(self)
    }

    fn visit_expr_inner(&mut self, expr: SemExprIdx) {
        let deps = self.calc_expr(expr);
        let effective_number_of_merges = &mut self.effective_number_of_merges;
        self.expr_static_mut_deps_table
            .insert_new_or_merge(expr, deps, |deps0, deps| {
                if deps0.merge(&deps) {
                    *effective_number_of_merges += 1;
                }
            });
    }

    fn visit_stmts(&mut self, stmts: SemStmtIdxRange, f: impl FnOnce(&mut Self)) {
        f(self)
    }

    fn visit_stmt(&mut self, stmt: SemStmtIdx, f: impl FnOnce(&mut Self)) {
        f(self)
    }

    fn visit_stmt_inner(&mut self, stmt: SemStmtIdx) {
        todo!()
    }

    fn visit_loop(&mut self, stmt: SemStmtIdx, f: impl Fn(&mut Self)) {
        loop {
            let old_number_of_effective_merges = self.effective_number_of_merges;
            f(self);
            if old_number_of_effective_merges == self.effective_number_of_merges {
                break;
            }
        }
    }

    fn visit_condition(&mut self, condition: SemCondition, f: impl FnOnce(&mut Self)) {
        ()
    }

    fn visit_condition_inner(&mut self, condition: SemCondition) {
        ()
    }
}
