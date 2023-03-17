#[derive(Debug, PartialEq, Eq)]
pub enum ManifestAstError {}

pub type ManifestAstResult<T> = Result<T, ManifestAstError>;
