//! System-related things.

use crate::foundations::{TypstDict, TypstModuleEvaluation, TypstValueAssignmentGroup, Version};

/// A module with system-related things.
pub fn module(inputs: TypstDict) -> TypstModuleEvaluation {
    let mut scope = TypstValueAssignmentGroup::deduplicating();
    scope.define(
        "version",
        Version::from_iter([
            env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap(),
            env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap(),
            env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap(),
        ]),
    );
    scope.define("inputs", inputs);
    TypstModuleEvaluation::new("sys", scope)
}
