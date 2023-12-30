use super::*;
use crate::{center::TraceCenter, protocol::trivial::TrivialTraceProtocol, view::TraceViewData};

/// message sent from trace client to trace server
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceResponse<TraceProtocol: IsTraceProtocol> {
    Init {
        cache: TraceCenter<TraceProtocol>,
    },
    TakeCacheAction {
        cache_actions: smallvec::SmallVec<[TraceCacheAction<TraceProtocol>; 3]>,
    },
    Err(String),
}

#[test]
fn trace_response_serde_works() {
    use serde_impl::{json::SerdeJson, IsSerdeImpl};

    type TraceProtocol = TrivialTraceProtocol;
    fn ser_then_deser(response: TraceResponse<TraceProtocol>) {
        let response1: TraceResponse<TraceProtocol> =
            SerdeJson::from_str(&SerdeJson::to_value(&response).unwrap().to_string()).unwrap();
        assert_eq!(response, response1)
    }
    fn deser_then_ser(s: &str) {
        let s1 =
            SerdeJson::to_value(SerdeJson::from_str::<TraceResponse<TraceProtocol>>(s).unwrap())
                .unwrap()
                .to_string();
        assert_eq!(s, s1)
    }

    ser_then_deser(TraceResponse::Init {
        cache: TraceCenter::new([].into_iter()),
    });
    ser_then_deser(TraceResponse::Init {
        cache: TraceCenter::new(
            [(
                TraceId::from_index(0),
                TraceViewData::new(TraceKind::EagerCall, vec![], false),
            )]
            .into_iter(),
        ),
    });

    deser_then_ser("{\"Init\":{\"cache\":{\"pedestal\":{\"Specific\":0},\"root_trace_ids\":[{\"value\":1},{\"value\":2},{\"value\":3}],\"entries\":{\"entries\":[[{\"value\":1},{\"view_data\":{\"trace_kind\":\"Submodule\",\"lines_data\":[{\"tokens_data\":[{\"text\":\"mod\",\"token_class\":\"OtherKeyword\",\"spaces_before\":0,\"associated_trace_id\":null},{\"text\":\"digits\",\"token_class\":\"ModuleEntity\",\"spaces_before\":1,\"associated_trace_id\":null}]}],\"have_subtraces\":true},\"subtrace_ids\":null,\"associated_trace_ids\":{\"data\":[]},\"expanded\":false}],[{\"value\":2},{\"view_data\":{\"trace_kind\":\"Submodule\",\"lines_data\":[{\"tokens_data\":[{\"text\":\"mod\",\"token_class\":\"OtherKeyword\",\"spaces_before\":0,\"associated_trace_id\":null},{\"text\":\"major\",\"token_class\":\"ModuleEntity\",\"spaces_before\":1,\"associated_trace_id\":null}]}],\"have_subtraces\":true},\"subtrace_ids\":null,\"associated_trace_ids\":{\"data\":[]},\"expanded\":false}],[{\"value\":3},{\"view_data\":{\"trace_kind\":\"ValItem\",\"lines_data\":[{\"tokens_data\":[{\"text\":\"val\",\"token_class\":\"OtherKeyword\",\"spaces_before\":0,\"associated_trace_id\":null},{\"text\":\"main\",\"token_class\":\"ValEntity\",\"spaces_before\":1,\"associated_trace_id\":null},{\"text\":\":\",\"token_class\":\"Punctuation\",\"spaces_before\":0,\"associated_trace_id\":null},{\"text\":\"Class\",\"token_class\":\"TypeEntity\",\"spaces_before\":1,\"associated_trace_id\":null},{\"text\":\"MnistLabel\",\"token_class\":\"TypeEntity\",\"spaces_before\":1,\"associated_trace_id\":null},{\"text\":\"=\",\"token_class\":\"Punctuation\",\"spaces_before\":1,\"associated_trace_id\":null}]}],\"have_subtraces\":true},\"subtrace_ids\":null,\"associated_trace_ids\":{\"data\":[]},\"expanded\":false}]]},\"visual_components\":[],\"actions\":[]}}}")
}
