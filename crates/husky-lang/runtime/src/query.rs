use feature::FeatureBlock;

use crate::*;

pub trait AskCompileTime {
    fn compile_time(&self, version: usize) -> &HuskyLangCompileTime;
}

#[salsa::query_group(RuntimeQueryGroupStorage)]
pub trait RuntimeQueryGroup: AskCompileTime + AllocateTrace {
    #[salsa::input]
    fn package_main(&self) -> FilePtr;

    #[salsa::input]
    fn version(&self) -> usize;

    fn subtraces(&self, id: TraceId, input_locked_on: Option<usize>) -> Arc<Vec<Arc<Trace>>>;
    fn root_traces(&self) -> Arc<Vec<Arc<Trace>>>;
}

pub fn root_traces(this: &dyn RuntimeQueryGroup) -> Arc<Vec<Arc<Trace>>> {
    let compile_time = this.compile_time(this.version());
    let package_main = this.package_main();
    Arc::new(vec![this.new_trace(
        None,
        package_main,
        0,
        TraceKind::Main(compile_time.main_feature_block(package_main).unwrap()),
    )])
}

pub fn subtraces(
    this: &dyn RuntimeQueryGroup,
    id: TraceId,
    input_locked_on: Option<usize>,
) -> Arc<Vec<Arc<Trace>>> {
    let trace: &Trace = &this.trace(id);
    match trace.kind {
        TraceKind::Main(ref block) => Arc::new(this.feature_block_subtraces(&trace, block)),
        TraceKind::FeatureStmt(ref stmt) => Arc::new(vec![]),
        TraceKind::FeatureExpr(ref expr) => this
            .trace_allocator()
            .feature_expr_subtraces(trace, expr, None),
        TraceKind::FeatureBranch(ref branch) => this.feature_branch_subtraces(trace, branch),
    }
}
