mod data;
mod id;
mod stalk;
mod stats;
mod token;
mod tree;

pub use self::data::*;
pub use self::id::*;
pub use self::stalk::*;
pub use self::stats::*;
pub use self::token::*;
pub use self::tree::*;
use husky_feature_protocol::FeatureId;

use super::*;

pub type Indent = u8;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TraceKind {
    Main,
    Module,
    EntityFeatureLazy,
    EntityFeatureEager,
    FeatureStmt,
    FeatureBranch,
    FeatureExprLazy,
    FeatureExprEager,
    FeatureCallArgument,
    FuncStmt,
    EagerStmt,
    EagerBranch,
    FuncBranch,
    LoopFrame,
    EagerExpr,
    EagerCallArgument,
    CallHead,
}

impl TraceKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            TraceKind::Main => "Main",
            TraceKind::Module => "Module",
            TraceKind::EntityFeatureLazy => "EntityFeatureLazy",
            TraceKind::EntityFeatureEager => "EntityFeatureEager",
            TraceKind::FeatureStmt => "FeatureStmt",
            TraceKind::FeatureBranch => "FeatureBranch",
            TraceKind::FeatureExprLazy => "FeatureExprLazy",
            TraceKind::FeatureExprEager => "FeatureExprEager",
            TraceKind::FeatureCallArgument => "FeatureCallArgument",
            TraceKind::FuncStmt => "FuncStmt",
            TraceKind::EagerStmt => "ProcStmt",
            TraceKind::EagerBranch => "ProcBranch",
            TraceKind::FuncBranch => "FuncBranch",
            TraceKind::LoopFrame => "LoopFrame",
            TraceKind::EagerExpr => "EagerExpr",
            TraceKind::EagerCallArgument => "EagerCallArgument",
            TraceKind::CallHead => "CallHead",
        }
    }
    pub fn can_have_stalk(self) -> bool {
        match self {
            TraceKind::Main
            | TraceKind::EntityFeatureLazy
            | TraceKind::EntityFeatureEager
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FeatureExprLazy
            | TraceKind::FeatureExprEager => true,
            TraceKind::Module
            | TraceKind::FeatureCallArgument
            | TraceKind::EagerCallArgument
            | TraceKind::FuncStmt
            | TraceKind::EagerStmt
            | TraceKind::EagerBranch
            | TraceKind::FuncBranch
            | TraceKind::LoopFrame
            | TraceKind::EagerExpr
            | TraceKind::CallHead => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceLineData {
    pub indent: Indent,
    pub tokens: Vec<TraceTokenData>,
    pub idx: usize,
}
