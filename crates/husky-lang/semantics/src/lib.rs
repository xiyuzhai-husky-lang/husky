#![allow(dead_code, warnings)]
mod config;
mod error;
mod expr;
mod kind;
mod package;
mod query;
mod stmt;

pub use error::{SemanticError, SemanticResult, SemanticResultArc};
pub use expr::{Expr, ExprKind};
pub use package::Package;
pub use query::*;
pub use stmt::{DeclStmt, DeclStmtKind, StrictStmt, StrictStmtKind};

use kind::*;
use std::sync::Arc;

use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    pub(crate) ident: CustomIdentifier,
    pub(crate) kind: EntityKind,
    pub(crate) subentities: Arc<Vec<Arc<Entity>>>,
}

impl Entity {
    pub fn ident(&self) -> CustomIdentifier {
        self.ident
    }

    pub fn kind(&self) -> &EntityKind {
        &self.kind
    }

    pub fn subentities(&self) -> &[Arc<Entity>] {
        &self.subentities
    }

    pub(crate) fn new(
        &self,
        ident: CustomIdentifier,
        kind: EntityKind,
        subentities: Arc<Vec<Arc<Entity>>>,
    ) -> Entity {
        Self {
            ident,
            kind,
            subentities,
        }
    }
}
