use crate::{
    region::ItemDefnSemVarDepsRegion,
    var_deps::{
        control_flow::SemControlFlowVarDeps,
        control_transfer::SemControlTransferVarDeps,
        domain::{SemDomainVarDeps, SemDomainVarDepsGuard},
        value::SemValueVarDeps,
        EffectiveMergeCounter,
    },
};
use husky_entity_path::{
    menu::{item_path_menu, ItemPathMenu},
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
    helpers::{path::sem_expr_region_from_region_path, visitor::VisitSemExpr},
    obelisks::closure_parameter::ClosureParameterObelisk,
    stmt::condition::SemCondition,
    SemExprData, SemExprIdx, SemExprMap, SemExprRegionData, SemRitchieArgument, SemStmtData,
    SemStmtIdx, SemStmtIdxRange, SemStmtMap,
};
use husky_syn_expr::variable::{CurrentVariableIdx, VariableMap};
use husky_syn_expr::{region::SynExprRegionData, variable::CurrentVariableIdxRange};
use husky_term_prelude::Contract;
use vec_like::OrderedSmallVecSet;

pub(crate) struct SemVarDepsBuilder<'db, 'a, F>
where
    F: Fn(ItemPath) -> &'a SemValueVarDeps,
{
    db: &'db ::salsa::Db,
    item_path_menu: &'db ItemPathMenu,
    syn_expr_region_data: &'db SynExprRegionData,
    sem_expr_region_data: &'db SemExprRegionData,
    expr_value_var_deps_table: SemExprMap<SemValueVarDeps>,
    expr_control_transfer_var_deps_table: SemExprMap<SemControlTransferVarDeps>,
    expr_domain_var_deps_table: SemExprMap<SemDomainVarDeps>,
    expr_control_flow_var_deps_table: SemExprMap<SemControlFlowVarDeps>,
    stmt_value_var_deps_table: SemStmtMap<SemValueVarDeps>,
    stmt_control_transfer_var_deps_table: SemStmtMap<SemControlTransferVarDeps>,
    stmt_domain_var_deps_table: SemStmtMap<SemDomainVarDeps>,
    stmt_control_flow_var_deps_table: SemStmtMap<SemControlFlowVarDeps>,
    self_value_var_deps: SemValueVarDeps,
    variable_var_deps_table: VariableMap<SemValueVarDeps>,
    counter: EffectiveMergeCounter,
    domain_guard: SemDomainVarDepsGuard,
    f: F,
}

/// # constructor
impl<'db, 'a, F> SemVarDepsBuilder<'db, 'a, F>
where
    F: Fn(ItemPath) -> &'a SemValueVarDeps,
{
    pub(crate) fn new(db: &'db ::salsa::Db, region_path: RegionPath, f: F) -> Option<Self> {
        let sem_expr_region = sem_expr_region_from_region_path(region_path, db)?;
        let syn_expr_region = sem_expr_region.syn_expr_region(db);
        let syn_expr_region_data = syn_expr_region.data(db);
        let sem_expr_region_data = sem_expr_region.data(db);
        Some(Self {
            db,
            item_path_menu: item_path_menu(db, region_path.toolchain(db)),
            syn_expr_region_data,
            sem_expr_region_data,
            expr_value_var_deps_table: SemExprMap::new(sem_expr_region_data.sem_expr_arena()),
            expr_control_transfer_var_deps_table: SemExprMap::new(
                sem_expr_region_data.sem_expr_arena(),
            ),
            expr_domain_var_deps_table: SemExprMap::new(sem_expr_region_data.sem_expr_arena()),
            expr_control_flow_var_deps_table: SemExprMap::new(
                sem_expr_region_data.sem_expr_arena(),
            ),
            stmt_value_var_deps_table: SemStmtMap::new(sem_expr_region_data.sem_stmt_arena()),
            stmt_control_transfer_var_deps_table: SemStmtMap::new(
                sem_expr_region_data.sem_stmt_arena(),
            ),
            stmt_domain_var_deps_table: SemStmtMap::new(sem_expr_region_data.sem_stmt_arena()),
            stmt_control_flow_var_deps_table: SemStmtMap::new(
                sem_expr_region_data.sem_stmt_arena(),
            ),
            self_value_var_deps: SemValueVarDeps::default(),
            variable_var_deps_table: VariableMap::new_initialized(
                syn_expr_region_data.variable_region(),
                |_, _| Some(Default::default()),
                |_, _| None,
            ),
            f,
            counter: Default::default(),
            domain_guard: Default::default(),
        })
    }
}

