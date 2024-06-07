use super::*;

pub(crate) struct LinearGraphContext<'a> {
    db: &'a ::salsa::Db,
    len: usize,
}

const LEN: usize = 10;

#[salsa::interned]
pub struct LinearGraphNode {
    #[return_ref]
    id: usize,
}

impl<'db> IsPathIntegralContext for LinearGraphContext<'db> {
    type Node = LinearGraphNode;

    type Weight = ();

    type Value = usize;

    fn identity_weight(&self) -> Self::Weight {
        ()
    }

    fn compose_weights(&self, (): &Self::Weight, (): &Self::Weight) -> Self::Weight {
        ()
    }

    fn merge_weights(&self, (): &Self::Weight, (): Self::Weight) -> Self::Weight {
        ()
    }

    fn value(&self, node: Self::Node) -> &Self::Value {
        node.id(self.db)
    }

    fn weighted_reaches(&self, node: Self::Node) -> impl IntoIterator<Item = (Self::Node, &())> {
        let db = self.db;
        let id = *node.id(db);
        if id + 1 < self.len {
            vec![(LinearGraphNode::new(db, id + 1), &())]
        } else {
            vec![]
        }
    }

    fn full_reaches(&self, node: Self::Node) -> &[Self::Node] {
        linear_graph_full_reaches(self.db, node)
    }

    fn integrate<'a>(
        &self,
        weighted_values: impl IntoIterator<Item = (&'a Self::Value, Self::Weight)>,
    ) -> Self::Value
    where
        Self::Value: 'a,
    {
        todo!()
    }

    fn center(&self) -> Self::Node {
        todo!()
    }

    fn integrated_value(&self, node: Self::Node) -> &Self::Value {
        todo!()
    }
}

#[salsa::tracked(return_ref)]
pub fn linear_graph_full_reaches(db: &::salsa::Db, node: LinearGraphNode) -> Vec<LinearGraphNode> {
    let ctx = LinearGraphContext { db, len: LEN };
    ctx.calc_full_reaches();
    todo!()
}

#[test]
fn linear_graph_full_reaches_works() {
    fn t(len: usize) {}
}
