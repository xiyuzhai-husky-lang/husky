use super::*;

pub(super) enum CollectorAction {}

impl<'a> EntityTreeCollector<'a> {
    pub(super) fn collect_possible_actions(&self) -> Vec<PresheetAction> {
        let mut actions = vec![];
        for presheet in self.presheets.iter() {
            presheet.collect_possible_actions(&mut actions)
        }
        actions
    }
}
