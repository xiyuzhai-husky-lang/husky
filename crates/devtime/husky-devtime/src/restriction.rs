use crate::*;

impl HuskyDevtime {
    pub fn presentation(&self) -> &Presentation {
        self.state.presentation()
    }

    pub fn set_restriction(
        &mut self,
        restriction: Presentation,
    ) -> HuskyDevtimeTakeChangeM<HuskyDevtimeStateChange> {
        self.state.set_presentation(restriction)?;
        self.update()?;
        self.take_change()
    }
}
