use crate::*;

impl<Task: IsTask> Devtime<Task> {
    pub fn presentation(&self) -> &Presentation {
        self.state.presentation()
    }

    pub fn set_presentation(&mut self, presentation: Presentation) -> DevtimeStateChange {
        self.state.set_presentation(presentation);
        self.update();
        self.take_change()
    }
}
