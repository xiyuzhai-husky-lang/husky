use crate::*;

impl Debugtime {
    pub fn presentation(&self) -> &Presentation {
        self.state.presentation()
    }

    pub fn set_restriction(
        &mut self,
        restriction: Presentation,
    ) -> DebugtimeTakeChangeM<DebugtimeStateChange> {
        self.state.set_presentation(restriction)?;
        self.update()?;
        self.take_change()
    }
}
