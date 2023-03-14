use super::*;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum EntityTreeBundleError {
    #[error("from toolchain error")]
    Toolchain(#[from] ToolchainError),
    #[error("from prelude error")]
    Prelude(#[from] PreludeError),
}

pub type EntityTreeBundleResult<T> = Result<T, EntityTreeBundleError>;
pub type EntityTreeBundleResultRef<'a, T> = Result<T, &'a EntityTreeBundleError>;

impl From<&EntityTreeBundleError> for EntityTreeBundleError {
    fn from(_value: &EntityTreeBundleError) -> Self {
        todo!()
    }
}
