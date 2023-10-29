use husky_sema_expr::{
    helpers::analysis::sema_expr_region_contains_gn, SemaExprData, SemaStmtData,
};
use husky_syn_decl::{FugitiveSynDecl, HasSynDecl};
use husky_syn_defn::{FugitiveSynDefn, HasSynDefn};

use crate::registry::{
    associated_trace::VoidAssociatedTraceRegistry, trace_path::TracePathRegistry,
};

use super::*;

#[salsa::interned(db = TraceDb, jar = TraceJar, constructor = new)]
pub struct ValItemTracePath {
    // todo: make it like submodule trace path??
    pub val_item_path: FugitivePath,
}

#[salsa::tracked(db = TraceDb, jar = TraceJar, constructor = new)]
pub struct ValItemTrace {
    #[id]
    pub path: ValItemTracePath,
}

impl ValItemTrace {
    pub fn from_val_item_path(val_item_path: FugitivePath, db: &dyn TraceDb) -> Self {
        debug_assert_eq!(val_item_path.fugitive_kind(db), FugitiveKind::Val);
        ValItemTrace::new(db, ValItemTracePath::new(db, val_item_path))
    }

    pub fn view_data(self, db: &dyn TraceDb) -> TraceViewData {
        let tokens = val_item_trace_view_tokens(db, self);
        TraceViewData::new(tokens.data().to_vec(), /* ad hoc */ true)
    }

