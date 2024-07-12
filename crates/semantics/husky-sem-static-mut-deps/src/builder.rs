use crate::{
    region::SemStaticMutDepsRegion,
    static_mut_deps::{EffectiveMergeCounter, SemStaticMutDeps},
};
use husky_entity_path::{
    path::{ItemPath, PrincipalEntityPath},
    region::RegionPath,
};
use husky_fly_term::{
    dispatch::field::FieldFlySignature,
    signature::assoc_item::trai_for_ty_item::{
        binary_opr::SemaBinaryOprFlySignature, index::FlyIndexSignature,
    },
};
use husky_sem_expr::{
    helpers::{region::sem_expr_region_from_region_path, visitor::VisitSemExpr},
    obelisks::closure_parameter::ClosureParameterObelisk,
    stmt::condition::SemCondition,
    SemExprData, SemExprIdx, SemExprMap, SemExprRegionData, SemStmtData, SemStmtIdx,
    SemStmtIdxRange, SemStmtMap, SemaRitchieArgument,
};
use husky_syn_expr::variable::{CurrentVariableIdx, VariableMap};
use husky_syn_expr::{region::SynExprRegionData, variable::CurrentVariableIdxRange};
use husky_term_prelude::Contract;
use vec_like::OrderedSmallVecSet;

