use husky_standard_linket_impl::static_var::StandardStaticVarId;
use husky_trace_protocol::chart::{Chart, ChartDim0, ChartDim1, ChartDim2};

pub type StandardChart<R> = Chart<StandardStaticVarId, R>;
pub type StandardChartDim0<R> = ChartDim0<StandardStaticVarId, R>;
pub type StandardChartDim1<R> = ChartDim1<StandardStaticVarId, R>;
pub type StandardChartDim2<R> = ChartDim2<StandardStaticVarId, R>;
