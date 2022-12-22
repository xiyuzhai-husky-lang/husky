use super::*;

pub(crate) enum PresheetAction {}

impl EntitySymbolPresheet {
    pub(crate) fn collect_possible_actions(
        &self,
        ctx: &ModuleSymbolContext,
        actions: &mut Vec<PresheetAction>,
    ) {
        for entity_use_tracker in &self.entity_use_trackers {
            if !entity_use_tracker.resolved() {
                let ident = entity_use_tracker.ident();
                if let Some(_) = ctx.get(ident) {
                    todo!()
                }
            }
        }
        for use_all_tracker in &self.use_all_trackers {
            todo!()
        }
    }
}
