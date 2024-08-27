use crate::*;
use husky_item_path_interface::ItemPathIdInterface;
use husky_linket_impl::{
    linket_impl::{IsLinketImpl, LinketImplKiControlFlow, LinketImplTrackedException},
    pedestal::IsPedestal,
};
use husky_trace_protocol::{anchor::Anchor, chart::Chart, protocol::IsTraceProtocol};
use vec_like::SmallVecPairMap;

pub type DevsoulValue<Devsoul> = <<Devsoul as IsDevsoul>::LinketImpl as IsLinketImpl>::Value;
pub type DevsoulStaticVarId<Devsoul> =
    <<<Devsoul as IsDevsoul>::LinketImpl as IsLinketImpl>::Pedestal as IsPedestal>::StaticVarId;
pub type DevsoulStaticVarMap<Devsoul> =
    SmallVecPairMap<ItemPathIdInterface, DevsoulStaticVarId<Devsoul>, 2>;
pub type DevsoulTrackedException<Devsoul> =
    LinketImplTrackedException<<Devsoul as IsDevsoul>::LinketImpl>;
pub type DevsoulValueResult<Devsoul> = LinketImplKiControlFlow<<Devsoul as IsDevsoul>::LinketImpl>;
pub type DevsoulKiControlFlow<Devsoul, C = DevsoulValue<Devsoul>> =
    LinketImplKiControlFlow<<Devsoul as IsDevsoul>::LinketImpl, C>;
pub type DevsoulAnchor<Devsoul> =
    Anchor<<<Devsoul as IsDevsoul>::Pedestal as IsPedestal>::StaticVarId>;
pub type DevsoulCaryatid<Devsoul> =
    <<Devsoul as IsDevsoul>::TraceProtocol as IsTraceProtocol>::Caryatid;
pub type DevsoulFigure<Devsoul> =
    <<Devsoul as IsDevsoul>::TraceProtocol as IsTraceProtocol>::Figure;
pub type DevsoulChart<Devsoul, R> = Chart<DevsoulStaticVarId<Devsoul>, R>;
