use super::*;

#[derive(Debug)]
pub struct TraceNode {
    pub(crate) trace: Arc<Trace>,
    pub(crate) expansion: bool,
    pub(crate) shown: bool,
}
