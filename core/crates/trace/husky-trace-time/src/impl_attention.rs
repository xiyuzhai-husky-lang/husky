use crate::*;
use text::Text;
use vm::{History, VMControl};

impl HuskyTraceTime {
    pub fn set_attention(&mut self, attention: Attention) {
        self.attention = attention;
    }
}
