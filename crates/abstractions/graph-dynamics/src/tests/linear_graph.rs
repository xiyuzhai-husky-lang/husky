use super::*;
use crate::deps::{IsGraphDepsContext, IsGraphDepsScheme};
use propagate::{PropagationResult, PropagationResultRef};

#[derive(Clone, Copy)]
pub(crate) struct LinearGraphContext<'db> {
    db: &'db ::salsa::Db,
    len: usize,
}

#[salsa::interned]
pub struct LinearGraphNode {
    #[return_ref]
    id: usize,
    len: usize,
}

pub struct LinearGraphScheme;

impl<'db> IsGraphDepsScheme for LinearGraphScheme {
    type Node = LinearGraphNode;
    const CYCLE_GROUP_N: usize = 2;
    type CycleGroupItd = LinearGraphCycleGroupItd;
}

impl<'db> IsGraphDynamicsScheme for LinearGraphScheme {
    type Value = usize;
    const MAX_ITERATION: usize = 1000;
}

impl<'db> IsGraphDepsContext<'db> for LinearGraphContext<'db> {
    type Scheme = LinearGraphScheme;

    fn deps_cropped(self, node: LinearGraphNode) -> impl IntoIterator<Item = LinearGraphNode> {
        let db = self.db;
        let id = *node.id(db);
        if id + 1 < self.len {
            vec![LinearGraphNode::new(db, id + 1, self.len)]
        } else {
            vec![]
        }
    }

    fn full_deps_cropped(self, node: LinearGraphNode) -> &'db [LinearGraphNode] {
        linear_graph_full_deps_cropped(self.db, node)
    }

    fn cycle_group_itd(self, node: LinearGraphNode) -> LinearGraphCycleGroupItd
    where
        [(); <Self::Scheme as IsGraphDepsScheme>::CYCLE_GROUP_N]:,
    {
        linear_graph_cycle_group_itd(self.db, node)
    }
}

impl<'db> IsGraphDynamicsContext<'db> for LinearGraphContext<'db> {
    type Scheme = LinearGraphScheme;

    fn deps_cropped(self, node: LinearGraphNode) -> impl IntoIterator<Item = LinearGraphNode> {
        <Self as IsGraphDepsContext<'db>>::deps_cropped(self, node)
    }

    fn full_deps_cropped(self, node: LinearGraphNode) -> &'db [LinearGraphNode] {
        <Self as IsGraphDepsContext<'db>>::full_deps_cropped(self, node)
    }

    fn cycle_group_itd(self, node: LinearGraphNode) -> LinearGraphCycleGroupItd {
        <Self as IsGraphDepsContext<'db>>::cycle_group_itd(self, node)
    }

    fn initial_value(self, node: LinearGraphNode) -> usize {
        *node.id(self.db)
    }

    fn updated_value<'a>(
        self,
        node: LinearGraphNode,
        query: impl Fn(LinearGraphNode) -> &'a usize,
    ) -> usize {
        // in our case, deps is equal to deps_cropped
        *IsGraphDepsContext::deps_cropped(self, node)
            .into_iter()
            .map(&query)
            .chain([query(node)])
            .max()
            .expect("impossible to be none because of chaining")
    }

    fn cycle_group_values(
        self,
        cycle_group_itd: LinearGraphCycleGroupItd,
    ) -> PropagationResultRef<'db, &'db CycleGroupMap<LinearGraphScheme>> {
        linear_graph_cycle_group_final_values(self.db, cycle_group_itd).as_ref()
    }
}

#[salsa::tracked(return_ref)]
pub fn linear_graph_full_deps_cropped(
    db: &::salsa::Db,
    node: LinearGraphNode,
) -> Vec<LinearGraphNode> {
    let ctx = LinearGraphContext {
        db,
        len: node.len(db),
    };
    ctx.calc_full_deps_cropped(node)
}

#[salsa::interned]
pub struct LinearGraphCycleGroupItd {
    len: usize,
    #[return_ref]
    cycle_group: CycleGroup<LinearGraphScheme>,
}

#[salsa::tracked(return_ref)]
pub fn linear_graph_cycle_group_final_values(
    db: &::salsa::Db,
    cycle_group_itd: LinearGraphCycleGroupItd,
) -> PropagationResult<CycleGroupMap<LinearGraphScheme>> {
    let ctx = LinearGraphContext {
        db,
        len: cycle_group_itd.len(db),
    };
    ctx.calc_cycle_group_final_values(cycle_group_itd.cycle_group(db))
}

#[test]
fn linear_graph_full_deps_cropped_works() {
    #[track_caller]
    fn t(len: usize, id: usize, expected: &[usize], db: &::salsa::Db) {
        let full_reaches = linear_graph_full_deps_cropped(db, LinearGraphNode::new(db, id, len))
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

#[salsa::tracked]
pub fn linear_graph_cycle_group_itd(
    db: &::salsa::Db,
    node: LinearGraphNode,
) -> LinearGraphCycleGroupItd {
    let len = node.len(db);
    let ctx = LinearGraphContext { db, len };
    let cycle_group = ctx.calc_cycle_group(node);
    LinearGraphCycleGroupItd::new(db, len, cycle_group)
}

#[test]
fn linear_graph_final_value_works() {
    #[track_caller]
    fn t(len: usize, id: usize, expected: usize, db: &::salsa::Db) {
        let ctx = LinearGraphContext { db, len };
        let recursion_value = *ctx.value(LinearGraphNode::new(db, id, len)).unwrap();
        assert_eq!(recursion_value, expected);
    }

    let db = &DB::default();
    t(1, 0, 0, db);
    t(2, 0, 1, db);
    t(2, 1, 1, db);
    t(3, 0, 2, db);
    t(4, 0, 3, db);
}
