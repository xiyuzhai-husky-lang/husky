use super::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Runnable {
    pub use_name_in_title: bool,
    pub nav: NavigationTarget,
    pub kind: RunnableKind,
    pub cfg: Option<CfgExpr>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RunnableKind {}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CfgExpr {}
