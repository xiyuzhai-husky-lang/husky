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
        let tokens = eager_expr_trace_view_tokens(db, self);
        TraceViewData::new(tokens.data().to_vec(), self.have_subtraces(db))
    }

    pub fn have_subtraces(self, db: &dyn TraceDb) -> bool {
        eager_expr_trace_have_subtraces(db, self)
    }

    pub fn subtraces(self, db: &dyn TraceDb) -> &[Trace] {
        todo!()
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn eager_expr_trace_view_tokens(db: &dyn TraceDb, trace: EagerExprTrace) -> TraceViewTokens {
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
    TraceViewTokens::new(
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
                    parent_expr_idx,
                    parent_path,
                    colon_colon_regional_token,
                    ident_token,
                    static_dispatch,
                } => todo!(),
                SemaExprData::InheritedSymbol { .. } | SemaExprData::CurrentSymbol { .. } => false,
                SemaExprData::FrameVarDecl {
                    regional_token_idx,
                    ident,
                    frame_var_symbol_idx,
                    current_symbol_kind,
                } => todo!(),
                SemaExprData::SelfType(_) => false,
                SemaExprData::SelfValue(_) => false,
                SemaExprData::Binary {
                    lopd,
                    opr,
                    opr_regional_token_idx,
                    dispatch,
                    ropd,
                } => todo!(),
                SemaExprData::Be {
                    src,
                    be_regional_token_idx,
                    target,
                } => todo!(),
                SemaExprData::Prefix {
                    opr,
                    opr_regional_token_idx,
                    opd_sema_expr_idx,
                } => todo!(),
                SemaExprData::Suffix {
                    opd_sema_expr_idx,
                    opr,
                    opr_regional_token_idx,
                } => todo!(),
                SemaExprData::FunctionApplication {
                    function_sema_expr_idx,
                    argument_sema_expr_idx,
                } => todo!(),
                SemaExprData::FunctionFnCall {
                    function_sema_expr_idx,
                    template_arguments,
                    lpar_regional_token_idx,
                    ritchie_parameter_argument_matches,
                    rpar_regional_token_idx,
                } => todo!(),
                SemaExprData::FunctionGnCall {
                    function,
                    template_arguments,
                    lpar_regional_token_idx,
                    ritchie_parameter_argument_matches,
                    rpar_regional_token_idx,
                } => todo!(),
                SemaExprData::Ritchie {
                    ritchie_kind_regional_token_idx,
                    ritchie_kind,
                    lpar_token,
                    parameter_ty_items,
                    rpar_regional_token_idx,
                    light_arrow_token,
                    return_ty_sema_expr_idx,
                } => todo!(),
                SemaExprData::Field {
                    owner_sema_expr_idx,
                    dot_regional_token_idx,
                    ident_token,
                    field_dispatch,
                } => todo!(),
                SemaExprData::MethodApplication {
                    self_argument,
                    dot_regional_token_idx,
                    ident_token,
                    template_arguments,
                    lpar_regional_token_idx,
                    items,
                    rpar_regional_token_idx,
                } => todo!(),
                SemaExprData::MethodFnCall {
                    self_argument_sema_expr_idx,
                    dot_regional_token_idx,
                    ident_token,
                    dispatch,
                    template_arguments,
                    lpar_regional_token_idx,
                    ritchie_parameter_argument_matches,
                    rpar_regional_token_idx,
                } => todo!(),
                SemaExprData::MethodGnCall {
                    self_argument_sema_expr_idx,
                    dot_regional_token_idx,
                    ident_token,
                    method_dynamic_dispatch,
                    template_arguments,
                    lpar_regional_token_idx,
                    ritchie_parameter_argument_matches,
                    rpar_regional_token_idx,
                } => todo!(),
                SemaExprData::TemplateInstantiation {
                    template,
                    template_arguments,
                } => todo!(),
                SemaExprData::At {
                    at_regional_token_idx,
                    place_label_regional_token,
                } => todo!(),
                SemaExprData::Unit {
                    lpar_regional_token_idx,
                    rpar_regional_token_idx,
                } => todo!(),
                SemaExprData::Bracketed {
                    lpar_regional_token_idx,
                    item,
                    rpar_regional_token_idx,
                } => todo!(),
                SemaExprData::NewTuple {
                    lpar_regional_token_idx,
                    items,
                    rpar_regional_token_idx,
                } => todo!(),
                SemaExprData::Index {
                    owner_sema_expr_idx,
                    lbox_regional_token_idx,
                    index_sema_list_items,
                    rbox_regional_token_idx,
                    index_dynamic_dispatch,
                } => todo!(),
                SemaExprData::CompositionWithList {
                    owner,
                    lbox_regional_token_idx,
                    items,
                    rbox_regional_token_idx,
                } => todo!(),
                SemaExprData::NewList {
                    lbox_regional_token_idx,
                    items,
                    rbox_regional_token_idx,
                } => todo!(),
                SemaExprData::BoxColonList {
                    lbox_regional_token_idx,
                    colon_regional_token_idx,
                    items,
                    rbox_regional_token_idx,
                } => todo!(),
                SemaExprData::VecFunctor {
                    lbox_regional_token_idx,
                    rbox_regional_token_idx,
                } => todo!(),
                SemaExprData::ArrayFunctor {
                    lbox_regional_token_idx,
                    items,
                    rbox_regional_token_idx,
                } => todo!(),
                SemaExprData::Block { stmts } => todo!(),
                SemaExprData::EmptyHtmlTag {
                    empty_html_bra_idx,
                    function_ident,
                    arguments,
                    empty_html_ket,
                } => todo!(),
                SemaExprData::Sorry { regional_token_idx } => todo!(),
                SemaExprData::Todo { regional_token_idx } => todo!(),
                SemaExprData::Unreachable { regional_token_idx } => todo!(),
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