    pub fn subtraces(self, db: &dyn TraceDb) -> &[Trace] {
        val_item_trace_subtraces(db, self)
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn val_item_trace_view_tokens(db: &dyn TraceDb, val_item_trace: ValItemTrace) -> TraceViewTokens {
    use husky_entity_syn_tree::helpers::tokra_region::HasDeclTokraRegion;
    use husky_entity_syn_tree::HasSynNodePath;
    let val_item_trace_path = val_item_trace.path(db);
    let val_item_path = val_item_trace_path.val_item_path(db);
    let token_idx_range = val_item_path
        .syn_node_path(db)
        .decl_tokra_region_token_idx_range(db);
    TraceViewTokens::new::<VoidAssociatedTraceRegistry>(
        val_item_path.module_path(db),
        token_idx_range,
        db,
    )
}

#[salsa::tracked(jar = TraceJar, return_ref)]
fn val_item_trace_subtraces(db: &dyn TraceDb, val_item_trace: ValItemTrace) -> Vec<Trace> {
    let val_item_trace_path = val_item_trace.path(db);
    let val_item_path = val_item_trace_path.val_item_path(db);
    let Ok(FugitiveSynDefn::Val(val_item_defn)) = val_item_path.syn_defn(db) else {
        unreachable!("no error at trace stage")
    };
    let Some((body, syn_expr_region)) = val_item_defn.body_with_syn_expr_region(db) else {
        return vec![];
    };
    let sema_expr_region = db.sema_expr_region(syn_expr_region);
    let sema_expr_region_data = sema_expr_region.data(db);
    // let body = sema_expr_region_data.sema_expr_roots(body);
    let body: SemaExprIdx = sema_expr_region_data.syn_root_to_sema_expr_idx(body);
    let mut registry = TracePathRegistry::<LazyStmtTracePathData>::default();
    let mut subtraces: Vec<Trace> = vec![];
    let sema_expr_arena = sema_expr_region_data.sema_expr_arena();
    let sema_stmt_arena = sema_expr_region_data.sema_stmt_arena();
    match body.data(sema_expr_arena) {
        &SemaExprData::Block { stmts } => {
            for stmt in stmts {
                match stmt.data(sema_stmt_arena) {
                    SemaStmtData::Let {
                        let_token,
                        let_pattern_sema_obelisk,
                        eq_token,
                        initial_value_sema_expr_idx,
                    } => {
                        let path_data = LazyStmtTracePathData::Let {};
                        let lazy_stmt_trace = LazyStmtTrace::new(
                            val_item_trace,
                            val_item_trace_path,
                            path_data,
                            &mut registry,
                            stmt,
                            LazyStmtTraceData::BasicStmt,
                            sema_expr_region,
                            db,
                        );
                        subtraces.push(lazy_stmt_trace.into())
                    }
                    SemaStmtData::Return {
                        return_token,
                        result,
                    } => {
                        let path_data = LazyStmtTracePathData::Return {};
                        let lazy_stmt_trace = LazyStmtTrace::new(
                            val_item_trace,
                            val_item_trace_path,
                            path_data,
                            &mut registry,
                            stmt,
                            LazyStmtTraceData::BasicStmt,
                            sema_expr_region,
                            db,
                        );
                        subtraces.push(lazy_stmt_trace.into())
                    }
                    SemaStmtData::Require {
                        require_token,
                        condition,
                    } => {
                        let path_data = LazyStmtTracePathData::Require {};
                        let lazy_stmt_trace = LazyStmtTrace::new(
                            val_item_trace,
                            val_item_trace_path,
                            path_data,
                            &mut registry,
                            stmt,
                            LazyStmtTraceData::BasicStmt,
                            sema_expr_region,
                            db,
                        );
                        subtraces.push(lazy_stmt_trace.into())
                    }
                    SemaStmtData::Assert {
                        assert_token,
                        condition,
                    } => {
                        let path_data = LazyStmtTracePathData::Assert {};
                        let lazy_stmt_trace = LazyStmtTrace::new(
                            val_item_trace,
                            val_item_trace_path,
                            path_data,
                            &mut registry,
                            stmt,
                            LazyStmtTraceData::BasicStmt,
                            sema_expr_region,
                            db,
                        );
                        subtraces.push(lazy_stmt_trace.into())
                    }
                    SemaStmtData::Break { break_token } => {
                        let path_data = LazyStmtTracePathData::Break {};
                        let lazy_stmt_trace = LazyStmtTrace::new(
                            val_item_trace,
                            val_item_trace_path,
                            path_data,
                            &mut registry,
                            stmt,
                            LazyStmtTraceData::BasicStmt,
                            sema_expr_region,
                            db,
                        );
                        subtraces.push(lazy_stmt_trace.into())
                    }
                    SemaStmtData::Eval {
                        sema_expr_idx,
                        eol_semicolon,
                    } => {
                        let path_data = LazyStmtTracePathData::Eval {};
                        let lazy_stmt_trace = LazyStmtTrace::new(
                            val_item_trace,
                            val_item_trace_path,
                            path_data,
                            &mut registry,
                            stmt,
                            LazyStmtTraceData::BasicStmt,
                            sema_expr_region,
                            db,
                        );
                        subtraces.push(lazy_stmt_trace.into())
                    }
                    SemaStmtData::ForBetween {
                        for_token,
                        particulars,
                        for_loop_var_symbol_idx,
                        eol_colon,
                        block,
                    } => todo!(),
                    SemaStmtData::ForIn {
                        for_token,
                        condition,
                        eol_colon,
                        block,
                    } => todo!(),
                    SemaStmtData::Forext {
                        forext_token,
                        particulars,
                        eol_colon,
                        block,
                    } => todo!(),
                    SemaStmtData::While {
                        while_token,
                        condition,
                        eol_colon,
                        block,
                    } => todo!(),
                    SemaStmtData::DoWhile {
                        do_token,
                        while_token,
                        condition,
                        eol_colon,
                        block,
                    } => todo!(),
                    SemaStmtData::IfElse {
                        sema_if_branch,
                        sema_elif_branches,
                        sema_else_branch,
                    } => todo!(),
                    SemaStmtData::Match {
                        match_token,
                        match_target_sema_expr_idx,
                        eol_with_token,
                        sema_case_branches,
                    } => todo!(),
                }
            }
        }
        _ => todo!(),
    }
    subtraces
}
