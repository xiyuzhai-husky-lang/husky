use super::*;

pub struct DocTab {
    id: DocId,
}

impl DocTab {
    pub fn id(&self) -> DocId {
        self.id
    }
}
