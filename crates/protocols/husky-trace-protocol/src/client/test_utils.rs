use super::{server::IsTracetime, *};
use mock::MOCK_SERVER;

pub trait TestTraceClient: IsTracetime {
    fn test_trace_client(self);
}

impl<T> TestTraceClient for T
where
    T: IsTracetime,
{
    fn test_trace_client(self) {
        let tokio_runtime = Arc::new(tokio::runtime::Runtime::new().unwrap());
        let server_thread = std::thread::spawn(|| self.serve_traces(MOCK_SERVER));
        let mut client = TraceClient::<T::TraceProtocol, ()>::new_mock(tokio_runtime, ());
        client.take_view_action(TraceViewAction::FollowTrace {
            trace_id: TraceId::from_index(0),
        });
        server_thread.join();
    }
}
