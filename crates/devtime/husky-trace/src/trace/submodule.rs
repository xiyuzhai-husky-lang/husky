use crate::registry::associated_trace::VoidAssociatedTraceRegistry;

use super::*;
use husky_entity_kind::FugitiveKind;
use husky_entity_path::{ItemPath, MajorItemPath};
use husky_entity_syn_tree::helpers::paths::module_item_paths;
use husky_vfs::SubmodulePath;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db(db = TraceDb)]
pub struct SubmoduleTrace {
    submodule_path: SubmodulePath,
}

impl salsa::AsId for SubmoduleTrace {
    fn as_id(self) -> salsa::Id {
        self.submodule_path.as_id()
    }

    fn from_id(id: salsa::Id) -> Self {
        Self {
            submodule_path: SubmodulePath::from_id(id),
        }
    }
}

impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for SubmoduleTrace
where
    DB: ?Sized + salsa::DbWithJar<VfsJar>,
{
    fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
}

impl SubmoduleTrace {
    pub fn from_submodule_path(submodule_path: SubmodulePath, db: &dyn TraceDb) -> Option<Self> {
        if !submodule_contains_val_item(db, submodule_path) {
            return None;
        }
        Some(Self { submodule_path })
    }

    pub fn view_data(self, db: &dyn TraceDb) -> TraceViewData {
        let tokens = submodule_view_tokens(db, self);
        TraceViewData::new(tokens.data().to_vec(), true)
    }

    pub fn subtraces(self, db: &dyn TraceDb) -> &[Trace] {
        submodule_trace_subtraces(db, self)
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
pub(crate) fn submodule_view_tokens(
    db: &dyn TraceDb,
    submodule_trace: SubmoduleTrace,
) -> TraceViewLines {
    use husky_entity_syn_tree::helpers::tokra_region::HasDeclTokraRegion;
    use husky_entity_syn_tree::HasSynNodePath;
    let submodule_path = submodule_trace.submodule_path;
    let token_idx_range = submodule_path
        .syn_node_path(db)
        .decl_tokra_region_token_idx_range(db);
    TraceViewLines::new(
        submodule_path.parent(db),
        token_idx_range,
        VoidAssociatedTraceRegistry,
        db,
    )
}

#[salsa::tracked(jar = TraceJar, return_ref)]
pub(crate) fn submodule_trace_subtraces(
    db: &dyn TraceDb,
    submodule_trace: SubmoduleTrace,
) -> Vec<Trace> {
    module_item_paths(db, submodule_trace.submodule_path.inner())
        .iter()
        .filter_map(|&item_path| Trace::from_item_path(item_path, db))
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = TraceDb)]
pub enum SubmoduleSubtrace {
    Submodule(SubmoduleTrace),
    ValItem(ValItemTrace),
}

#[salsa::tracked(jar = TraceJar)]
fn submodule_contains_val_item(db: &dyn TraceDb, submodule_path: SubmodulePath) -> bool {
    for &subitem_path in module_item_paths(db, submodule_path.module_path(db)) {
        match subitem_path {
            ItemPath::Submodule(subitem_submodule_path) => {
                if submodule_contains_val_item(db, subitem_submodule_path) {
                    return true;
                }
            }
            ItemPath::MajorItem(MajorItemPath::Fugitive(fugitive_path))
                if fugitive_path.fugitive_kind(db) == FugitiveKind::Val =>
            {
                return true
            }
            _ => (),
        }
    }
    false
}
