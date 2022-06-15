use crate::*;
use text::Text;
use vm::{History, VMControl};

impl HuskyTraceTime {
    pub fn set_focus(&mut self, focus: Focus) {
        self.focus = focus;
    }
}
