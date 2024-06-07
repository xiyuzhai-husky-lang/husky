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
        *weighted_values.into_iter().map(|(v, ())| v).max().unwrap()
    }

    fn center(&self) -> Self::Node {
        self.center
    }

    fn integrated_value(&self, node: Self::Node) -> &Self::Value {
        linear_graph_integrated_value(self.db, node)
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

#[salsa::tracked(return_ref)]
pub fn linear_graph_integrated_value(db: &::salsa::Db, node: LinearGraphNode) -> usize {
    let ctx = LinearGraphContext {
        db,
        len: node.len(db),
        center: node,
    };
    ctx.calc_integrated_value()
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
    t(2, 1, &[1], db);
    t(3, 0, &[0, 1, 2], db);
    t(4, 0, &[0, 1, 2, 3], db);
}

#[test]
fn linear_graph_integrated_value_works() {
    #[track_caller]
    fn t(len: usize, id: usize, expected: usize, db: &::salsa::Db) {
        let integrated_value =
            *linear_graph_integrated_value(db, LinearGraphNode::new(db, id, len));
        assert_eq!(integrated_value, expected);
    }

    let db = &DB::default();
    t(1, 0, 0, db);
    t(2, 0, 1, db);
    t(2, 1, 1, db);
    t(3, 0, 2, db);
    t(4, 0, 3, db);
}
