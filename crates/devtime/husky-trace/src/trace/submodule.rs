use super::*;
use husky_entity_kind::FugitiveKind;
use husky_entity_path::{ItemPath, MajorItemPath};
use husky_entity_syn_tree::helpers::paths::module_item_paths;
use husky_vfs::SubmodulePath;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub fn view_data<'a>(self, db: &'a dyn TraceDb) -> &'a TraceViewData {
        submodule_view_data(db, self)
    }

    pub fn subtraces(self, db: &dyn TraceDb) -> Option<&[SubmoduleSubtrace]> {
        todo!()
    }
}

#[salsa::tracked(jar = TraceJar, return_ref)]
pub(crate) fn submodule_view_data(
    db: &dyn TraceDb,
    submodule_trace: SubmoduleTrace,
) -> TraceViewData {
    let tokens_data = todo!();
    TraceViewData::new(tokens_data)
}

#[salsa::tracked(jar = TraceJar, return_ref)]
pub(crate) fn submodule_subtraces(
    db: &dyn TraceDb,
    submodule_trace: SubmoduleTrace,
) -> Vec<SubmoduleSubtrace> {
    todo!()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = TraceDb)]
pub enum SubmoduleSubtrace {
    Submodule(SubmoduleTrace),
    ValItem(ValItemTrace),
}

#[salsa::tracked(jar = TraceJar)]
fn submodule_contains_val_item(db: &dyn TraceDb, submodule_path: SubmodulePath) -> bool {
    let Ok(subitem_paths) = module_item_paths(db, submodule_path.module_path(db)) else {
        unreachable!()
    };
    for &subitem_path in subitem_paths {
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
