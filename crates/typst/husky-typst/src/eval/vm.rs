use comemo::Tracked;

use crate::engine::TypstEngine;
use crate::eval::FlowEvent;
use crate::foundations::{IntoTypstValue, TypstValueAssignmentGroups};
use crate::syntax::ast::{self, TypstAstNode};
use crate::syntax::TypstSynSpan;
use crate::IsTypstWorld;

/// A virtual machine.
///
/// Holds the state needed to [evaluate](crate::eval::eval()) Typst sources. A
/// new virtual machine is created for each module evaluation and function call.
pub struct Vm<'a> {
    /// The underlying virtual typesetter.
    pub(crate) engine: TypstEngine<'a>,
    /// A control flow event that is currently happening.
    pub(crate) flow: Option<FlowEvent>,
    /// The stack of scopes.
    pub(crate) scopes: TypstValueAssignmentGroups<'a>,
    /// A span that is currently under inspection.
    pub(crate) inspected: Option<TypstSynSpan>,
}

impl<'a> Vm<'a> {
    /// Create a new virtual machine.
    pub fn new(
        engine: TypstEngine<'a>,
        scopes: TypstValueAssignmentGroups<'a>,
        target: TypstSynSpan,
    ) -> Self {
        let inspected = target.id().and_then(|id| engine.tracer.inspected(id));
        Self {
            engine,
            flow: None,
            scopes,
            inspected,
        }
    }

    /// Access the underlying world.
    pub fn world(&self) -> Tracked<'a, dyn IsTypstWorld + 'a> {
        self.engine.world
    }

    /// Define a variable in the current scope.
    pub fn define(&mut self, var: ast::Ident, value: impl IntoTypstValue) {
        let value = value.into_value();
        if self.inspected == Some(var.span()) {
            self.engine.tracer.value(value.clone());
        }
        self.scopes.top.define(var.get().clone(), value);
    }
}
