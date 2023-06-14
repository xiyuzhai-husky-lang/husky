use trackable::{TrackableVecChange, *};

use super::*;

impl DeveloperGuiContext {
    pub(crate) fn process_change(&'static self, change: ServerTraceStateChange) {
        // order matters!
        let modified_entries = match change.trace_nodes {
            TrackableVecChange::Incremental {
                new_entries,
                modified_entries,
            } => {
                self.trace_nodes.borrow_mut(file!(), line!()).extend(
                    new_entries
                        .into_iter()
                        .map(|trace_node| TraceNodeState::from_data(self.visibility, trace_node)),
                );
                modified_entries
            }
        };
        match change.subtrace_ids_map {
            TrackableMapChange::None => (),
            TrackableMapChange::Append { new_entries } => {
                self.subtrace_ids_map.borrow_mut(file!(), line!()).extend(
                    new_entries
                        .into_iter()
                        .map(|(k, v)| (k, self.alloc_value(v) as &'static [TraceId])),
                )
            }
        }
        match change.trace_stalks {
            TrackableMapChange::None => (),
            TrackableMapChange::Append { new_entries } => self
                .trace_stalks
                .borrow_mut(file!(), line!())
                .extend(self.alloc_key_value_pairs(new_entries)),
        }
        match change.trace_statss {
            TrackableMapChange::None => (),
            TrackableMapChange::Append { new_entries } => self
                .trace_statss
                .borrow_mut(file!(), line!())
                .extend(self.alloc_key_opt_value_pairs(new_entries)),
        }
        match change.generic_figure_canvases {
            TrackableMapChange::None => (),
            TrackableMapChange::Append { new_entries } => self
                .generic_figure_canvases
                .borrow_mut(file!(), line!())
                .extend(self.alloc_key_value_pairs(new_entries)),
        }
        match change.specific_figure_canvases {
            TrackableMapChange::None => (),
            TrackableMapChange::Append { new_entries } => self
                .specific_figure_canvases
                .borrow_mut(file!(), line!())
                .extend(self.alloc_key_value_pairs(new_entries)),
        }
        match change.figure_controls {
            TrackableMapChange::None => (),
            TrackableMapChange::Append { new_entries } => self
                .figure_controls
                .borrow_mut(file!(), line!())
                .extend(self.alloc_key_signal_pairs(new_entries)),
        }
        match change.presentation {
            TrackableAtomChange::Some(presentation) => self.set_presentation(presentation),
            TrackableAtomChange::None => (),
        }
        match change.root_traces {
            TrackableVecChange::Incremental {
                new_entries,
                modified_entries,
            } => {
                let mut root_traces = self.root_trace_ids_signal.cget();
                root_traces.extend(new_entries);
                if modified_entries.len() > 0 {
                    todo!()
                }
                self.root_trace_ids_signal.set(root_traces)
            }
        }
        // ad hoc to avoid borrow issues
        for (i, new_value) in modified_entries {
            let expansion = self.trace_nodes.borrow(file!(), line!())[i].expansion;
            expansion.set(new_value.expanded);
            let shown = self.trace_nodes.borrow(file!(), line!())[i].shown;
            shown.set(new_value.shown);
        }
        self.update_trace_listing();
    }
}
