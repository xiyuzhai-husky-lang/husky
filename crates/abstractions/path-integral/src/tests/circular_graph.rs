use super::*;

pub(crate) struct CircularGraphContext<'a> {
    db: &'a ::salsa::Db,
    len: usize,
    center: CircularGraphNode,
}

#[salsa::interned]
pub struct CircularGraphNode {
    #[return_ref]
    id: usize,
    len: usize,
}

impl<'db> IsPathIntegralContext for CircularGraphContext<'db> {
    type Node = CircularGraphNode;

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
            vec![(CircularGraphNode::new(db, id + 1, self.len), &())]
        } else {
            todo!();
            vec![]
        }
    }

    fn full_reaches(&self, node: Self::Node) -> &[Self::Node] {
        circular_graph_full_reaches(self.db, node)
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
        circular_graph_integrated_value(self.db, node)
    }
}

#[salsa::tracked(return_ref)]
pub fn circular_graph_full_reaches(
    db: &::salsa::Db,
    node: CircularGraphNode,
) -> Vec<CircularGraphNode> {
    let ctx = CircularGraphContext {
        db,
        len: node.len(db),
        center: node,
    };
    ctx.calc_full_reaches()
}

#[salsa::tracked(return_ref)]
pub fn circular_graph_integrated_value(db: &::salsa::Db, node: CircularGraphNode) -> usize {
    let ctx = CircularGraphContext {
        db,
        len: node.len(db),
        center: node,
    };
    ctx.calc_integrated_value()
}

#[test]
#[ignore]
fn circular_graph_full_reaches_works() {
    #[track_caller]
    fn t(len: usize, id: usize, expected: &[usize], db: &::salsa::Db) {
        let full_reaches = circular_graph_full_reaches(db, CircularGraphNode::new(db, id, len))
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
#[ignore]
fn circular_graph_integrated_value_works() {
    #[track_caller]
    fn t(len: usize, id: usize, expected: usize, db: &::salsa::Db) {
        let integrated_value =
            *circular_graph_integrated_value(db, CircularGraphNode::new(db, id, len));
        assert_eq!(integrated_value, expected);
    }

    let db = &DB::default();
    t(1, 0, 0, db);
    t(2, 0, 1, db);
    t(2, 1, 1, db);
    t(3, 0, 2, db);
    t(4, 0, 3, db);
}
