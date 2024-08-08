use super::{server::IsTracetime, *};

pub trait TestTraceClient: IsTracetime {
    fn test_trace_client(self);
}

impl<T> TestTraceClient for T
where
    T: IsTracetime,
{
    fn test_trace_client(self) {
        let tokio_runtime = Arc::new(tokio::runtime::Runtime::new().unwrap());
        let server_thread = std::thread::spawn(|| self.serve_traces("localhost:51718"));
        let wait_time = std::time::Duration::from_millis(10);
        // wait until the server is there for sure
        std::thread::sleep(wait_time);
        let mut client =
            TraceClient::<T::TraceProtocol, ()>::new(tokio_runtime, "ws://localhost:51718/ws", ());
        std::thread::sleep(wait_time);
        client.update(&mut None);
        // std::thread::sleep(hundred_millis);
        // client.take_view_action(TraceViewAction::FollowTrace {
        //     trace_id: TraceId::from_index(0),
        // });
        // server_thread.join();
    }
}
