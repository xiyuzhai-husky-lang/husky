use crate::*;

/// a simple graph where the value of each node is
/// - the maximum of its dependencies if it's nonleaf
/// - 0 otherwise
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct MaxGraph {
    nodes: Vec<MaxGraphNode>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct MaxGraphNode {
    value: i32,
    dependencies: Vec<usize>,
}

impl Graph for MaxGraph {
    type Value = i32;

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn value_mut(&mut self, idx: usize) -> &mut Self::Value {
        &mut self.nodes[idx].value
    }

    fn eval(&self, idx: usize) -> Self::Value {
        // take the maximum of its dependencies
        self.nodes[idx]
            .dependencies
            .iter()
            .map(|idx| self.nodes[*idx].value)
            .max()
            .unwrap_or_default()
    }

    fn dependencies(&self, idx: usize) -> &[usize] {
        &self.nodes[idx].dependencies
    }
}

#[test]
fn progation_works() {
    fn t(
        initial_nodes: impl IntoIterator<Item = (i32, Vec<usize>)>,
        final_nodes: impl IntoIterator<Item = (i32, Vec<usize>)>,
    ) {
        let mut graph = MaxGraph {
            nodes: initial_nodes
                .into_iter()
                .map(|(value, dependencies)| MaxGraphNode {
                    value,
                    dependencies,
                })
                .collect(),
        };
        graph.propagate(1000).unwrap();
        let final_graph = MaxGraph {
            nodes: final_nodes
                .into_iter()
                .map(|(value, dependencies)| MaxGraphNode {
                    value,
                    dependencies,
                })
                .collect(),
        };
        assert_eq!(graph, final_graph);
    }

    t([], []);
    t([(0, vec![])], [(0, vec![])]);
    t([(0, vec![]), (0, vec![])], [(0, vec![]), (0, vec![])]);
    t([(0, vec![1]), (0, vec![])], [(0, vec![1]), (0, vec![])]);
    t([(0, vec![1]), (23, vec![])], [(23, vec![1]), (23, vec![])]);
    t(
        [(0, vec![1]), (0, vec![2]), (23, vec![])],
        [(23, vec![1]), (23, vec![2]), (23, vec![])],
    );
}
