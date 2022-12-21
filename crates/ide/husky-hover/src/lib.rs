mod config;
mod db;
mod ir;
mod navigation;
mod runnable;

pub use config::*;
pub use db::*;
pub use navigation::*;
pub use runnable::*;

use husky_vfs::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct HoverResult {
    #[serde(flatten)]
    pub hover: lsp_types::Hover,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<CommandLinkGroup>,
}

// LSP v3.15 Command does not have a `tooltip` field, vscode supports one.
#[derive(Debug, PartialEq, Clone, Default, Deserialize, Serialize)]
pub struct CommandLink {
    #[serde(flatten)]
    pub command: lsp_types::Command,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
}

impl CommandLink {
    fn new_goto_location() -> Self {
        Self {
            command: lsp_types::Command {
                title: "Location".into(),
                command: "husky-analyzer.gotoLocation".into(),
                arguments: None,
            },
            tooltip: Some("goto_location_tooltip".into()),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Deserialize, Serialize)]
pub struct CommandLinkGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub commands: Vec<CommandLink>,
}

impl CommandLink {
    pub(crate) fn show_references() -> Self {
        todo!()
    }
}

impl CommandLinkGroup {
    pub(crate) fn new_goto_types() -> Self {
        Self {
            title: Some("Go to ".into()),
            commands: vec![CommandLink::new_goto_location()],
        }
    }
}
