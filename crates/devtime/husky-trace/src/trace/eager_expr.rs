use husky_sema_expr::{helpers::range::sema_expr_range_region, SemaExprData, SemaExprRegion};

use crate::registry::associated_trace::VoidAssociatedTraceRegistry;

use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct EagerExprTracePath {
    pub biological_parent_path: EagerExprTraceBiologicalParentPath,
    #[return_ref]
    pub data: EagerExprTracePathData,
    pub disambiguator: TracePathDisambiguator<EagerExprTracePathData>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum EagerExprTraceBiologicalParentPath {
    EagerStmt(EagerStmtTracePath),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EagerExprTracePathData {
    Haha,
}

impl EagerExprTracePath {
    fn new(
        biological_parent_path: EagerExprTraceBiologicalParentPath,
        path_data: EagerExprTracePathData,
        eager_expr_trace_path_registry: &mut TracePathRegistry<EagerExprTracePathData>,
        db: &dyn TraceDb,
    ) -> Self {
        Self::new_inner(
            db,
            biological_parent_path,
            path_data.clone(),
            eager_expr_trace_path_registry.issue(path_data),
        )
    }
}

#[salsa::tracked(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct EagerExprTrace {
    #[id]
    pub path: EagerExprTracePath,
    pub biological_parent: EagerExprTraceBiologicalParent,
    pub data: EagerExprTraceData,
    #[skip_fmt]
    pub sema_expr_region: SemaExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum EagerExprTraceBiologicalParent {
    EagerStmt(EagerStmtTrace),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum EagerExprTraceData {
    Expr(SemaExprIdx),
    PatternExpr(SynPatternExprIdx),
}

impl EagerExprTrace {
    pub(crate) fn new(
        biological_parent_path: impl Into<EagerExprTraceBiologicalParentPath>,
        biological_parent: impl Into<EagerExprTraceBiologicalParent>,
        data: impl Into<EagerExprTraceData>,
        sema_expr_region: SemaExprRegion,
        lazy_expr_trace_path_registry: &mut TracePathRegistry<EagerExprTracePathData>,
        db: &dyn TraceDb,
    ) -> Self {
        let path_data = EagerExprTracePathData::Haha;
        let path = EagerExprTracePath::new(
            biological_parent_path.into(),
            path_data,
            lazy_expr_trace_path_registry,
            db,
        );
        Self::new_inner(
            db,
            path,
            biological_parent.into(),
            data.into(),
            sema_expr_region,
        )
    }

    pub fn view_data(self, db: &dyn TraceDb) -> TraceViewData {
        let tokens = eager_expr_trace_view_lines(db, self);
        TraceViewData::new(tokens.data().to_vec(), self.have_subtraces(db))
    }

    pub fn have_subtraces(self, db: &dyn TraceDb) -> bool {
        eager_expr_trace_have_subtraces(db, self)
    }

    pub fn subtraces(self, _db: &dyn TraceDb) -> &[Trace] {
        todo!()
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn eager_expr_trace_view_lines(db: &dyn TraceDb, trace: EagerExprTrace) -> TraceViewLines {
    let sema_expr_region = trace.sema_expr_region(db);
    let sema_expr_range_region = sema_expr_range_region(db, sema_expr_region);
    let sema_expr_range_region_data = sema_expr_range_region.data(db);
    let region_path = sema_expr_region.path(db);
    let regional_token_idx_range = match trace.data(db) {
        EagerExprTraceData::Expr(sema_expr_idx) => sema_expr_range_region_data[sema_expr_idx],
        EagerExprTraceData::PatternExpr(_) => todo!(),
    };
    let token_idx_range =
        regional_token_idx_range.token_idx_range(region_path.regional_token_idx_base(db).unwrap());
    TraceViewLines::new(
        region_path.module_path(db),
        token_idx_range,
        VoidAssociatedTraceRegistry,
        db,
    )
}

#[salsa::tracked(jar = TraceJar)]
fn eager_expr_trace_have_subtraces(db: &dyn TraceDb, trace: EagerExprTrace) -> bool {
    match trace.data(db) {
        EagerExprTraceData::Expr(sema_expr_idx) => {
            let sema_expr_region = trace.sema_expr_region(db);
            let sema_expr_region_data = sema_expr_region.data(db);
            match sema_expr_idx.data(sema_expr_region_data.sema_expr_arena()) {
                SemaExprData::Literal(_, _) => false,
                SemaExprData::PrincipalEntityPath { .. } => todo!(),
                SemaExprData::AssociatedItem {
                    parent_expr_idx: _,
                    parent_path: _,
                    colon_colon_regional_token: _,
                    ident_token: _,
                    static_dispatch: _,
                } => todo!(),
                SemaExprData::InheritedSynSymbol { .. } | SemaExprData::CurrentSynSymbol { .. } => {
                    false
                }
                SemaExprData::FrameVarDecl {
                    regional_token_idx: _,
                    ident: _,
                    frame_var_symbol_idx: _,
                    current_syn_symbol_kind: _,
                } => todo!(),
                SemaExprData::SelfType(_) => false,
                SemaExprData::SelfValue(_) => false,
                SemaExprData::Binary {
                    lopd: _,
                    opr: _,
                    opr_regional_token_idx: _,
                    dispatch: _,
                    ropd: _,
                } => todo!(),
                SemaExprData::Be {
                    src: _,
                    be_regional_token_idx: _,
                    target: _,
                } => todo!(),
                SemaExprData::Prefix {
                    opr: _,
                    opr_regional_token_idx: _,
                    opd_sema_expr_idx: _,
                } => todo!(),
                SemaExprData::Suffix {
                    opd_sema_expr_idx: _,
                    opr: _,
                    opr_regional_token_idx: _,
                } => todo!(),
                SemaExprData::FunctionApplication {
                    function_sema_expr_idx: _,
                    argument_sema_expr_idx: _,
                } => todo!(),
                SemaExprData::FunctionFnCall {
                    function_sema_expr_idx: _,
                    template_arguments: _,
                    lpar_regional_token_idx: _,
                    ritchie_parameter_argument_matches: _,
                    rpar_regional_token_idx: _,
                } => todo!(),
                SemaExprData::FunctionGnCall {
                    function: _,
                    template_arguments: _,
                    lpar_regional_token_idx: _,
                    ritchie_parameter_argument_matches: _,
                    rpar_regional_token_idx: _,
                } => todo!(),
                SemaExprData::Ritchie {
                    ritchie_kind_regional_token_idx: _,
                    ritchie_kind: _,
                    lpar_token: _,
                    parameter_ty_items: _,
                    rpar_regional_token_idx: _,
                    light_arrow_token: _,
                    return_ty_sema_expr_idx: _,
                } => todo!(),
                SemaExprData::Field {
                    owner_sema_expr_idx: _,
                    dot_regional_token_idx: _,
                    ident_token: _,
                    field_dispatch: _,
                } => todo!(),
                SemaExprData::MethodApplication {
                    self_argument: _,
                    dot_regional_token_idx: _,
                    ident_token: _,
                    template_arguments: _,
                    lpar_regional_token_idx: _,
                    items: _,
                    rpar_regional_token_idx: _,
                } => todo!(),
                SemaExprData::MethodFnCall {
                    self_argument_sema_expr_idx: _,
                    dot_regional_token_idx: _,
                    ident_token: _,
                    dispatch: _,
                    template_arguments: _,
                    lpar_regional_token_idx: _,
                    ritchie_parameter_argument_matches: _,
                    rpar_regional_token_idx: _,
                } => todo!(),
                SemaExprData::MethodGnCall {
                    self_argument_sema_expr_idx: _,
                    dot_regional_token_idx: _,
                    ident_token: _,
                    method_dynamic_dispatch: _,
                    template_arguments: _,
                    lpar_regional_token_idx: _,
                    ritchie_parameter_argument_matches: _,
                    rpar_regional_token_idx: _,
                } => todo!(),
                SemaExprData::TemplateInstantiation {
                    template: _,
                    template_arguments: _,
                } => todo!(),
                SemaExprData::At {
                    at_regional_token_idx: _,
                    place_label_regional_token: _,
                } => todo!(),
                SemaExprData::Unit {
                    lpar_regional_token_idx: _,
                    rpar_regional_token_idx: _,
                } => todo!(),
                SemaExprData::Bracketed {
                    lpar_regional_token_idx: _,
                    item: _,
                    rpar_regional_token_idx: _,
                } => todo!(),
                SemaExprData::NewTuple {
                    lpar_regional_token_idx: _,
                    items: _,
                    rpar_regional_token_idx: _,
                } => todo!(),
                SemaExprData::Index {
                    owner_sema_expr_idx: _,
                    lbox_regional_token_idx: _,
                    index_sema_list_items: _,
                    rbox_regional_token_idx: _,
                    index_dynamic_dispatch: _,
                } => todo!(),
                SemaExprData::CompositionWithList {
                    owner: _,
                    lbox_regional_token_idx: _,
                    items: _,
                    rbox_regional_token_idx: _,
                } => todo!(),
                SemaExprData::NewList {
                    lbox_regional_token_idx: _,
                    items: _,
                    rbox_regional_token_idx: _,
                } => todo!(),
                SemaExprData::BoxColonList {
                    lbox_regional_token_idx: _,
                    colon_regional_token_idx: _,
                    items: _,
                    rbox_regional_token_idx: _,
                } => todo!(),
                SemaExprData::VecFunctor {
                    lbox_regional_token_idx: _,
                    rbox_regional_token_idx: _,
                } => todo!(),
                SemaExprData::ArrayFunctor {
                    lbox_regional_token_idx: _,
                    items: _,
                    rbox_regional_token_idx: _,
                } => todo!(),
                SemaExprData::Block { stmts: _ } => todo!(),
                SemaExprData::EmptyHtmlTag {
                    empty_html_bra_idx: _,
                    function_ident: _,
                    arguments: _,
                    empty_html_ket: _,
                } => todo!(),
                SemaExprData::Sorry { regional_token_idx: _ } => todo!(),
                SemaExprData::Todo { regional_token_idx: _ } => todo!(),
                SemaExprData::Unreachable { regional_token_idx: _ } => todo!(),
            }
        }
        EagerExprTraceData::PatternExpr(_) => false,
    }
}

#[salsa::tracked(jar = TraceJar)]
fn eager_expr_trace_subtraces(db: &dyn TraceDb, trace: EagerExprTrace) -> Vec<Trace> {
    match trace.data(db) {
        EagerExprTraceData::Expr(_) => todo!(),
        EagerExprTraceData::PatternExpr(_) => todo!(),
    }
}
