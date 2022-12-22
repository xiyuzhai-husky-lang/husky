use super::*;

pub(super) enum CollectorAction {}

impl<'a> EntitySymbolCollector<'a> {
    pub(super) fn collect_possible_actions(&self) -> Vec<PresheetAction> {
        let mut actions = vec![];
        let ctx = ModuleSymbolContext::new(self.db);
        for presheet in self.presheets.iter() {
            presheet.collect_possible_actions(&ctx, &mut actions)
        }
        actions
    }
}
