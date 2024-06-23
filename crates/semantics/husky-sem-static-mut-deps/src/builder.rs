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
            variable_static_mut_deps_table: VariableMap::new(
                syn_expr_region_data.variable_region(),
            ),
            f,
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
        // todo: handle error
        match expr.data(self.sem_expr_region_data.sem_expr_arena()) {
            SemExprData::Literal(_, _) => todo!(),
            SemExprData::Unit {
                lpar_regional_token_idx,
                rpar_regional_token_idx,
            } => todo!(),
            SemExprData::PrincipalEntityPath {
                path_expr_idx,
                path,
                ty_path_disambiguation,
                instantiation,
            } => todo!(),
            SemExprData::MajorItemPathAssocItem {
                parent_expr_idx,
                parent_path,
                colon_colon_regional_token,
                ident_token,
                ontology_dispatch,
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
                ontology_dispatch,
            } => todo!(),
            SemExprData::AssocItem {
                parent_expr_idx,
                colon_colon_regional_token_idx,
                ident,
                ident_regional_token_idx,
                ontology_dispatch,
            } => todo!(),
            SemExprData::InheritedSynSymbol {
                ident,
                regional_token_idx,
                inherited_syn_symbol_idx,
                inherited_syn_symbol_kind,
            } => todo!(),
            SemExprData::CurrentSynSymbol {
                ident,
                regional_token_idx,
                current_variable_idx,
                current_variable_kind,
            } => todo!(),
            SemExprData::FrameVarDecl {
                regional_token_idx,
                ident,
                frame_var_symbol_idx,
                current_variable_kind,
            } => todo!(),
            SemExprData::SelfType(_) => todo!(),
            SemExprData::SelfValue(_) => todo!(),
            SemExprData::Binary {
                lopd,
                opr,
                dispatch,
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
                unveil_output_ty_signature,
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
                template_arguments,
                lpar_regional_token_idx,
                ritchie_parameter_argument_matches,
                rpar_regional_token_idx,
            } => todo!(),
            SemExprData::Ritchie {
                ritchie_kind_regional_token_idx,
                ritchie_kind,
                lpar_token,
                parameter_ty_items,
                rpar_regional_token_idx,
                light_arrow_token,
                return_ty,
            } => todo!(),
            SemExprData::Field {
                self_argument,
                self_ty,
                dot_regional_token_idx,
                ident_token,
                dispatch,
            } => todo!(),
            SemExprData::MethodApplication {
                self_argument,
                dot_regional_token_idx,
                ident_token,
                template_arguments,
                lpar_regional_token_idx,
                items,
                rpar_regional_token_idx,
            } => todo!(),
            SemExprData::MethodRitchieCall {
                self_argument,
                self_contract,
                dot_regional_token_idx,
                ident_token,
                instance_dispatch,
                template_arguments,
                lpar_regional_token_idx,
                ritchie_parameter_argument_matches,
                rpar_regional_token_idx,
            } => todo!(),
            SemExprData::TemplateInstantiation {
                template,
                template_arguments,
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
                items,
                rpar_regional_token_idx,
            } => todo!(),
            SemExprData::Index {
                owner,
                lbox_regional_token_idx,
                index_sem_list_items,
                rbox_regional_token_idx,
                index_dynamic_dispatch,
            } => todo!(),
            SemExprData::CompositionWithList {
                owner,
                lbox_regional_token_idx,
                items,
                rbox_regional_token_idx,
            } => todo!(),
            SemExprData::NewList {
                lbox_regional_token_idx,
                items,
                element_ty,
                rbox_regional_token_idx,
            } => todo!(),
            SemExprData::BoxColonList {
                lbox_regional_token_idx,
                colon_regional_token_idx,
                items,
                rbox_regional_token_idx,
            } => todo!(),
            SemExprData::VecFunctor {
                lbox_regional_token_idx,
                rbox_regional_token_idx,
            } => todo!(),
            SemExprData::ArrayFunctor {
                lbox_regional_token_idx,
                items,
                rbox_regional_token_idx,
            } => todo!(),
            SemExprData::Block { stmts } => todo!(),
            SemExprData::EmptyHtmlTag {
                empty_html_bra_idx,
                function_ident,
                arguments,
                empty_html_ket,
            } => todo!(),
            SemExprData::Closure {
                closure_kind_regional_token_idx,
                lvert_regional_token_idx,
                parameter_obelisks,
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
        self.expr_static_mut_deps_table
            .insert_new_or_merge(expr, deps, |deps0, deps| deps0.merge(&deps));
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

    fn visit_condition(&mut self, condition: SemCondition, f: impl FnOnce(&mut Self)) {
        ()
    }

    fn visit_condition_inner(&mut self, condition: SemCondition) {
        ()
    }
}