/// # getters
impl<'db, 'a, F> SemVarDepsBuilder<'db, 'a, F> where F: Fn(ItemPath) -> &'a SemValueVarDeps {}

/// # actions
impl<'db, 'a, F> SemVarDepsBuilder<'db, 'a, F>
where
    F: Fn(ItemPath) -> &'a SemValueVarDeps,
{
    pub(crate) fn calc_root(&mut self) -> SemValueVarDeps {
        let root_body = self.sem_expr_region_data().root_body();
        self.visit_root_body();
        let mut deps = self.expr_value_var_deps_table[root_body].clone();
        deps.merge(&self.expr_control_transfer_var_deps_table[root_body]);
        deps
    }

    fn calc_path(&self, path: impl Into<ItemPath>) -> SemValueVarDeps {
        /// todo: make this expr dependent, because of possible overrides
        let mut deps = (self.f)(path.into()).clone();
        let db = self.db;
        // todo: we can make this things much more general, not just task ty
        match self.sem_expr_region_data.context_itd().task_ty(db) {
            Some(_) => {
                deps.remove_item_path(self.item_path_menu.task_ty_var_path());
                deps.remove_item_path(self.item_path_menu.task_static_var_path());
            }
            None => (),
        }
        deps
    }

    // todo: move this out of this module
    fn calc_expr_value(&mut self, expr: SemExprIdx) -> SemValueVarDeps {
        match *expr.data(self.sem_expr_region_data.sem_expr_arena()) {
            SemExprData::Literal(_, _) | SemExprData::Unit { .. } => Default::default(),
            SemExprData::PrincipalEntityPath { path, .. } => match path {
                PrincipalEntityPath::Module(_) => todo!(),
                PrincipalEntityPath::MajorItem(path) => self.calc_path(path),
                PrincipalEntityPath::TypeVariant(path) => self.calc_path(path),
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
            } => self.variable_var_deps_table[inherited_variable_idx].clone(),
            SemExprData::CurrentVariable {
                current_variable_idx,
                ident,
                ..
            } => self.variable_var_deps_table[current_variable_idx].clone(),
            SemExprData::FrameVarDecl {
                for_loop_varible_idx,
                ..
            } => Default::default(),
            SemExprData::SelfType(_) => Default::default(),
            SemExprData::SelfValue(_) => self.self_value_var_deps.clone(),
            SemExprData::Binary {
                lopd,
                ref dispatch,
                ropd,
                ..
            } => {
                let mut deps = self.expr_value_var_deps_table[lopd].clone();
                deps.merge(&self.expr_value_var_deps_table[ropd]);
                match dispatch.signature() {
                    SemaBinaryOprFlySignature::Builtin => (),
                }
                deps
            }
            SemExprData::Be {
                src,
                contract,
                be_regional_token_idx,
                target,
            } => todo!(),
            // todo: handle signature
            SemExprData::Prefix { opr, opd, .. } => self.expr_value_var_deps_table[opd].clone(),
            // todo: handle signature
            SemExprData::Suffix { opd, opr, .. } => self.expr_value_var_deps_table[opd].clone(),
            SemExprData::Unveil {
                opd,
                ref unveil_output_ty_signature,
                unveil_assoc_fn_path,
                return_ty,
                ..
            } => {
                let mut deps = self.expr_value_var_deps_table[opd].clone();
                let path_deps = self.calc_path(unveil_assoc_fn_path);
                deps.merge(&path_deps);
                deps
            }
            SemExprData::Unwrap { opd, .. } => self.expr_value_var_deps_table[opd].clone(),
            SemExprData::FunctionApplication { function, argument } => todo!(),
            SemExprData::FunctionRitchieCall {
                function,
                ritchie_ty_kind,
                ref template_arguments,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let mut deps = SemValueVarDeps::default();
                deps.merge(&self.expr_value_var_deps_table[function]);
                for m in ritchie_parameter_argument_matches {
                    match m {
                        SemRitchieArgument::Simple(_, arg) => {
                            deps.merge(&self.expr_value_var_deps_table[arg.argument_expr_idx])
                        }
                        SemRitchieArgument::Variadic(_, args) => {
                            for arg in args {
                                deps.merge(
                                    &self.expr_value_var_deps_table[arg.argument_expr_idx()],
                                );
                            }
                        }
                        // todo: handle default argument???
                        SemRitchieArgument::Keyed(_, arg) => match arg {
                            Some(arg) => {
                                deps.merge(&self.expr_value_var_deps_table[arg.argument_expr_idx()])
                            }
                            None => (),
                        },
                    }
                }
                for m in ritchie_parameter_argument_matches {
                    match m {
                        SemRitchieArgument::Simple(param, arg) => {
                            if param.contract == Contract::BorrowMut {
                                self.expr_value_var_deps_table[arg.argument_expr_idx].merge(&deps)
                            }
                        }
                        SemRitchieArgument::Variadic(param, _) => {
                            if param.contract() == Contract::BorrowMut {
                                todo!()
                            }
                        }
                        SemRitchieArgument::Keyed(param, _) => {
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
                let mut deps = self.expr_value_var_deps_table[self_argument].clone();
                match *dispatch.signature() {
                    FieldFlySignature::PropsStruct { .. } => (),
                    FieldFlySignature::Memoized { path, .. } => {
                        let path_deps = self.calc_path(path);
                        deps.merge(&path_deps);
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
                dispatch: ref instance_dispatch,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let mut deps = self.expr_value_var_deps_table[self_argument].clone();
                deps.merge((self.f)(instance_dispatch.signature().path().into()));
                for m in ritchie_parameter_argument_matches {
                    match m {
                        SemRitchieArgument::Simple(param, arg) => {
                            deps.merge(&self.expr_value_var_deps_table[arg.argument_expr_idx])
                        }
                        SemRitchieArgument::Variadic(_, _) => todo!(),
                        SemRitchieArgument::Keyed(_, _) => todo!(),
                    }
                }
                for m in ritchie_parameter_argument_matches {
                    match m {
                        SemRitchieArgument::Simple(param, arg) => {
                            if param.contract == Contract::BorrowMut {
                                self.expr_value_var_deps_table[arg.argument_expr_idx].merge(&deps)
                            }
                        }
                        SemRitchieArgument::Variadic(_, _) => todo!(),
                        SemRitchieArgument::Keyed(_, _) => todo!(),
                    }
                }
                if self_contract == Contract::BorrowMut {
                    self.expr_value_var_deps_table[self_argument].merge(&deps);
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
            SemExprData::Delimitered { item, .. } => self.expr_value_var_deps_table[item].clone(),
            SemExprData::NewTuple { ref items, .. } => todo!(),
            SemExprData::Index {
                self_argument: owner,
                items: ref index_sem_list_items,
                ref index_dynamic_dispatch,
                ..
            } => {
                let mut deps = self.expr_value_var_deps_table[owner].clone();
                for item in index_sem_list_items {
                    deps.merge(&self.expr_value_var_deps_table[item.expr])
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
                let mut deps = SemValueVarDeps::default();
                for item in items {
                    deps.merge(&self.expr_value_var_deps_table[item.expr]);
                }
                deps
            }
            SemExprData::BoxColonList { ref items, .. } => todo!(),
            SemExprData::VecFunctor { .. } => Default::default(),
            SemExprData::ArrayFunctor { .. } => Default::default(),
            SemExprData::Block { stmts } | SemExprData::NestedBlock { stmts, .. } => {
                self.stmt_value_var_deps_table[stmts.last().unwrap()].clone()
            }
            SemExprData::EmptyHtmxTag {
                empty_htmx_bra_idx: empty_html_bra_idx,
                function_ident,
                ref arguments,
                empty_htmx_ket: empty_html_ket,
            } => {
                let mut deps = SemValueVarDeps::default();
                for argument in arguments {
                    deps.merge(&self.expr_value_var_deps_table[argument.expr()])
                }
                deps
            }
            // ad hoc
            SemExprData::Closure {
                ref parameter_obelisks,
                body,
                ..
            } => self.expr_value_var_deps_table[body].clone(),
            SemExprData::Sorry { .. }
            | SemExprData::Todo { .. }
            | SemExprData::Unreachable { .. } => Default::default(),
        }
    }

    // todo: move this out of this module
    fn calc_expr_control_transfer(&mut self, expr: SemExprIdx) -> SemControlTransferVarDeps {
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
                let mut deps = self.expr_control_transfer_var_deps_table[lopd].clone();
                deps.merge(&self.expr_control_transfer_var_deps_table[ropd]);
                deps
            }
            SemExprData::Be {
                src,
                contract,
                be_regional_token_idx,
                target,
            } => todo!(),
            SemExprData::Prefix { opd, .. } => {
                self.expr_control_transfer_var_deps_table[opd].clone()
            }
            SemExprData::Suffix { opd, .. } => {
                self.expr_control_transfer_var_deps_table[opd].clone()
            }
            SemExprData::Unveil {
                opd,
                opr_regional_token_idx,
                ref unveil_output_ty_signature,
                ref unveil_assoc_fn_signature,
                unveil_assoc_fn_path,
                return_ty,
            } => {
                let mut deps = self.expr_control_transfer_var_deps_table[opd].clone();
                deps.merge_with_value(&self.expr_value_var_deps_table[opd]);
                let path_deps = self.calc_path(unveil_assoc_fn_path);
                deps.merge_with_value(&path_deps);
                deps
            }
            SemExprData::Unwrap { opd, .. } => {
                let mut deps = self.expr_control_transfer_var_deps_table[opd].clone();
                deps.merge_with_value(&self.expr_value_var_deps_table[opd]);
                deps
            }
            SemExprData::FunctionApplication { function, argument } => todo!(),
            SemExprData::FunctionRitchieCall {
                function,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let mut deps = self.expr_control_transfer_var_deps_table[function].clone();
                for m in ritchie_parameter_argument_matches {
                    match m {
                        SemRitchieArgument::Simple(_, arg) => deps.merge(
                            &self.expr_control_transfer_var_deps_table[arg.argument_expr_idx],
                        ),
                        SemRitchieArgument::Variadic(_, args) => {
                            for arg in args {
                                deps.merge(
                                    &self.expr_control_transfer_var_deps_table
                                        [arg.argument_expr_idx()],
                                );
                            }
                        }
                        SemRitchieArgument::Keyed(_, arg) => match arg {
                            Some(arg) => deps.merge(
                                &self.expr_control_transfer_var_deps_table[arg.argument_expr_idx()],
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
                self.expr_control_transfer_var_deps_table[self_argument].clone()
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
                dispatch: ref instance_dispatch,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                let mut deps = self.expr_control_transfer_var_deps_table[self_argument].clone();
                for m in ritchie_parameter_argument_matches {
                    match m {
                        SemRitchieArgument::Simple(_, arg) => deps.merge(
                            &self.expr_control_transfer_var_deps_table[arg.argument_expr_idx],
                        ),
                        SemRitchieArgument::Variadic(_, _) => todo!(),
                        SemRitchieArgument::Keyed(_, _) => todo!(),
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
                self.expr_control_transfer_var_deps_table[item].clone()
            }
            SemExprData::NewTuple {
                lpar_regional_token_idx,
                ref items,
                rpar_regional_token_idx,
            } => todo!(),
            SemExprData::Index {
                self_argument: owner,
                items: ref index_sem_list_items,
                ref index_dynamic_dispatch,
                ..
            } => {
                let mut deps = self.expr_control_transfer_var_deps_table[owner].clone();
                for item in index_sem_list_items {
                    deps.merge(&self.expr_control_transfer_var_deps_table[item.expr])
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
                let mut deps = SemControlTransferVarDeps::default();
                for item in items {
                    deps.merge(&self.expr_control_transfer_var_deps_table[item.expr]);
                }
                deps
            }
            SemExprData::BoxColonList { .. }
            | SemExprData::VecFunctor { .. }
            | SemExprData::ArrayFunctor { .. } => Default::default(),
            SemExprData::Block { stmts } | SemExprData::NestedBlock { stmts, .. } => {
                let mut deps = Default::default();
                self.calc_stmts_control_transfer(None, stmts, &mut deps);
                deps
            }
            SemExprData::EmptyHtmxTag {
                empty_htmx_bra_idx: empty_html_bra_idx,
                function_ident,
                ref arguments,
                empty_htmx_ket: empty_html_ket,
            } => {
                let mut deps = SemControlTransferVarDeps::default();
                for argument in arguments {
                    deps.merge(&self.expr_control_transfer_var_deps_table[argument.expr()])
                }
                deps
            }
            SemExprData::Closure { .. } => Default::default(),
            // ad hoc, todo: consider arguments
            SemExprData::Sorry { regional_token_idx } => SemControlTransferVarDeps::new_empty(),
            // ad hoc, todo: consider arguments
            SemExprData::Todo { regional_token_idx } => SemControlTransferVarDeps::new_empty(),
            // ad hoc, todo: consider arguments
            SemExprData::Unreachable { regional_token_idx } => {
                SemControlTransferVarDeps::new_empty()
            }
        }
    }

    fn calc_stmt_value(&mut self, stmt: SemStmtIdx) -> SemValueVarDeps {
        match *stmt.data(self.sem_expr_region_data.sem_stmt_arena()) {
            SemStmtData::Let {
                let_token,
                let_pattern_sem_obelisk,
                contract,
                eq_token,
                initial_value,
                ref coercion_outcome,
            } => {
                let initial_value_deps = &self.expr_value_var_deps_table[initial_value];
                for variable in let_pattern_sem_obelisk.variables() {
                    self.variable_var_deps_table.insert_new_current_or_merge(
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
                    self.expr_value_var_deps_table[expr].clone()
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
                let mut deps = SemValueVarDeps::default();
                let mut t = |condition: Option<SemCondition>, stmts: SemStmtIdxRange| {
                    condition.map(|condition| deps.merge(&self.calc_condition_value(condition)));
                    deps.merge(&self.calc_stmt_value(stmts.last().unwrap()));
                };
                t(Some(if_branch.condition), if_branch.stmts);
                for elif_branch in elif_branches {
                    t(Some(elif_branch.condition), elif_branch.stmts)
                }
                if let Some(else_branch) = else_branch {
                    t(None, else_branch.stmts)
                }
                deps
            }
            SemStmtData::Match {
                match_token,
                opd,
                eol_with_token,
                ref case_branches,
                ..
            } => {
                let mut deps = self.expr_value_var_deps_table[opd].clone();
                for case_branch in case_branches {
                    if let Some(condition) = case_branch.condition() {
                        deps.merge(&self.calc_condition_value(condition));
                    }
                    deps.merge(&self.stmt_value_var_deps_table[case_branch.stmts.last().unwrap()]);
                }
                deps
            }
            // ad hoc
            SemStmtData::Narrate { narrate_token } => Default::default(),
        }
    }

    fn calc_stmt_control_transfer(&mut self, stmt: SemStmtIdx) -> SemControlTransferVarDeps {
        match *stmt.data(self.sem_expr_region_data.sem_stmt_arena()) {
            SemStmtData::Let { initial_value, .. } => {
                self.expr_control_transfer_var_deps_table[initial_value].clone()
            }
            SemStmtData::Return { result, .. } => {
                let mut deps = self.expr_control_transfer_var_deps_table[result].clone();
                deps.merge_with_value(&self.expr_value_var_deps_table[result]);
                deps
            }
            SemStmtData::Require { condition, .. } => {
                // todo: consider deps of Default::default
                let mut deps = self.calc_condition_control_transfer(condition);
                deps.merge_with_value(&self.calc_condition_value(condition));
                deps
            }
            SemStmtData::Assert { condition, .. } => {
                let mut deps = self.calc_condition_control_transfer(condition);
                deps.merge_with_value(&self.calc_condition_value(condition));
                deps
            }
            SemStmtData::Break { break_token } => Default::default(),
            SemStmtData::Eval { expr, .. } => {
                self.expr_control_transfer_var_deps_table[expr].clone()
            }
            SemStmtData::ForBetween {
                ref particulars,
                stmts,
                ..
            } => {
                let mut deps = SemControlTransferVarDeps::default();
                if let Some(bound_expr) = particulars.range().initial_boundary.bound_expr {
                    self.expr_control_transfer_var_deps_table[bound_expr].clone();
                }
                if let Some(bound_expr) = particulars.range().final_boundary.bound_expr {
                    self.expr_control_transfer_var_deps_table[bound_expr].clone();
                }
                let mut condition_value_deps = SemValueVarDeps::default();
                if let Some(bound_expr) = particulars.range().initial_boundary.bound_expr {
                    condition_value_deps.merge(&self.expr_value_var_deps_table[bound_expr]);
                }
                if let Some(bound_expr) = particulars.range().final_boundary.bound_expr {
                    condition_value_deps.merge(&self.expr_value_var_deps_table[bound_expr]);
                }
                self.calc_stmts_control_transfer(Some(condition_value_deps), stmts, &mut deps);
                deps
            }
            SemStmtData::ForIn {
                for_token,
                range,
                eol_colon,
                stmts,
            } => {
                let mut deps = SemControlTransferVarDeps::default();
                todo!();
                // if let Some(bound_expr) = t
                // self.expr_control_flow_var_deps_table[bound_expr].clone();
                let condition_value_deps = todo!();
                self.calc_stmts_control_transfer(Some(condition_value_deps), stmts, &mut deps);
                deps
            }
            SemStmtData::Forext {
                ref particulars,
                stmts,
                ..
            } => {
                let mut deps =
                    self.expr_control_transfer_var_deps_table[particulars.bound_expr].clone();
                let condition_value_deps =
                    self.expr_value_var_deps_table[particulars.bound_expr].clone();
                self.calc_stmts_control_transfer(Some(condition_value_deps), stmts, &mut deps);
                deps
            }
            SemStmtData::While {
                condition, stmts, ..
            }
            | SemStmtData::DoWhile {
                condition, stmts, ..
            } => {
                let mut deps = self.calc_condition_control_transfer(condition);
                let condition_value_deps = self.calc_condition_value(condition);
                self.calc_stmts_control_transfer(Some(condition_value_deps), stmts, &mut deps);
                deps
            }
            SemStmtData::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => {
                let mut deps = SemControlTransferVarDeps::default();
                let mut t = |condition: Option<SemCondition>, stmts| {
                    condition.map(|condition| {
                        deps.merge(&self.calc_condition_control_transfer(condition))
                    });
                    let condition_value_deps =
                        condition.map(|condition| self.calc_condition_value(condition));
                    self.calc_stmts_control_transfer(condition_value_deps, stmts, &mut deps);
                };
                t(Some(if_branch.condition), if_branch.stmts);
                for elif_branch in elif_branches {
                    t(Some(elif_branch.condition), elif_branch.stmts)
                }
                if let Some(else_branch) = else_branch {
                    t(None, else_branch.stmts)
                }
                deps
            }
            SemStmtData::Match {
                match_token,
                opd: match_opd,
                contract: match_contract,
                eol_with_token,
                ref case_branches,
            } => {
                let mut deps = self.expr_control_transfer_var_deps_table[match_opd].clone();
                for case_branch in case_branches {
                    let mut condition_value_deps =
                        self.expr_value_var_deps_table[match_opd].clone();
                    if let Some(condition) = case_branch.condition() {
                        deps.merge(&self.calc_condition_control_transfer(condition));
                        condition_value_deps.merge(&self.calc_condition_value(condition));
                    }
                    self.calc_stmts_control_transfer(
                        Some(condition_value_deps),
                        case_branch.stmts,
                        &mut deps,
                    )
                }
                deps
            }
            // ad hoc
            SemStmtData::Narrate { .. } => Default::default(),
        }
    }

    fn calc_stmts_control_transfer(
        &mut self,
        condition_value_var_deps: Option<SemValueVarDeps>,
        stmts: SemStmtIdxRange,
        deps: &mut SemControlTransferVarDeps,
    ) {
        match condition_value_var_deps {
            Some(condition_var_deps) if condition_var_deps.len() > 0 => {
                let mut stmts_deps = SemControlTransferVarDeps::default();
                for stmt in stmts {
                    stmts_deps.merge(&self.stmt_control_transfer_var_deps_table[stmt]);
                }
                stmts_deps.compose_with_value(&condition_var_deps);
                deps.merge(&stmts_deps)
            }
            _ => {
                for stmt in stmts {
                    deps.merge(&self.stmt_control_transfer_var_deps_table[stmt]);
                }
            }
        }
    }

    fn calc_condition_value(&mut self, condition: SemCondition) -> SemValueVarDeps {
        match condition {
            SemCondition::Be {
                src,
                contract,
                be_regional_token_idx,
                target,
            } => {
                let deps = self.expr_value_var_deps_table[src].clone();
                self.populate_into_current_variables(target.variables(), &deps);
                deps
            }
            SemCondition::Other { expr: src, .. } => self.expr_value_var_deps_table[src].clone(),
        }
    }

    fn calc_condition_control_transfer(
        &mut self,
        condition: SemCondition,
    ) -> SemControlTransferVarDeps {
        match condition {
            SemCondition::Be { src, .. } => self.expr_control_transfer_var_deps_table[src].clone(),
            SemCondition::Other { expr: src, .. } => {
                self.expr_control_transfer_var_deps_table[src].clone()
            }
        }
    }

    fn populate_into_current_variables(
        &mut self,
        variables: CurrentVariableIdxRange,
        deps: &SemValueVarDeps,
    ) {
        for variable in variables {
            self.variable_var_deps_table.insert_new_current_or_merge(
                variable,
                deps.clone(),
                |deps0, deps| SemValueVarDeps::merge(deps0, &deps),
            )
        }
    }

    fn with_domain_guard_contained(&mut self, f: impl FnOnce(&mut Self)) {
        let domain_guard = self.domain_guard.clone();
        f(self);
        self.domain_guard = domain_guard;
    }

    pub(crate) fn finish(self) -> ItemDefnSemVarDepsRegion {
        ItemDefnSemVarDepsRegion::new(
            self.db,
            self.expr_value_var_deps_table,
            self.expr_control_transfer_var_deps_table,
            self.expr_domain_var_deps_table,
            self.expr_control_flow_var_deps_table,
            self.stmt_value_var_deps_table,
            self.stmt_control_transfer_var_deps_table,
            self.stmt_domain_var_deps_table,
            self.stmt_control_flow_var_deps_table,
            self.self_value_var_deps,
            self.variable_var_deps_table,
        )
    }
}

impl<'db, 'a, F> VisitSemExpr<'db> for SemVarDepsBuilder<'db, 'a, F>
where
    F: Fn(ItemPath) -> &'a SemValueVarDeps,
{
    fn db(&self) -> &'db salsa::Db {
        self.db
    }

    fn sem_expr_region_data(&self) -> &'db SemExprRegionData {
        self.sem_expr_region_data
    }

    fn visit_expr(&mut self, expr: SemExprIdx, f: impl FnOnce(&mut Self)) {
        self.expr_domain_var_deps_table.insert_new_or_merge(
            expr,
            self.domain_guard.domain_var_deps(),
            |deps0, deps| deps0.merge_counted(&deps, &mut self.counter),
        );
        f(self)
    }

    fn visit_expr_itself(&mut self, expr: SemExprIdx) {
        let deps = self.calc_expr_value(expr);
        self.expr_value_var_deps_table
            .insert_new_or_merge(expr, deps, |deps0, deps| {
                deps0.merge_counted(&deps, &mut self.counter)
            });
        let deps = self.calc_expr_control_transfer(expr);
        self.expr_control_transfer_var_deps_table
            .insert_new_or_merge(expr, deps, |deps0, deps| {
                deps0.merge_counted(&deps, &mut self.counter)
            });
        self.expr_control_flow_var_deps_table.insert(
            expr,
            SemControlFlowVarDeps::new(
                &self.expr_value_var_deps_table[expr],
                &self.expr_control_transfer_var_deps_table[expr],
                &self.expr_domain_var_deps_table[expr],
            ),
        );
        self.domain_guard
            .extend(&self.expr_control_transfer_var_deps_table[expr]);
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
        self.stmt_domain_var_deps_table.insert_new_or_merge(
            stmt,
            self.domain_guard.domain_var_deps(),
            |deps0, deps| deps0.merge_counted(&deps, &mut self.counter),
        );
        f(self)
    }

    fn visit_for_loop_stmt_inner(
        &mut self,
        stmt: SemStmtIdx,
        current_variable_idx: CurrentVariableIdx,
        f: impl Fn(&mut Self),
    ) {
        self.variable_var_deps_table
            .insert_new_current(current_variable_idx, Default::default());
        f(self)
    }

    fn visit_stmt_itself(&mut self, stmt: SemStmtIdx) {
        let deps = self.calc_stmt_value(stmt);
        self.stmt_value_var_deps_table
            .insert_new_or_merge(stmt, deps, |deps0, deps| {
                deps0.merge_counted(&deps, &mut self.counter)
            });
        let deps = self.calc_stmt_control_transfer(stmt);
        self.stmt_control_transfer_var_deps_table
            .insert_new_or_merge(stmt, deps, |deps0, deps| {
                deps0.merge_counted(&deps, &mut self.counter)
            });
        self.stmt_control_flow_var_deps_table.insert(
            stmt,
            SemControlFlowVarDeps::new(
                &self.stmt_value_var_deps_table[stmt],
                &self.stmt_control_transfer_var_deps_table[stmt],
                &self.stmt_domain_var_deps_table[stmt],
            ),
        );
        self.domain_guard
            .extend(&self.stmt_control_transfer_var_deps_table[stmt]);
    }

    fn visit_loop(&mut self, stmt: SemStmtIdx, f: impl Fn(&mut Self)) {
        loop {
            let old_count = self.counter.count();
            f(self);
            if old_count == self.counter.count() {
                break;
            }
            debug_assert!(self.counter.count() < 1000);
        }
    }

    fn visit_branches(&mut self, f: impl Fn(&mut Self)) {
        self.with_domain_guard_contained(f);
    }

    fn visit_branch(&mut self, f: impl Fn(&mut Self)) {
        f(self)
    }

    fn visit_condition(&mut self, condition: SemCondition, f: impl FnOnce(&mut Self)) {
        f(self);
        let condition_value_var_deps = match condition {
            SemCondition::Be {
                src,
                contract,
                be_regional_token_idx,
                target,
            } => &self.expr_value_var_deps_table[src],
            SemCondition::Other { expr, conversion } => &self.expr_value_var_deps_table[expr],
        };
    }

    fn visit_condition_inner(&mut self, condition: SemCondition) {
        ()
    }

    fn visit_branch_stmts(&mut self, f: impl Fn(&mut Self)) {
        self.with_domain_guard_contained(f);
    }
}
