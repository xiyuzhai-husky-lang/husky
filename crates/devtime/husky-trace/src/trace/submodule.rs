use crate::*;
use husky_vfs::SubmodulePath;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubmoduleTrace {
    submodule_path: SubmodulePath,
}

impl SubmoduleTrace {
    pub fn subtraces(self, db: &dyn TraceDb) -> Option<&[SubmoduleSubtrace]> {
        todo!()
    }
}

fn submodule_subtraces(db: &dyn TraceDb, module_path: ModulePath) -> Vec<SubmoduleSubtrace> {
    todo!()
}

pub enum SubmoduleSubtrace {
    Submodule(SubmoduleTrace),
    ValItem(ValItemTrace),
}
