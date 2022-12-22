use super::*;

pub(super) enum CollectorAction {}

impl<'a> EntitySymbolCollector<'a> {
    pub(super) fn collect_possible_actions(&self) -> Vec<PresheetAction> {
        let mut actions = vec![];
        for presheet in self.presheets.iter() {
            presheet.collect_possible_actions(self.context(presheet), &mut actions)
        }
        actions
    }

    fn context(&self, presheet: &EntitySymbolPresheet) -> EntitySymbolContext {
        EntitySymbolContext::new(self.db, presheet.module_path(), self.crate_prelude())
    }

    fn crate_prelude(&self) -> CratePrelude<'a> {
        todo!()
    }
}
