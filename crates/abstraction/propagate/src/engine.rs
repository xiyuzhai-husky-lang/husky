use crate::*;

pub(crate) struct PropagationEngine<'a, G: Graph> {
    versions: Vec<usize>,
    graph: &'a mut G,
    max_version: usize,
}

impl<'a, G: Graph> PropagationEngine<'a, G> {
    pub(crate) fn new(graph: &'a mut G) -> Self {
        Self {
            versions: (0..graph.len()).into_iter().map(|_| 1).collect(),
            graph,
            max_version: 1,
        }
    }

    pub(crate) fn update_all(&mut self) {
        for i in 0..self.versions.len() {
            self.update_one(i)
        }
    }

    fn update_one(&mut self, i: usize) {
        let mut try_updating = false;
        let version = self.versions[i];
        // only try updating when at least one of the dependencies
        // is of version no less than current
        for d in self.graph.dependencies(i) {
            if self.versions[*d] >= version {
                try_updating = true;
                break;
            }
        }
        if try_updating {
            let changed = self.graph.update(i);
            if changed {
                self.max_version += 1;
                self.versions[i] = self.max_version
            }
        }
    }

    pub(crate) fn max_version(&self) -> usize {
        self.max_version
    }
}
