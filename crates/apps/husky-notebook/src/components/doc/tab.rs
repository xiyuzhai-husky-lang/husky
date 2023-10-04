use super::*;

pub struct DocTab {
    id: DocId,
}

impl DocTab {
    pub(super) fn new(id: DocId) -> Self {
        Self { id }
    }

    pub fn id(&self) -> DocId {
        self.id
    }
}
