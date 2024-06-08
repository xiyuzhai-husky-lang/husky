use super::*;

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

impl<'db> IsGraphRecursionScheme for LinearGraphScheme {
    type Node = LinearGraphNode;
    type Value = usize;
    const CYCLE_GROUP_N: usize = 2;
    type CycleGroupItd = LinearGraphCycleGroupItd;
}

impl<'db> IsGraphRecursionContext<'db> for LinearGraphContext<'db> {
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
        todo!()
    }

    fn cycle_group_itd(self, node: LinearGraphNode) -> LinearGraphCycleGroupItd {
        todo!()
    }

    fn initial_value(self, node: LinearGraphNode) -> usize {
        todo!()
    }

    fn updated_value(
        self,
        node: LinearGraphNode,
        query: impl Fn(LinearGraphNode) -> usize,
    ) -> usize {
        // in our case, deps is equal to deps_cropped
        self.deps_cropped(node)
            .into_iter()
            .map(&query)
            .chain([query(node)])
            .max()
            .expect("impossible to be none because of chaining")
    }

    fn cycle_group_values(
        self,
        cycle_group_itd: LinearGraphCycleGroupItd,
    ) -> &'db CycleGroupMap<LinearGraphScheme> {
        linear_graph_cycle_group_final_values(self.db, cycle_group_itd)
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
    cycle_group: CycleGroup<LinearGraphScheme>,
}

#[salsa::tracked(return_ref)]
pub fn linear_graph_cycle_group_final_values(
    db: &::salsa::Db,
    cycle_group_itd: LinearGraphCycleGroupItd,
) -> CycleGroupMap<LinearGraphScheme> {
    // let ctx = LinearGraphContext {
    //     db,
    //     len: node.len(db),
    // };
    // ctx.calc_value(cycle_group)
    // ctx.calc_integrated_value()
    todo!()
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

#[test]
#[ignore]
fn linear_graph_final_value_works() {
    #[track_caller]
    fn t(len: usize, id: usize, expected: usize, db: &::salsa::Db) {
        let ctx = LinearGraphContext { db, len };
        let recursion_value = *ctx.value(LinearGraphNode::new(db, id, len));
        assert_eq!(recursion_value, expected);
    }

    let db = &DB::default();
    t(1, 0, 0, db);
    t(2, 0, 1, db);
    t(2, 1, 1, db);
    t(3, 0, 2, db);
    t(4, 0, 3, db);
}
