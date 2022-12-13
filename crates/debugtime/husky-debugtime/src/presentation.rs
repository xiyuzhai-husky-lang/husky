use crate::*;

impl Debugtime {
    pub fn presentation(&self) -> &Presentation {
        self.state.presentation()
    }

    pub fn set_presentation(
        &mut self,
<<<<<<< HEAD:crates/debugtime/husky-debugtime/src/restriction.rs
        restriction: Presentation,
    ) -> DebugtimeTakeChangeM<DebugtimeStateChange> {
        self.state.set_presentation(restriction)?;
=======
        presentation: Presentation,
    ) -> HuskyDevtimeTakeChangeM<HuskyDevtimeStateChange> {
        self.state.set_presentation(presentation)?;
>>>>>>> cd50934257db08a3571c5f4b8b68528b5962e1a4:crates/debugtime/husky-debugtime/src/presentation.rs
        self.update()?;
        self.take_change()
    }
}