pub(crate) struct SemStaticMutDepsBuilder<'db, 'a, F>
where
    F: Fn(ItemPath) -> &'a SemStaticMutDeps,
{
    db: &'db ::salsa::Db,
    syn_expr_region_data: &'db SynExprRegionData,
    sem_expr_region_data: &'db SemExprRegionData,
    expr_value_static_mut_deps_table: SemExprMap<SemStaticMutDeps>,
    expr_control_flow_static_mut_deps_table: SemExprMap<SemStaticMutDeps>,
    stmt_value_static_mut_deps_table: SemStmtMap<SemStaticMutDeps>,
    stmt_control_flow_static_mut_deps_table: SemStmtMap<SemStaticMutDeps>,
    self_value_static_mut_deps: SemStaticMutDeps,
    variable_static_mut_deps_table: VariableMap<SemStaticMutDeps>,
    counter: EffectiveMergeCounter,
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
            expr_value_static_mut_deps_table: SemExprMap::new(
                sem_expr_region_data.sem_expr_arena(),
            ),
            expr_control_flow_static_mut_deps_table: SemExprMap::new(
                sem_expr_region_data.sem_expr_arena(),
            ),
            stmt_value_static_mut_deps_table: SemStmtMap::new(
                sem_expr_region_data.sem_stmt_arena(),
            ),
            stmt_control_flow_static_mut_deps_table: SemStmtMap::new(
                sem_expr_region_data.sem_stmt_arena(),
            ),
            self_value_static_mut_deps: SemStaticMutDeps::default(),
            variable_static_mut_deps_table: VariableMap::new_initialized(
                syn_expr_region_data.variable_region(),
                |_, _| Some(Default::default()),
                |_, _| None,
            ),
            f,
            counter: Default::default(),
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
        let mut deps = self.expr_value_static_mut_deps_table[root_body].clone();
        deps.merge(&self.expr_control_flow_static_mut_deps_table[root_body]);
        deps
    }

    fn calc_path(&self, path: impl Into<ItemPath>) -> &'a SemStaticMutDeps {
        (self.f)(path.into())
    }

    fn calc_expr_value(&mut self, expr: SemExprIdx) -> SemStaticMutDeps {
        match *expr.data(self.sem_expr_region_data.sem_expr_arena()) {
            SemExprData::Literal(_, _) | SemExprData::Unit { .. } => Default::default(),
            SemExprData::PrincipalEntityPath { path, .. } => match path {
                PrincipalEntityPath::Module(_) => todo!(),
                PrincipalEntityPath::MajorItem(path) => self.calc_path(path).clone(),
                PrincipalEntityPath::TypeVariant(path) => self.calc_path(path).clone(),
            },
            SemExprData::MajorItemPathAssocItem {
                parent_path,
                ref ontology_dispatch,
                ..
            } => self.calc_path(ontology_dispatch.path()).clone(),
            SemExprData::TypeAsTraitItem {
                ty,
                trai,
                ref ontology_dispatch,
                ..
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
                current_variable_idx,
                ident,
                ..
            } => self.variable_static_mut_deps_table[current_variable_idx].clone(),
            SemExprData::FrameVarDecl {
                for_loop_varible_idx,
                ..
            } => todo!(),
            SemExprData::SelfType(_) => Default::default(),
            SemExprData::SelfValue(_) => self.self_value_static_mut_deps.clone(),
            SemExprData::Binary {
                lopd,
                ref dispatch,
                ropd,
                ..
            } => {
                let mut deps = self.expr_value_static_mut_deps_table[lopd].clone();
                deps.merge(&self.expr_value_static_mut_deps_table[ropd]);
                match dispatch.signature() {
                    SemaBinaryOprFlySignature::Builtin => (),
                }
                deps
            }
            SemExprData::Be {
                src,
                be_regional_token_idx,
                target,
            } => todo!(),
            // todo: handle signature
            SemExprData::Prefix { opr, opd, .. } => {
                self.expr_value_static_mut_deps_table[opd].clone()
            }
            // todo: handle signature
            SemExprData::Suffix { opd, opr, .. } => {
                self.expr_value_static_mut_deps_table[opd].clone()
            }
            SemExprData::Unveil {
                opd,
                ref unveil_output_ty_signature,
                unveil_assoc_fn_path,
                return_ty,
                ..
            } => {
                let mut deps = self.expr_value_static_mut_deps_table[opd].clone();
                let path_deps = self.calc_path(unveil_assoc_fn_path);
                deps.merge(path_deps);
                deps
            }
            SemExprData::Unwrap { opd, .. } => self.expr_value_static_mut_deps_table[opd].clone(),
            SemExprData::FunctionApplication { function, argument } => todo!(),
            SemExprData::FunctionRitchieCall {
                function,
                ritchie_ty_kind,
                ref template_arguments,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let mut deps = SemStaticMutDeps::default();
                deps.merge(&self.expr_value_static_mut_deps_table[function]);
                for m in ritchie_parameter_argument_matches {
                    match m {
                        SemaRitchieArgument::Simple(_, arg) => deps
                            .merge(&self.expr_value_static_mut_deps_table[arg.argument_expr_idx]),
                        SemaRitchieArgument::Variadic(_, args) => {
                            for arg in args {
                                deps.merge(
                                    &self.expr_value_static_mut_deps_table[arg.argument_expr_idx()],
                                );
                            }
                        }
                        // todo: handle default argument???
                        SemaRitchieArgument::Keyed(_, arg) => match arg {
                            Some(arg) => deps.merge(
                                &self.expr_value_static_mut_deps_table[arg.argument_expr_idx()],
                            ),
                            None => (),
                        },
                    }
                }
                for m in ritchie_parameter_argument_matches {
                    match m {
                        SemaRitchieArgument::Simple(param, arg) => {
                            if param.contract == Contract::BorrowMut {
                                self.expr_value_static_mut_deps_table[arg.argument_expr_idx]
                                    .merge(&deps)
                            }
                        }
                        SemaRitchieArgument::Variadic(param, _) => {
                            if param.contract() == Contract::BorrowMut {
                                todo!()
                            }
                        }
                        SemaRitchieArgument::Keyed(param, _) => {
                            if param.contract() == Contract::BorrowMut {
                                todo!()
                            }
                        }
                    }
                }
                deps
            }
            SemExprData::Ritchie { .. } => Default::default(),
            SemExprData::Field {
                self_argument,
                ref dispatch,
                ..
            } => {
                let mut deps = self.expr_value_static_mut_deps_table[self_argument].clone();
                match *dispatch.signature() {
                    FieldFlySignature::PropsStruct { .. } => (),
                    FieldFlySignature::Memoized { path, .. } => {
                        let path_deps = self.calc_path(path);
                        deps.merge(path_deps);
                    }
                }
                deps
            }
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
                self_contract,
                ref instance_dispatch,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let mut deps = self.expr_value_static_mut_deps_table[self_argument].clone();
                deps.merge((self.f)(instance_dispatch.signature().path().into()));
                for m in ritchie_parameter_argument_matches {
                    match m {
                        SemaRitchieArgument::Simple(param, arg) => deps
                            .merge(&self.expr_value_static_mut_deps_table[arg.argument_expr_idx]),
                        SemaRitchieArgument::Variadic(_, _) => todo!(),
                        SemaRitchieArgument::Keyed(_, _) => todo!(),
                    }
                }
                for m in ritchie_parameter_argument_matches {
                    match m {
                        SemaRitchieArgument::Simple(param, arg) => {
                            if param.contract == Contract::BorrowMut {
                                self.expr_value_static_mut_deps_table[arg.argument_expr_idx]
                                    .merge(&deps)
                            }
                        }
                        SemaRitchieArgument::Variadic(_, _) => todo!(),
                        SemaRitchieArgument::Keyed(_, _) => todo!(),
                    }
                }
                if self_contract == Contract::BorrowMut {
                    self.expr_value_static_mut_deps_table[self_argument].merge(&deps);
                }
                deps
            }
            SemExprData::TemplateInstantiation {
                template,
                ref template_arguments,
            } => todo!(),
            SemExprData::At {
                at_regional_token_idx,
                place_label_regional_token,
            } => todo!(),
            SemExprData::Delimitered { item, .. } => {
                self.expr_value_static_mut_deps_table[item].clone()
            }
            SemExprData::NewTuple { ref items, .. } => todo!(),
            SemExprData::Index {
                owner,
                ref index_sem_list_items,
                ref index_dynamic_dispatch,
                ..
            } => {
                let mut deps = self.expr_value_static_mut_deps_table[owner].clone();
                for item in index_sem_list_items {
                    deps.merge(&self.expr_value_static_mut_deps_table[item.sem_expr_idx])
                }
                match index_dynamic_dispatch.signature() {
                    FlyIndexSignature::Int { element_ty } => (),
                    FlyIndexSignature::Regular { element_ty } => (),
                    FlyIndexSignature::Index { element_ty } => (),
                }
                deps
            }
            SemExprData::CompositionWithList {
                owner, ref items, ..
            } => todo!(),
            SemExprData::NewList { ref items, .. } => {
                let mut deps = SemStaticMutDeps::default();
                for item in items {
                    deps.merge(&self.expr_value_static_mut_deps_table[item.sem_expr_idx]);
                }
                deps
            }
            SemExprData::BoxColonList { ref items, .. } => todo!(),
            SemExprData::VecFunctor { .. } => Default::default(),
            SemExprData::ArrayFunctor { .. } => Default::default(),
            SemExprData::Block { stmts } | SemExprData::NestedBlock { stmts, .. } => {
                self.stmt_value_static_mut_deps_table[stmts.last().unwrap()].clone()
            }
            SemExprData::EmptyHtmlTag {
                empty_html_bra_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => {
                let mut deps = SemStaticMutDeps::default();
                for argument in arguments {
                    deps.merge(&self.expr_value_static_mut_deps_table[argument.expr()])
                }
                deps
            }
            // ad hoc
            SemExprData::Closure {
                ref parameter_obelisks,
                body,
                ..
            } => self.expr_value_static_mut_deps_table[body].clone(),
            SemExprData::Sorry { .. }
            | SemExprData::Todo { .. }
            | SemExprData::Unreachable { .. } => Default::default(),
        }
    }

    fn calc_expr_control_flow(&mut self, expr: SemExprIdx) -> SemStaticMutDeps {
        match *expr.data(self.sem_expr_region_data.sem_expr_arena()) {
            SemExprData::Literal(_, _)
            | SemExprData::Unit { .. }
            | SemExprData::PrincipalEntityPath { .. }
            | SemExprData::MajorItemPathAssocItem { .. }
            | SemExprData::TypeAsTraitItem { .. }
            | SemExprData::AssocItem { .. }
            | SemExprData::InheritedVariable { .. }
            | SemExprData::CurrentVariable { .. }
            | SemExprData::FrameVarDecl { .. }
            | SemExprData::SelfType(_)
            | SemExprData::SelfValue(_) => Default::default(),
            SemExprData::Binary {
                lopd,
                ref dispatch,
                ropd,
                ..
            } => {
                let mut deps = self.expr_control_flow_static_mut_deps_table[lopd].clone();
                deps.merge(&self.expr_control_flow_static_mut_deps_table[ropd]);
                deps
            }
            SemExprData::Be {
                src,
                be_regional_token_idx,
                target,
            } => todo!(),
            SemExprData::Prefix { opd, .. } => {
                self.expr_control_flow_static_mut_deps_table[opd].clone()
            }
            SemExprData::Suffix { opd, .. } => {
                self.expr_control_flow_static_mut_deps_table[opd].clone()
            }
            SemExprData::Unveil {
                opd,
                opr_regional_token_idx,
                ref unveil_output_ty_signature,
                unveil_assoc_fn_path,
                return_ty,
            } => {
                let mut deps = self.expr_control_flow_static_mut_deps_table[opd].clone();
                deps.merge(&self.expr_value_static_mut_deps_table[opd]);
                let path_deps = self.calc_path(unveil_assoc_fn_path);
                deps.merge(path_deps);
                deps
            }
            SemExprData::Unwrap { opd, .. } => {
                let mut deps = self.expr_control_flow_static_mut_deps_table[opd].clone();
                deps.merge(&self.expr_value_static_mut_deps_table[opd]);
                deps
            }
            SemExprData::FunctionApplication { function, argument } => todo!(),
            SemExprData::FunctionRitchieCall {
                function,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let mut deps = self.expr_control_flow_static_mut_deps_table[function].clone();
                for m in ritchie_parameter_argument_matches {
                    match m {
                        SemaRitchieArgument::Simple(_, arg) => deps.merge(
                            &self.expr_control_flow_static_mut_deps_table[arg.argument_expr_idx],
                        ),
                        SemaRitchieArgument::Variadic(_, args) => {
                            for arg in args {
                                deps.merge(
                                    &self.expr_control_flow_static_mut_deps_table
                                        [arg.argument_expr_idx()],
                                );
                            }
                        }
                        SemaRitchieArgument::Keyed(_, arg) => match arg {
                            Some(arg) => deps.merge(
                                &self.expr_value_static_mut_deps_table[arg.argument_expr_idx()],
                            ),
                            None => (),
                        },
                    }
                }
                deps
            }
            SemExprData::Ritchie {
                ritchie_kind_regional_token_idx,
                ritchie_kind,
                lpar_token,
                ref parameter_ty_items,
                rpar_regional_token_idx,
                light_arrow_token,
                return_ty,
            } => todo!(),
            SemExprData::Field { self_argument, .. } => {
                self.expr_control_flow_static_mut_deps_table[self_argument].clone()
            }
            SemExprData::MethodApplication {
                self_argument,
                ref template_arguments,
                ref items,
                ..
            } => todo!(),
            SemExprData::MethodRitchieCall {
                self_argument,
                self_contract,
                ref instance_dispatch,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let mut deps = self.expr_control_flow_static_mut_deps_table[self_argument].clone();
                for m in ritchie_parameter_argument_matches {
                    match m {
                        SemaRitchieArgument::Simple(_, arg) => deps.merge(
                            &self.expr_control_flow_static_mut_deps_table[arg.argument_expr_idx],
                        ),
                        SemaRitchieArgument::Variadic(_, _) => todo!(),
                        SemaRitchieArgument::Keyed(_, _) => todo!(),
                    }
                }
                deps
            }
            SemExprData::TemplateInstantiation {
                template,
                ref template_arguments,
            } => todo!(),
            SemExprData::At {
                at_regional_token_idx,
                place_label_regional_token,
            } => todo!(),
            SemExprData::Delimitered { item, .. } => {
                self.expr_control_flow_static_mut_deps_table[item].clone()
            }
            SemExprData::NewTuple {
                lpar_regional_token_idx,
                ref items,
                rpar_regional_token_idx,
            } => todo!(),
            SemExprData::Index {
                owner,
                ref index_sem_list_items,
                ref index_dynamic_dispatch,
                ..
            } => {
                let mut deps = self.expr_control_flow_static_mut_deps_table[owner].clone();
                for item in index_sem_list_items {
                    deps.merge(&self.expr_control_flow_static_mut_deps_table[item.sem_expr_idx])
                }
                match index_dynamic_dispatch.signature() {
                    FlyIndexSignature::Int { element_ty } => (),
                    FlyIndexSignature::Regular { element_ty } => (),
                    FlyIndexSignature::Index { element_ty } => (),
                }
                deps
            }
            SemExprData::CompositionWithList {
                owner,
                lbox_regional_token_idx,
                ref items,
                rbox_regional_token_idx,
            } => todo!(),
            SemExprData::NewList { ref items, .. } => {
                let mut deps = SemStaticMutDeps::default();
                for item in items {
                    deps.merge(&self.expr_control_flow_static_mut_deps_table[item.sem_expr_idx]);
                }
                deps
            }
            SemExprData::BoxColonList { .. }
            | SemExprData::VecFunctor { .. }
            | SemExprData::ArrayFunctor { .. } => Default::default(),
            SemExprData::Block { stmts } | SemExprData::NestedBlock { stmts, .. } => {
                let mut deps = Default::default();
                self.calc_stmts_control_flow(stmts, &mut deps);
                deps
            }
            SemExprData::EmptyHtmlTag {
                empty_html_bra_idx,
                function_ident,
                ref arguments,
                empty_html_ket,
            } => {
                let mut deps = SemStaticMutDeps::default();
                for argument in arguments {
                    deps.merge(&self.expr_value_static_mut_deps_table[argument.expr()])
                }
                deps
            }
            SemExprData::Closure { .. } => Default::default(),
            // ad hoc
            SemExprData::Sorry { regional_token_idx } => Default::default(),
            // ad hoc
            SemExprData::Todo { regional_token_idx } => Default::default(),
            // ad hoc
            SemExprData::Unreachable { regional_token_idx } => Default::default(),
        }
    }

    fn calc_stmt_value(&mut self, stmt: SemStmtIdx) -> SemStaticMutDeps {
        match *stmt.data(self.sem_expr_region_data.sem_stmt_arena()) {
            SemStmtData::Let {
                let_token,
                let_pattern_sem_obelisk,
                contract,
                eq_token,
                initial_value,
                ref coercion_outcome,
            } => {
                let initial_value_deps = &self.expr_value_static_mut_deps_table[initial_value];
                for variable in let_pattern_sem_obelisk.variables() {
                    self.variable_static_mut_deps_table
                        .insert_new_current_or_merge(
                            variable,
                            initial_value_deps.clone(),
                            |deps0, deps| deps0.merge(&deps),
                        );
                }
                Default::default()
            }
            SemStmtData::Return { .. } => Default::default(), // because the type of a return statement is never
            SemStmtData::Require { condition, .. } | SemStmtData::Assert { condition, .. } => {
                self.calc_condition_value(condition)
            } // because it will panic if condition not met
            SemStmtData::Break { break_token } => Default::default(),
            SemStmtData::Eval {
                expr,
                ref outcome,
                eol_semicolon,
            } => {
                let discarded = eol_semicolon.is_some();
                if discarded {
                    Default::default()
                } else {
                    self.expr_value_static_mut_deps_table[expr].clone()
                }
            }
            SemStmtData::ForBetween { .. }
            | SemStmtData::ForIn { .. }
            | SemStmtData::Forext { .. }
            | SemStmtData::While { .. }
            | SemStmtData::DoWhile { .. } => Default::default(),
            SemStmtData::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => {
                let mut deps =
                    self.stmt_value_static_mut_deps_table[if_branch.stmts.last().unwrap()].clone();
                for elif_branch in elif_branches {
                    deps.merge(
                        &self.stmt_value_static_mut_deps_table[elif_branch.stmts.last().unwrap()],
                    );
                }
                if let Some(else_branch) = else_branch {
                    deps.merge(
                        &self.stmt_value_static_mut_deps_table[else_branch.stmts.last().unwrap()],
                    );
                }
                deps
            }
            SemStmtData::Match {
                match_token,
                match_opd,
                match_contract,
                eol_with_token,
                ref case_branches,
            } => {
                let mut deps = self.expr_value_static_mut_deps_table[match_opd].clone();
                for case_branch in case_branches {
                    deps.merge(
                        &self.stmt_value_static_mut_deps_table[case_branch.stmts.last().unwrap()],
                    );
                }
                deps
            }
            // ad hoc
            SemStmtData::Narrate { narrate_token } => Default::default(),
        }
    }

    fn calc_stmt_control_flow(&mut self, stmt: SemStmtIdx) -> SemStaticMutDeps {
        match *stmt.data(self.sem_expr_region_data.sem_stmt_arena()) {
            SemStmtData::Let { initial_value, .. } => {
                self.expr_control_flow_static_mut_deps_table[initial_value].clone()
            }
            SemStmtData::Return { result, .. } => {
                let mut deps = self.expr_control_flow_static_mut_deps_table[result].clone();
                deps.merge(&self.expr_value_static_mut_deps_table[result]);
                deps
            }
            SemStmtData::Require { condition, .. } => {
                // todo: consider deps of Default::default
                let mut deps = self.calc_condition_control_flow(condition);
                deps.merge(&self.calc_condition_value(condition));
                deps
            }
            SemStmtData::Assert { condition, .. } => {
                let mut deps = self.calc_condition_control_flow(condition);
                deps.merge(&self.calc_condition_value(condition));
                deps
            }
            SemStmtData::Break { break_token } => Default::default(),
            SemStmtData::Eval { expr, .. } => {
                self.expr_control_flow_static_mut_deps_table[expr].clone()
            }
            SemStmtData::ForBetween {
                ref particulars,
                stmts,
                ..
            } => {
                let mut deps = SemStaticMutDeps::default();
                if let Some(bound_expr) = particulars.range().initial_boundary.bound_expr {
                    self.expr_control_flow_static_mut_deps_table[bound_expr].clone();
                }
                if let Some(bound_expr) = particulars.range().final_boundary.bound_expr {
                    self.expr_control_flow_static_mut_deps_table[bound_expr].clone();
                }
                for stmt in stmts {
                    deps.merge(&self.stmt_control_flow_static_mut_deps_table[stmt]);
                }
                deps
            }
            SemStmtData::ForIn {
                for_token,
                range,
                eol_colon,
                stmts,
            } => {
                let mut deps = SemStaticMutDeps::default();
                todo!();
                // if let Some(bound_expr) = t
                // self.expr_control_flow_static_mut_deps_table[bound_expr].clone();
                for stmt in stmts {
                    deps.merge(&self.stmt_control_flow_static_mut_deps_table[stmt]);
                }
                deps
            }
            SemStmtData::Forext {
                ref particulars,
                stmts,
                ..
            } => {
                let mut deps =
                    self.expr_control_flow_static_mut_deps_table[particulars.bound_expr].clone();
                for stmt in stmts {
                    deps.merge(&self.stmt_control_flow_static_mut_deps_table[stmt]);
                }
                deps
            }
            SemStmtData::While {
                condition, stmts, ..
            }
            | SemStmtData::DoWhile {
                condition, stmts, ..
            } => {
                let mut deps = self.calc_condition_control_flow(condition);
                for stmt in stmts {
                    deps.merge(&self.stmt_control_flow_static_mut_deps_table[stmt]);
                }
                deps
            }
            SemStmtData::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => {
                let mut deps = SemStaticMutDeps::default();
                let mut t = |stmts| {
                    self.calc_stmts_control_flow(stmts, &mut deps);
                };
                t(if_branch.stmts);
                for elif_branch in elif_branches {
                    t(elif_branch.stmts)
                }
                if let Some(else_branch) = else_branch {
                    t(else_branch.stmts)
                }
                deps
            }
            SemStmtData::Match {
                match_token,
                match_opd,
                match_contract,
                eol_with_token,
                ref case_branches,
            } => {
                let mut deps = self.expr_control_flow_static_mut_deps_table[match_opd].clone();
                for case_branch in case_branches {
                    self.calc_stmts_control_flow(case_branch.stmts, &mut deps)
                }
                deps
            }
            // ad hoc
            SemStmtData::Narrate { .. } => Default::default(),
        }
    }

    fn calc_stmts_control_flow(&mut self, stmts: SemStmtIdxRange, deps: &mut SemStaticMutDeps) {
        for stmt in stmts {
            deps.merge(&self.stmt_control_flow_static_mut_deps_table[stmt]);
        }
    }

    fn calc_condition_value(&mut self, condition: SemCondition) -> SemStaticMutDeps {
        match condition {
            SemCondition::Be {
                src,
                be_regional_token_idx,
                target,
            } => {
                let deps = self.expr_control_flow_static_mut_deps_table[src].clone();
                self.populate_into_current_variables(target.variables(), &deps);
                deps
            }
            SemCondition::Other {
                sem_expr_idx: src, ..
            } => self.expr_control_flow_static_mut_deps_table[src].clone(),
        }
    }

    fn calc_condition_control_flow(&mut self, condition: SemCondition) -> SemStaticMutDeps {
        match condition {
            SemCondition::Be { src, .. } => {
                self.expr_control_flow_static_mut_deps_table[src].clone()
            }
            SemCondition::Other {
                sem_expr_idx: src, ..
            } => self.expr_control_flow_static_mut_deps_table[src].clone(),
        }
    }

    fn populate_into_current_variables(
        &mut self,
        variables: CurrentVariableIdxRange,
        deps: &SemStaticMutDeps,
    ) {
        for variable in variables {
            self.variable_static_mut_deps_table
                .insert_new_current_or_merge(variable, deps.clone(), |deps0, deps| {
                    SemStaticMutDeps::merge(deps0, &deps)
                })
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

    fn visit_expr_itself(&mut self, expr: SemExprIdx) {
        let deps = self.calc_expr_value(expr);
        self.expr_value_static_mut_deps_table
            .insert_new_or_merge(expr, deps, |deps0, deps| {
                deps0.merge_counted(&deps, &mut self.counter)
            });
        let deps = self.calc_expr_control_flow(expr);
        self.expr_control_flow_static_mut_deps_table
            .insert_new_or_merge(expr, deps, |deps0, deps| {
                deps0.merge_counted(&deps, &mut self.counter)
            });
    }

    fn visit_closure_inner(
        &mut self,
        expr: SemExprIdx,
        parameters: &[ClosureParameterObelisk],
        f: impl FnOnce(&mut Self),
    ) {
        for parameter in parameters {
            self.populate_into_current_variables(parameter.variables(), &Default::default())
        }
        f(self)
    }

    fn visit_stmts(&mut self, stmts: SemStmtIdxRange, f: impl FnOnce(&mut Self)) {
        f(self)
    }

    fn visit_stmt(&mut self, stmt: SemStmtIdx, f: impl FnOnce(&mut Self)) {
        f(self)
    }

    fn visit_for_loop_stmt_inner(
        &mut self,
        stmt: SemStmtIdx,
        current_variable_idx: CurrentVariableIdx,
        f: impl Fn(&mut Self),
    ) {
        self.variable_static_mut_deps_table
            .insert_new_current(current_variable_idx, Default::default());
        f(self)
    }

    fn visit_stmt_itself(&mut self, stmt: SemStmtIdx) {
        let deps = self.calc_stmt_value(stmt);
        self.stmt_value_static_mut_deps_table
            .insert_new_or_merge(stmt, deps, |deps0, deps| {
                deps0.merge_counted(&deps, &mut self.counter)
            });
        let deps = self.calc_stmt_control_flow(stmt);
        self.stmt_control_flow_static_mut_deps_table
            .insert_new_or_merge(stmt, deps, |deps0, deps| {
                deps0.merge_counted(&deps, &mut self.counter)
            });
    }

    fn visit_loop(&mut self, stmt: SemStmtIdx, f: impl Fn(&mut Self)) {
        loop {
            let old_count = self.counter.count();
            f(self);
            if old_count == self.counter.count() {
                break;
            }
        }
    }

    fn visit_branches(&mut self, f: impl Fn(&mut Self)) {
        f(self)
    }

    fn visit_branch(&mut self, f: impl Fn(&mut Self)) {
        f(self)
    }

    fn visit_condition(&mut self, condition: SemCondition, f: impl FnOnce(&mut Self)) {
        f(self)
    }

    fn visit_condition_inner(&mut self, condition: SemCondition) {
        ()
    }
}
