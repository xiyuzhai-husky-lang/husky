use crate::*;
use husky_print_utils::p;

pub(crate) struct PropagationEngine<G: IsGraph> {
    versions: Vec<usize>,
    graph: G,
    max_version: usize,
}

impl<G: IsGraph> PropagationEngine<G> {
    pub(crate) fn new(graph: G) -> Self {
        Self {
            versions: (0..graph.len()).into_iter().map(|_| 1).collect(),
            graph,
            max_version: 1,
        }
    }

    pub(crate) fn update_all(&mut self) {
        for i in 0..self.versions.len() {
            self.try_update(i)
        }
    }

    fn try_update(&mut self, i: usize) {
        if self.should_update(i) {
            let changed = self.graph.update(i);
            if changed {
                self.max_version += 1;
                self.versions[i] = self.max_version
            }
        }
    }

    fn should_update(&mut self, i: usize) -> bool {
        let version = self.versions[i];
        if version == 1 {
            return true;
        }
        for d in self.graph.deps(i) {
            if self.versions[d] >= version {
                return true;
            }
        }
        false
    }

    pub(crate) fn finish(self) -> G {
        self.graph
    }

    pub(crate) fn max_version(&self) -> usize {
        self.max_version
    }
}
