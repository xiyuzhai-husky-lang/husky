use super::*;
use crate::trace::{Trace, TraceSelection};
use enum_index::bitset::EnumBitSet;

pub struct TraceView<'a> {
    trace_selection: &'a mut TraceSelection,
}

impl<'a> TraceView<'a> {
    pub fn new(trace_selection: &'a mut TraceSelection) -> Self {
        Self { trace_selection }
    }
}

impl<'a> egui::Widget for TraceView<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        use enum_index::IsEnumIndex;

        ui.vertical(|ui| {
            let set = self.trace_selection.set_mut();
            for trace in Trace::all() {
                let mut selected = set.contains(trace);
                let old_selected = selected;
                ui.radio_value(&mut selected, true, trace.code());
                if selected != old_selected {
                    set.toggle(trace)
                }
                ui.separator();
            }
        })
        .response
    }
}
