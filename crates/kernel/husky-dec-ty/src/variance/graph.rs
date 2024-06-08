use super::*;
use husky_entity_path::path::ItemPath;
use vec_like::VecSet;

#[deprecated(note = "use crates/abstractions/graph-dynamics instead")]
#[derive(Debug)]
pub(super) struct VarianceGraph<'a> {
    ids: VecSet<VariancePath>,
    nodes: Vec<VarianceGraphNode<'a>>,
    original_len: usize,
}

impl<'a> IsGraph for VarianceGraph<'a> {
    type Value = Variance;

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn deps(&self, idx: usize) -> impl IntoIterator<Item = usize> {
        self.nodes[idx].deps.iter().copied()
    }

    fn value_mut(&mut self, idx: usize) -> &mut Self::Value {
        &mut self.nodes[idx].value
    }

    fn eval(&self, idx: usize) -> Self::Value {
        let node = &self.nodes[idx];
        let mut value = node.value;
        for _ in &node.deps {
            todo!()
        }
        value
    }
}

impl<'a> VarianceGraph<'a> {
    pub(super) fn new(db: &'a ::salsa::Db, path: ItemPath) -> VarianceResult<Self> {
        let Ok(item_variance_reprs) = item_variance_reprs(db, path) else {
            todo!()
        };
        let mut ids: VecSet<VariancePath> = Default::default();
        let nodes = item_variance_reprs
            .iter()
            .map(|repr| VarianceGraphNode::new(&mut ids, repr))
            .collect();
        let mut this = Self {
            ids,
            nodes,
            original_len: item_variance_reprs.len(),
        };
        this.init();
        Ok(this)
    }

    fn init(&mut self) {
        while self.ids.len() > self.nodes.len() {
            todo!()
        }
    }

    pub(crate) fn finish(&self) -> Vec<Variance> {
        self.nodes[0..self.original_len]
            .iter()
            .map(|node| node.value)
            .collect()
    }
}

#[derive(Debug)]
pub(super) struct VarianceGraphNode<'a> {
    repr: &'a VarianceRepr,
    value: Variance,
    deps: Vec<usize>,
}

impl<'a> VarianceGraphNode<'a> {
    pub(super) fn new(_ids: &mut VecSet<VariancePath>, repr: &'a VarianceRepr) -> Self {
        Self {
            repr,
            value: repr.base(),
            deps: repr.deps().iter().map(|_| todo!()).collect(),
        }
    }
}
