use crate::*;
use husky_item_path_interface::ItemPathIdInterface;
use husky_linket_impl::{
    linket_impl::{IsLinketImpl, LinketImplKiControlFlow, LinketImplTrackedException},
    pedestal::{IsPedestal, JointPedestal},
    static_var::StaticVarResult,
};
use husky_trace_protocol::{
    anchor::Anchor,
    chart::{Chart, ChartDim0, ChartDim1, ChartDim2},
    protocol::IsTraceProtocol,
    stalk::TraceStalk,
};
use vec_like::{ordered_small_vec_map::OrderedSmallVecPairMap, SmallVecPairMap};

pub type DevsoulPedestal<Devsoul> = <Devsoul as IsDevsoul>::Pedestal;
pub type DevsoulJointPedestal<Devsoul> = JointPedestal<DevsoulVarId<Devsoul>>;
pub type DevsoulValue<Devsoul> = <<Devsoul as IsDevsoul>::LinketImpl as IsLinketImpl>::Value;
pub type DevsoulVarId<Devsoul> =
    <<<Devsoul as IsDevsoul>::LinketImpl as IsLinketImpl>::Pedestal as IsPedestal>::VarId;
pub type DevsoulStaticVarMap<Devsoul> =
    SmallVecPairMap<ItemPathIdInterface, DevsoulVarId<Devsoul>, 4>;
pub type DevsoulOrderedVarMap<Devsoul> =
    OrderedSmallVecPairMap<ItemPathIdInterface, DevsoulVarId<Devsoul>, 4>;
pub type DevsoulTrackedException<Devsoul> =
    LinketImplTrackedException<<Devsoul as IsDevsoul>::LinketImpl>;
pub type DevsoulValueResult<Devsoul> = LinketImplKiControlFlow<<Devsoul as IsDevsoul>::LinketImpl>;
pub type DevsoulStaticVarResult<Devsoul, T> = StaticVarResult<DevsoulVarId<Devsoul>, T>;
pub type DevsoulKiControlFlow<Devsoul, C = DevsoulValue<Devsoul>> =
    LinketImplKiControlFlow<<Devsoul as IsDevsoul>::LinketImpl, C>;
pub type DevsoulAnchor<Devsoul> = Anchor<<DevsoulPedestal<Devsoul> as IsPedestal>::VarId>;
pub type DevsoulTraceStalk<Devsoul> = TraceStalk<DevsoulVarId<Devsoul>>;
pub type DevsoulCaryatid<Devsoul> =
    <<Devsoul as IsDevsoul>::TraceProtocol as IsTraceProtocol>::Caryatid;
pub type DevsoulFigure<Devsoul> =
    <<Devsoul as IsDevsoul>::TraceProtocol as IsTraceProtocol>::Figure;
pub type DevsoulChart<Devsoul, R> = Chart<DevsoulVarId<Devsoul>, R>;
pub type DevsoulChartDim0<Devsoul, R> = ChartDim0<DevsoulVarId<Devsoul>, R>;
pub type DevsoulChartDim1<Devsoul, R> = ChartDim1<DevsoulVarId<Devsoul>, R>;
pub type DevsoulChartDim2<Devsoul, R> = ChartDim2<DevsoulVarId<Devsoul>, R>;
