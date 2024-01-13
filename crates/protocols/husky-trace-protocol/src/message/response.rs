use super::*;
use crate::{center::TraceCenter, protocol::trivial::TrivialTraceProtocol, view::TraceViewData};

/// message sent from trace client to trace server
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceResponse<TraceProtocol: IsTraceProtocol> {
    Init {
        center: TraceCenter<TraceProtocol>,
    },
    TakeCacheAction {
        cache_actions: smallvec::SmallVec<[TraceCenterAction<TraceProtocol>; 3]>,
    },
    Err(String),
}

#[test]
fn trace_response_ser_then_deser_works() {
    use serde_impl::{json::SerdeJson, IsSerdeImpl};

    type TraceProtocol = TrivialTraceProtocol;

    fn t(response: TraceResponse<TraceProtocol>) {
        let response1: TraceResponse<TraceProtocol> =
            SerdeJson::from_str(&SerdeJson::to_value(&response).unwrap().to_string()).unwrap();
        assert_eq!(response, response1)
    }

    t(TraceResponse::Init {
        center: TraceCenter::new([].into_iter()),
    });
    t(TraceResponse::Init {
        center: TraceCenter::new(
            [(
                TraceId::from_index(0),
                TraceViewData::new(TraceKind::EagerCall, vec![], false),
            )]
            .into_iter(),
        ),
    });
}
