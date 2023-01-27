use super::*;

pub(super) struct VarianceGraph<'a> {
    db: &'a dyn TypeDb,
    /// holds at most 64 nodes
    nodes: Vec<VarianceGraphNode<'a>>,
    /// holds at most 64 nodes
    changes: BitSet64,
}

#[derive(Debug, Clone, Copy)]
struct BitSet64(u64);

impl BitSet64 {
    fn is_empty(self) -> bool {
        self.0 != 0
    }

    fn get(self, idx: usize) -> bool {
        assert!(idx < 64);
        self.0 & (1 << idx) == 0
    }
}

impl<'a> VarianceGraph<'a> {
    pub(super) fn new(db: &'a dyn TypeDb, path: EntityPath) -> VarianceResult<Self> {
        todo!()
        // let Ok(entity_variance_nodes) = entity_variance_nodes(db, path)
        //     else {
        //         todo!()
        //     };
        // Ok(Self {
        //     db,
        //     nodes: entity_variance_nodes
        //         .iter()
        //         .map(|node| VarianceGraphNode::new(node))
        //         .collect(),
        //     version: 1,
        // })
    }

    pub(super) fn has_changes(&self) -> bool {
        self.changes.is_empty()
    }
}

pub(super) struct VarianceGraphNode<'a> {
    node: &'a VarianceRepr,
    variance: Variance,
}

impl<'a> VarianceGraphNode<'a> {
    pub(super) fn new(node: &'a VarianceRepr) -> Self {
        Self {
            node,
            variance: node.base(),
        }
    }
}
