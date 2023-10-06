#[derive(Default, Debug, PartialEq, Eq)]
pub struct TraceSettings {}

pub trait HasTraceSettings {
    fn trace_settings(&self) -> &TraceSettings;
}
