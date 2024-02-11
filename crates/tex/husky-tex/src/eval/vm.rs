use comemo::Tracked;

use crate::engine::TexEngine;
use crate::eval::FlowEvent;
use crate::foundations::{IntoTexValue, TexValueAssignmentGroups};
use crate::syntax::ast::{self, TexAstNode};
use crate::syntax::Span;
use crate::IsTexWorld;

/// A virtual machine.
///
/// Holds the state needed to [evaluate](crate::eval::eval()) Tex sources. A
/// new virtual machine is created for each module evaluation and function call.
pub struct Vm<'a> {
    /// The underlying virtual typesetter.
    pub(crate) engine: TexEngine<'a>,
    /// A control flow event that is currently happening.
    pub(crate) flow: Option<FlowEvent>,
    /// The stack of scopes.
    pub(crate) scopes: TexValueAssignmentGroups<'a>,
    /// A span that is currently under inspection.
    pub(crate) inspected: Option<Span>,
}

impl<'a> Vm<'a> {
    /// Create a new virtual machine.
    pub fn new(engine: TexEngine<'a>, scopes: TexValueAssignmentGroups<'a>, target: Span) -> Self {
        let inspected = target.id().and_then(|id| engine.tracer.inspected(id));
        Self {
            engine,
            flow: None,
            scopes,
            inspected,
        }
    }

    /// Access the underlying world.
    pub fn world(&self) -> Tracked<'a, dyn IsTexWorld + 'a> {
        self.engine.world
    }

    /// Define a variable in the current scope.
    pub fn define(&mut self, var: ast::Ident, value: impl IntoTexValue) {
        let value = value.into_value();
        if self.inspected == Some(var.span()) {
            self.engine.tracer.value(value.clone());
        }
        self.scopes.top.define(var.get().clone(), value);
    }
}
