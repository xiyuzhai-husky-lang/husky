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

    pub(super) fn context<'b>(
        &'b self,
        presheet: &'b EntityTreePresheetMut<'a>,
    ) -> EntityTreeSymbolContext<'a, 'b> {
        let _module_path = presheet.module_path();
        EntityTreeSymbolContext::new(
            self.db,
            self.crate_path,
            self.crate_root_path,
            CratePrelude::new(self.db, self.crate_path).expect("todo"),
            presheet,
            &self.presheets,
        )
    }
}

#[macro_use]
macro_rules! context {
    ($self: ident, $presheet: expr) => {{
        EntityTreeSymbolContext::new(
            $self.db,
            $self.crate_path,
            $self.crate_root_path,
            CratePrelude::new($self.db, $self.crate_path).expect("todo"),
            $presheet,
            &$self.presheets,
        )
    }};
}
pub(super) use context;
