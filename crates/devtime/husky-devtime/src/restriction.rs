use crate::*;

impl Devtime {
    pub fn presentation(&self) -> &Presentation {
        self.state.presentation()
    }

    pub fn set_presentation(&mut self, presentation: Presentation) -> HuskyDevtimeStateChange {
        self.state.set_presentation(presentation);
        self.update();
        self.take_change()
    }
}
