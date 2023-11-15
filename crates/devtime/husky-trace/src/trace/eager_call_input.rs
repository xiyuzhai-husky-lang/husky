use super::*;
use husky_hir_eager_expr::HirEagerCallListItemGroup;

#[salsa::interned(db = TraceDb, jar = TraceJar, constructor = new)]
pub struct EagerCallInputTracePath {
    pub biological_parent_path: EagerCallInputTraceBiologicalParentPath,
    #[return_ref]
    pub data: EagerCallInputTracePathData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum EagerCallInputTraceBiologicalParentPath {
    EagerExpr(EagerExprTracePath),
    LazyExpr(LazyExprTracePath),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum EagerCallInputTracePathData {
    Haha,
}

#[salsa::tracked(db = TraceDb, jar = TraceJar, constructor = new_inner)]
pub struct EagerCallInputTrace {
    #[id]
    pub path: EagerCallInputTracePath,
    pub biological_parent: EagerCallInputTraceBiologicalParent,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum EagerCallInputTraceBiologicalParent {
    EagerExpr(EagerExprTrace),
    LazyExpr(LazyExprTrace),
}

impl EagerCallInputTrace {
    pub(crate) fn new(
        biological_parent_path: impl Into<EagerCallInputTraceBiologicalParentPath>,
        biological_parent: impl Into<EagerCallInputTraceBiologicalParent>,
        i: usize,
        item_group: &HirEagerCallListItemGroup,
        db: &dyn TraceDb,
    ) -> Self {
        Self::new_inner(
            db,
            EagerCallInputTracePath::new(
                db,
                biological_parent_path.into(),
                EagerCallInputTracePathData::Haha,
            ),
            biological_parent.into(),
        )
    }

    pub fn view_data(self, _db: &dyn TraceDb) -> TraceViewData {
        todo!()
    }

    pub fn subtraces(self, _db: &dyn TraceDb) -> &[Trace] {
        todo!()
    }

    pub fn val_repr(self, _db: &dyn TraceDb) -> ValRepr {
        todo!()
    }
}
