use condition::SemCondition;

use super::*;
use std::marker::PhantomData;

pub trait IsSemExprAggregation: Default {
    fn aggregate(&mut self, other: &Self);
}

pub trait IsSemExprAggregatorContext<'comptime> {
    type Aggregation: IsSemExprAggregation;

    fn sem_expr_region_data(&self) -> &'comptime SemExprRegionData;
    fn sem_expr_arena(&self) -> SemExprArenaRef<'comptime> {
        self.sem_expr_region_data().sem_expr_arena()
    }
    fn sem_stmt_arena(&self) -> SemStmtArenaRef<'comptime> {
        self.sem_expr_region_data().sem_stmt_arena()
    }
    fn calc_expr(&self, expr: &SemExprData) -> Self::Aggregation;
    fn calc_stmt(&self, expr: &SemStmtData) -> Self::Aggregation;
}

pub struct SemExprAggregationRegionData<Aggregation: IsSemExprAggregation> {
    expr_aggregations: SemExprMap<Aggregation>,
    stmt_aggregations: SemStmtMap<Aggregation>,
}

struct SemExprAggregator<'comptime, Context: IsSemExprAggregatorContext<'comptime>> {
    context: Context,
    expr_aggregations: SemExprMap<Context::Aggregation>,
    stmt_aggregations: SemStmtMap<Context::Aggregation>,
    stmts_aggregations: SemStmtsMap<Context::Aggregation>,
    phantom: PhantomData<&'comptime ()>,
}

impl<'comptime, Context: IsSemExprAggregatorContext<'comptime>>
    SemExprAggregator<'comptime, Context>
{
    fn new(context: Context) -> Self {
        Self {
            phantom: PhantomData,
            expr_aggregations: SemExprMap::new(context.sem_expr_arena()),
            stmt_aggregations: SemStmtMap::new(context.sem_stmt_arena()),
            stmts_aggregations: Default::default(),
            context,
        }
    }
}

