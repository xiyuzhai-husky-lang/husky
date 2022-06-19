use super::*;
use sycamore::prelude::Signalable;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct FigureControlData {
    opt_mutation_selection: Option<u8>,
}
impl Signalable for FigureControlData {}

impl FigureControlData {
    pub fn loop_default(loop_trace_props: &TraceData) -> Self {
        todo!()
        // let control_props = match loop_trace_props.kind {
        //     TraceKind::ProcStmt {
        //         ref stmt,
        //         ref history,
        //     } => match history.get(stmt).unwrap() {
        //         HistoryEntry::Loop { mutations, .. } => Self::mutations_default(mutations),
        //         _ => panic!(),
        //     },
        //     _ => panic!(),
        // };
        // control_props
    }

    // pub fn mutations_default(mutations: &[MutationData]) -> Self {
    //     let opt_mutation_selection = if mutations.len() > 0 { Some(0) } else { None };
    //     FigureControlData {
    //         opt_mutation_selection,
    //     }
    // }
}
