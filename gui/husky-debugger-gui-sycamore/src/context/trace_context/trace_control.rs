use super::*;

#[derive(Debug, Default)]
pub struct TraceControl {
    pub(super) active_trace_store: Signal<Option<Rc<TraceProps>>>,
    pub(super) expansion_stores: HashMap<TraceId, Signal<bool>>,
    pub(super) shown_stores: HashMap<TraceId, Signal<bool>>,
}

impl TraceControl {
    pub(super) fn init(
        &mut self,
        active_trace: Option<Rc<TraceProps>>,
        expansions: HashMap<TraceId, bool>,
        showns: HashMap<TraceId, bool>,
    ) {
        self.active_trace_store.set(active_trace);
        self.expansion_stores = expansions
            .into_iter()
            .map(|(trace_id, is_expanded)| (trace_id, Signal::new(is_expanded)))
            .collect();
        self.shown_stores = showns
            .into_iter()
            .map(|(trace_id, is_shown)| (trace_id, Signal::new(is_shown)))
            .collect();
    }

    pub(super) fn is_expanded(&self, trace_id: TraceId) -> bool {
        self.expansion_stores[&trace_id].get_cloned()
    }

    pub(super) fn did_toggle_expansion(&mut self, trace_id: TraceId) {
        let is_expanded: &mut bool = &mut self.expansion_stores[&trace_id].modify();
        *is_expanded = !*is_expanded
    }

    pub(super) fn is_shown(&self, trace_id: TraceId) -> bool {
        todo!()
        // return self.shown_stores.get_or(trace_id, false);
    }

    pub(super) fn did_toggle_show(&mut self, trace_id: TraceId) {
        todo!()
        // self.shown_stores.update(id, (shown) => !shown);
    }

    // print_state() {
    //     console.log("trace control:");
    //     console.log("    shown_stores:");
    //     self.shown_stores.print_state(8);
    //     console.log("    expansion_stores:");
    //     self.expansion_stores.print_state(8);
    // }
}
// function load_store_table<T>(value_table: { [id_str: string]: T }): {
//     [id: number]: Writable<T>;
// } {
//     let store_table: { [id: number]: Writable<T> } = {};
//     for (const id_str in value_table.showns) {
//         const id = parseInt(id_str);
//         store_table[id] = writable(value_table[id]);
//     }
//     return store_table;
// }