impl<'comptime, Context: IsSemExprAggregatorContext<'comptime>>
    SemExprAggregator<'comptime, Context>
{
    fn aggregate_all(&mut self) {
        self.aggregate_expr(self.context.sem_expr_region_data().root_body());
    }

    fn aggregate_expr(&mut self, expr: SemExprIdx) -> &Context::Aggregation {
        let aggregation = self.calc_expr_aggregation(expr);
        self.expr_aggregations.insert(expr, aggregation);
        self.expr_aggregations.get(expr).unwrap()
    }

    fn calc_expr_aggregation(&mut self, expr: SemExprIdx) -> Context::Aggregation {
        let expr_entry = &self.context.sem_expr_arena()[expr];
        let mut aggregation = self.context.calc_expr(expr_entry.data());
        match *expr_entry.data() {
            SemExprData::Literal(_, _) | SemExprData::Unit { .. } => {
                // Leaf nodes, no further aggregation needed
            }
            SemExprData::PrincipalEntityPath { path_expr_idx, .. } => {}
            SemExprData::MajorItemPathAssocItem {
                parent_expr_idx, ..
            } => {}
            SemExprData::TypeAsTraitItem { ty, trai, .. } => {
                aggregation.aggregate(self.aggregate_expr(ty));
                aggregation.aggregate(self.aggregate_expr(trai));
            }
            SemExprData::AssocItem {
                parent_expr_idx, ..
            } => {
                aggregation.aggregate(self.aggregate_expr(parent_expr_idx));
            }
            SemExprData::InheritedVariable { .. }
            | SemExprData::CurrentVariable { .. }
            | SemExprData::FrameVarDecl { .. }
            | SemExprData::SelfType(_)
            | SemExprData::SelfValue(_) => {
                // Leaf nodes, no further aggregation needed
            }
            SemExprData::Binary { lopd, ropd, .. } => {
                aggregation.aggregate(self.aggregate_expr(lopd));
                aggregation.aggregate(self.aggregate_expr(ropd));
            }
            SemExprData::Be { src, target, .. } => {
                aggregation.aggregate(self.aggregate_expr(src));
                // Aggregate pattern if needed
            }
            SemExprData::Prefix { opd, .. }
            | SemExprData::Suffix { opd, .. }
            | SemExprData::Unveil { opd, .. }
            | SemExprData::Unwrap { opd, .. } => {
                aggregation.aggregate(self.aggregate_expr(opd));
            }
            SemExprData::FunctionApplication { function, argument } => {
                aggregation.aggregate(self.aggregate_expr(function));
                aggregation.aggregate(self.aggregate_expr(argument));
            }
            SemExprData::FunctionRitchieCall {
                function,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                aggregation.aggregate(self.aggregate_expr(function));
                self.aggregate_ritchie_parameter_argument_matches(
                    ritchie_parameter_argument_matches,
                    &mut aggregation,
                );
            }
            SemExprData::Ritchie {
                ref parameter_ty_items,
                return_ty,
                ..
            } => {
                for item in parameter_ty_items.iter() {
                    aggregation.aggregate(self.aggregate_expr(item.expr));
                }
                if let Some(return_ty) = return_ty {
                    aggregation.aggregate(self.aggregate_expr(return_ty));
                }
            }
            SemExprData::Field { self_argument, .. } => {
                aggregation.aggregate(self.aggregate_expr(self_argument));
            }
            SemExprData::MethodApplication {
                self_argument,
                ref items,
                ..
            } => {
                aggregation.aggregate(self.aggregate_expr(self_argument));
                for item in items.iter() {
                    aggregation.aggregate(self.aggregate_expr(item.expr));
                }
            }
            SemExprData::MethodRitchieCall {
                self_argument,
                ref ritchie_parameter_argument_matches,
                ..
            } => {
                aggregation.aggregate(self.aggregate_expr(self_argument));
                self.aggregate_ritchie_parameter_argument_matches(
                    ritchie_parameter_argument_matches,
                    &mut aggregation,
                );
            }
            SemExprData::TemplateInstantiation {
                template,
                ref template_arguments,
            } => {
                aggregation.aggregate(self.aggregate_expr(template));
                for arg in template_arguments.arguments() {
                    aggregation.aggregate(self.aggregate_expr(arg.expr));
                }
            }
            SemExprData::At { .. } => {}
            SemExprData::Block { stmts, .. } => {
                aggregation.aggregate(self.aggregate_stmts(stmts));
            }
            SemExprData::Delimitered { item, .. } => {
                aggregation.aggregate(self.aggregate_expr(item));
            }
            SemExprData::NewTuple { ref items, .. } => {
                self.aggregate_expr_items(items, &mut aggregation);
            }
            SemExprData::Index {
                self_argument,
                ref items,
                ..
            } => {
                aggregation.aggregate(self.aggregate_expr(self_argument));
                self.aggregate_expr_items(items, &mut aggregation);
            }
            SemExprData::CompositionWithList {
                owner, ref items, ..
            } => {
                aggregation.aggregate(self.aggregate_expr(owner));
                self.aggregate_expr_items(items, &mut aggregation);
            }
            SemExprData::NewList { ref items, .. } => {
                self.aggregate_expr_items(items, &mut aggregation);
            }
            SemExprData::BoxColonList { ref items, .. } => {
                self.aggregate_expr_items(items, &mut aggregation);
            }
            SemExprData::VecFunctor { .. } => {
                // No expressions to aggregate
            }
            SemExprData::ArrayFunctor { ref items, .. } => {
                self.aggregate_expr_items(items, &mut aggregation);
            }
            SemExprData::EmptyHtmxTag { ref arguments, .. } => {
                for arg in arguments {
                    self.aggregate_htmx_argument(arg, &mut aggregation);
                }
            }
            SemExprData::Closure {
                return_ty, body, ..
            } => {
                if let Some((_, ty, _)) = return_ty {
                    aggregation.aggregate(self.aggregate_expr(ty));
                }
                aggregation.aggregate(self.aggregate_expr(body));
            }
            SemExprData::Sorry { .. }
            | SemExprData::Todo { .. }
            | SemExprData::Unreachable { .. } => {
                // No expressions to aggregate
            }
            SemExprData::NestedBlock { stmts, .. } => {
                aggregation.aggregate(self.aggregate_stmts(stmts));
            }
        }

        aggregation
    }

    fn aggregate_expr_items(
        &mut self,
        items: &[SemCommaListItem],
        aggregation: &mut Context::Aggregation,
    ) {
        for item in items {
            aggregation.aggregate(self.aggregate_expr(item.expr));
        }
    }

    fn aggregate_stmts(&mut self, stmts: SemStmtIdxRange) -> &Context::Aggregation {
        let aggregation = self.calc_stmts_aggregation(stmts);
        self.stmts_aggregations.insert((stmts, aggregation));
        &self.stmts_aggregations[stmts].1
    }

    fn calc_stmts_aggregation(&mut self, stmts: SemStmtIdxRange) -> Context::Aggregation {
        let mut aggregation = Context::Aggregation::default();
        for stmt in stmts {
            aggregation.aggregate(self.aggregate_stmt(stmt));
        }
        aggregation
    }

    fn aggregate_stmt(&mut self, stmt: SemStmtIdx) -> &Context::Aggregation {
        let aggregation = self.calc_stmt_aggregation(stmt);
        self.stmt_aggregations.insert(stmt, aggregation);
        self.stmt_aggregations.get(stmt).unwrap()
    }

    fn calc_stmt_aggregation(&mut self, stmt: SemStmtIdx) -> Context::Aggregation {
        let stmt_data = stmt.data(self.context.sem_stmt_arena());
        let mut aggregation = self.context.calc_stmt(stmt_data);

        match stmt_data {
            SemStmtData::Let { initial_value, .. } => {
                aggregation.aggregate(self.aggregate_expr(*initial_value));
            }
            SemStmtData::Return { result, .. } => {
                aggregation.aggregate(self.aggregate_expr(*result));
            }
            SemStmtData::Require { condition, .. } | SemStmtData::Assert { condition, .. } => {
                aggregation.aggregate(self.aggregate_condition(condition));
            }
            SemStmtData::Eval { expr, .. } => {
                aggregation.aggregate(self.aggregate_expr(*expr));
            }
            SemStmtData::ForBetween { stmts, .. }
            | SemStmtData::ForIn { stmts, .. }
            | SemStmtData::Forext { stmts, .. }
            | SemStmtData::While { stmts, .. }
            | SemStmtData::DoWhile { stmts, .. } => {
                aggregation.aggregate(self.aggregate_stmts(*stmts));
            }
            SemStmtData::ForIn { range, .. } => {
                aggregation.aggregate(self.aggregate_expr(*range));
            }
            SemStmtData::IfElse {
                if_branch,
                elif_branches,
                else_branch,
            } => {
                aggregation.aggregate(self.aggregate_condition(&if_branch.condition));
                aggregation.aggregate(self.aggregate_stmts(if_branch.stmts));

                for elif in elif_branches {
                    aggregation.aggregate(self.aggregate_condition(&elif.condition));
                    aggregation.aggregate(self.aggregate_stmts(elif.stmts));
                }

                if let Some(else_branch) = else_branch {
                    aggregation.aggregate(self.aggregate_stmts(else_branch.stmts));
                }
            }
            SemStmtData::Match {
                opd, case_branches, ..
            } => {
                aggregation.aggregate(self.aggregate_expr(*opd));
                for case in case_branches {
                    // Assuming case.pattern might contain expressions to aggregate
                    // if let Some(pattern_expr) = case.pattern.expr() {
                    //     aggregation.aggregate(self.aggregate_expr(pattern_expr));
                    // }
                    aggregation.aggregate(self.aggregate_stmts(case.stmts));
                }
            }
            SemStmtData::Break { .. } | SemStmtData::Narrate { .. } => {
                // These statements don't contain expressions to aggregate
            }
        }

        aggregation
    }

    fn aggregate_condition(&mut self, condition: &SemCondition) -> &Context::Aggregation {
        match condition {
            SemCondition::Be { src, .. } => self.aggregate_expr(*src),
            SemCondition::Other { expr, .. } => self.aggregate_expr(*expr),
        }
    }

    fn aggregate_ritchie_argument(
        &mut self,
        arg: &SemRitchieArgument,
        aggregation: &mut Context::Aggregation,
    ) {
        match arg {
            SemRitchieArgument::Simple(_, simple_arg) => {
                aggregation.aggregate(self.aggregate_expr(simple_arg.argument_expr_idx));
            }
            SemRitchieArgument::Variadic(_, variadic_items) => {
                for item in variadic_items {
                    aggregation.aggregate(self.aggregate_expr(item.argument_expr_idx()));
                }
            }
            SemRitchieArgument::Keyed(_, Some(keyed_item)) => {
                aggregation.aggregate(self.aggregate_expr(keyed_item.argument_expr_idx()));
            }
            SemRitchieArgument::Keyed(_, None) => {}
        }
    }

    fn aggregate_ritchie_parameter_argument_matches(
        &mut self,
        matches: &[SemRitchieArgument],
        aggregation: &mut Context::Aggregation,
    ) {
        for item in matches.iter() {
            self.aggregate_ritchie_argument(item, aggregation);
        }
    }

    fn aggregate_htmx_argument(
        &mut self,
        arg: &SemHtmxArgumentExpr,
        aggregation: &mut Context::Aggregation,
    ) {
        match *arg {
            SemHtmxArgumentExpr::Expanded { argument, .. } => {
                aggregation.aggregate(self.aggregate_expr(argument));
            }
            SemHtmxArgumentExpr::Shortened { .. } => {
                // ad hoc
                // Handle shortened arguments if necessary
            }
        }
    }

    fn finish(self) -> SemExprAggregationRegionData<Context::Aggregation> {
        SemExprAggregationRegionData {
            expr_aggregations: self.expr_aggregations,
            stmt_aggregations: self.stmt_aggregations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::path::sem_expr_region_from_region_path;
    use husky_entity_tree::region_path::SynNodeRegionPath;

    #[derive(Default, Clone, Copy)]
    struct ExprDepth(usize);

    impl IsSemExprAggregation for ExprDepth {
        fn aggregate(&mut self, other: &Self) {
            self.0 = self.0.max(other.0);
        }
    }

    struct DepthContext<'comptime> {
        sem_expr_region_data: &'comptime SemExprRegionData,
    }

    impl<'comptime> IsSemExprAggregatorContext<'comptime> for DepthContext<'comptime> {
        type Aggregation = ExprDepth;

        fn sem_expr_region_data(&self) -> &'comptime SemExprRegionData {
            self.sem_expr_region_data
        }

        fn calc_expr(&self, expr: &SemExprData) -> Self::Aggregation {
            ExprDepth(1)
        }

        fn calc_stmt(&self, stmt: &SemStmtData) -> Self::Aggregation {
            ExprDepth(0)
        }
    }

    // TODO: refactor it using ItemPath
    #[test]
    fn test_sem_expr_aggregator() {
        DB::ast_rich_test_debug(
            |db, syn_node_region_path: SynNodeRegionPath| {
                let region_path = syn_node_region_path.region_path(db)?;
                let sem_expr_region = sem_expr_region_from_region_path(region_path, db)?;
                let sem_expr_region_data = sem_expr_region.data(db);
                let Some(_) = sem_expr_region_data.opt_root_body() else {
                    return None;
                };
                let context = DepthContext {
                    sem_expr_region_data,
                };

                let mut aggregator = SemExprAggregator::new(context);
                aggregator.aggregate_all();
                let result = aggregator.finish();

                let root_depth = result.expr_aggregations[sem_expr_region_data.root_body()].0;
                Some(root_depth)
            },
            &AstTestConfig::new(
                "sem_expr_aggregator",
                FileExtensionConfig::Markdown,
                TestDomainsConfig::SEMANTICS,
            ),
        )
    }
}
