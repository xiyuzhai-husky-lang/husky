use super::*;

pub(crate) struct LinearGraphContext<'a> {
    db: &'a ::salsa::Db,
    len: usize,
    center: LinearGraphNode,
}

#[salsa::interned]
pub struct LinearGraphNode {
    #[return_ref]
    id: usize,
    len: usize,
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
            vec![(LinearGraphNode::new(db, id + 1, self.len), &())]
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
        self.center
    }

    fn integrated_value(&self, node: Self::Node) -> &Self::Value {
        todo!()
    }
}

#[salsa::tracked(return_ref)]
pub fn linear_graph_full_reaches(db: &::salsa::Db, node: LinearGraphNode) -> Vec<LinearGraphNode> {
    let ctx = LinearGraphContext {
        db,
        len: node.len(db),
        center: node,
    };
    ctx.calc_full_reaches()
}

#[test]
fn linear_graph_full_reaches_works() {
    #[track_caller]
    fn t(len: usize, id: usize, expected: &[usize], db: &::salsa::Db) {
        let full_reaches = linear_graph_full_reaches(db, LinearGraphNode::new(db, id, len))
            .iter()
            .map(|reach| *reach.id(db))
            .collect::<Vec<_>>();
        assert_eq!(full_reaches, expected);
    }

    let db = &DB::default();
    t(1, 0, &[0], db);
    t(2, 0, &[0, 1], db);
}
