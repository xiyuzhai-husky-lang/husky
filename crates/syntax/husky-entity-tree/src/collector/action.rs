use super::*;

pub(super) enum CollectorAction {}

impl<'a> EntityTreeCollector<'a> {
    pub(super) fn collect_possible_actions(&self) -> Vec<PresheetAction> {
        let mut actions = vec![];
        for presheet in self.presheets.iter() {
            presheet.collect_possible_actions(self.context(presheet), &mut actions)
        }
        actions
    }

    fn context<'b>(&'b self, presheet: &'b EntityTreePresheet) -> EntitySymbolContext<'b> {
        let module_path = presheet.module_path();
        EntitySymbolContext::new(
            self.db,
            module_path,
            presheet.module_symbols(),
            self.crate_prelude(),
        )
    }

    fn crate_prelude<'b>(&'b self) -> CratePrelude<'b> {
        let universal_prelude = self
            .opt_universal_prelude
            .unwrap_or_else(|| self.presheets[self.core_prelude_module].module_symbols());
        CratePrelude::new(universal_prelude, self.crate_specific_prelude)
    }
}
