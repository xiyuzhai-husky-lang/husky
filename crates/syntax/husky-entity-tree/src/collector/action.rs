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

    fn context<'b>(&'b self, presheet: &'b EntreePresheetMut<'b>) -> EntreeSymbolContext<'b> {
        let module_path = presheet.module_path();
        EntreeSymbolContext::new(
            self.db,
            module_path,
            presheet.module_specific_symbols(),
            crate_prelude(
                self.opt_universal_prelude,
                self.core_prelude_module,
                &self.presheets,
                self.crate_specific_prelude,
            ),
        )
    }
}
