use super::*;

impl TracerContextInternal {
    pub(super) fn handle_server_message_str(&self, server_message_str: &str) {
        self.handle_server_message(serde_json::from_str(server_message_str).unwrap())
    }

    fn handle_server_message(&self, server_message: DebuggerServerMessage) {
        if let Some(request_id) = server_message.opt_request_id {
            self.call_backs.borrow_mut().remove(&request_id).unwrap()(self, server_message)
        } else {
            match server_message.variant {
                DebuggerServerMessageVariant::Init { init_data } => self.init(init_data),
                DebuggerServerMessageVariant::Trace { trace_props } => todo!(),
                DebuggerServerMessageVariant::DecodeFocus { focus_result } => todo!(),
                DebuggerServerMessageVariant::LockFocus {
                    focus,
                    opt_active_trace_id_for_figure,
                    opt_figure,
                    opt_figure_control,
                } => todo!(),
                DebuggerServerMessageVariant::TraceStalk { stalk } => todo!(),
                _ => panic!(),
            }
        }
    }
}
