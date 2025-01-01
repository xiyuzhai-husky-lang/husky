use super::*;

pub struct VdBaseqHypothesisStack {
    hypotheses: Vec<VdBaseqHypothesisIdx>,
}

impl VdBaseqHypothesisStack {
    pub fn append(&mut self, h: VdBaseqHypothesisIdx) {
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
