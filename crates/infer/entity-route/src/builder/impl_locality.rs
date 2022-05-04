use super::*;

impl<'a> EntityRouteSheetBuilder<'a> {
    pub(super) fn enter_block(&mut self) {
        self.trait_uses.enter()
    }
    pub(super) fn exit_block(&mut self) {
        self.trait_uses.exit()
    }
}
