use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct EntityPathMenu0 {
    // library
    core: EntityPathItd,
    std: EntityPathItd,
}

impl EntityPathMenu0 {
    pub(crate) fn new(db: &dyn EntityPathDb) -> Self {
        Self {
            core: todo!(),
            std: todo!(),
        }
    }

    pub fn core(&self) -> EntityPathItd {
        self.core
    }
}
