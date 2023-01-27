use super::*;
use vec_like::VecSet;

pub(super) struct VarianceGraph<'a> {
    ids: VecSet<VarianceId>,
    nodes: Vec<VarianceGraphNode<'a>>,
    original_len: usize,
}

impl<'a> Graph for VarianceGraph<'a> {
    type Value = Variance;

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn dependencies(&self, idx: usize) -> &[usize] {
        todo!()
    }

    fn value_mut(&mut self, idx: usize) -> &mut Self::Value {
        todo!()
    }

    fn eval(&self, idx: usize) -> Self::Value {
        todo!()
    }
}

impl<'a> VarianceGraph<'a> {
    pub(super) fn new(db: &'a dyn TypeDb, path: EntityPath) -> VarianceResult<Self> {
        let Ok(entity_variance_reprs) = entity_variance_reprs(db, path)
            else {
                todo!()
            };
        let mut ids = Default::default();
        let mut nodes = vec![];
        for repr in entity_variance_reprs {
            todo!()
        }
        Ok(Self {
            ids,
            nodes,
            original_len: entity_variance_reprs.len(),
        })
    }

    pub(crate) fn finish(&self) -> Vec<Variance> {
        self.nodes[0..self.original_len]
            .iter()
            .map(|node| node.value)
            .collect()
    }
}

pub(super) struct VarianceGraphNode<'a> {
    repr: &'a VarianceRepr,
    value: Variance,
}

impl<'a> VarianceGraphNode<'a> {
    pub(super) fn new(node: &'a VarianceRepr) -> Self {
        Self {
            repr: node,
            value: node.base(),
        }
    }
}
