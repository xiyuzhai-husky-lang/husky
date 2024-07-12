use husky_standard_devsoul_interface::pedestal::StandardPedestal;
use husky_trace_protocol::{figure::IsFigure, protocol::IsTraceProtocol};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct StandardTraceProtocol<Figure: IsFigure<StandardPedestal>>(PhantomData<Figure>);

impl<Figure: IsFigure<StandardPedestal>> Default for StandardTraceProtocol<Figure> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<Figure> IsTraceProtocol for StandardTraceProtocol<Figure>
where
    Figure: IsFigure<StandardPedestal>,
{
    type Pedestal = StandardPedestal;

    type Figure = Figure;
}
