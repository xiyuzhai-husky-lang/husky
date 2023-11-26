use super::*;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = EntitySynTreeDb, jar = EntitySynTreeJar)]
pub enum EntityTreeBundleError {
    #[error("from toolchain error")]
    Toolchain(#[from] ToolchainError),
    #[error("from prelude error")]
    Prelude(#[from] PreludeError),
}

pub type EntitySynTreeBundleResult<T> = Result<T, EntityTreeBundleError>;
pub type EntityTreeBundleResultRef<'a, T> = Result<T, &'a EntityTreeBundleError>;

impl From<&EntityTreeBundleError> for EntityTreeBundleError {
    fn from(_value: &EntityTreeBundleError) -> Self {
        todo!()
    }
}

impl From<&PreludeError> for EntityTreeBundleError {
    fn from(_value: &PreludeError) -> Self {
        todo!()
    }
}
