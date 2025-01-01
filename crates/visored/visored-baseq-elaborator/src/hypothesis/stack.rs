use super::*;

pub struct VdBaseqHypothesisStack<'sess> {
    hypotheses: Vec<VdBaseqHypothesisIdx<'sess>>,
}

impl<'sess> VdBaseqHypothesisStack<'sess> {
    pub(super) fn new() -> Self {
        Self {
            hypotheses: Vec::new(),
        }
    }
}

impl<'sess> VdBaseqHypothesisStack<'sess> {
    pub fn append(&mut self, h: VdBaseqHypothesisIdx<'sess>) {
        self.hypotheses.push(h);
    }

    pub fn len(&self) -> usize {
        self.hypotheses.len()
    }

    pub fn rollback(&mut self, len: usize) {
        debug_assert!(len <= self.len());
        self.hypotheses.truncate(len);
    }
}
