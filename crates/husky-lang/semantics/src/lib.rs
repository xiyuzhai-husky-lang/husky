#![allow(dead_code, warnings)]
mod config;
mod error;
mod expr;
mod kind;
mod package;
mod query;
mod stmt;

pub use config::Config;
pub use error::{SemanticError, SemanticResult, SemanticResultArc};
pub use expr::{BinaryOpnKind, Expr, ExprKind, Opn};
pub use kind::EntityKind;
pub use package::Package;
pub use query::*;
use scope::ScopePtr;
pub use stmt::{DeclStmt, DeclStmtKind, StrictStmt, StrictStmtKind};

use kind::*;
use std::sync::Arc;
use unique_vector::UniqVec;
use vc::{Uid, VersionControl};

use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    ident: CustomIdentifier,
    kind: Arc<EntityKind>,
    subentities: Arc<Vec<Arc<Entity>>>,
    scope: ScopePtr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityInnerId {
    raw: usize,
}

pub type EntityVersionControl = VersionControl<ScopePtr, EntityKind>;

pub trait ControlEntityVersion {
    fn entity_vc(&self) -> &EntityVersionControl;
    fn entity_uid(&self, scope: ScopePtr) -> Uid {
        self.entity_vc().uid(scope)
    }
}

impl Entity {
    pub fn kind(&self) -> &EntityKind {
        &self.kind
    }

    fn dependees(kind: &EntityKind) -> UniqVec<ScopePtr> {
        match kind {
            EntityKind::Module(_) => Default::default(),
            EntityKind::Feature(_) => todo!(),
            EntityKind::Pattern(_) => todo!(),
            EntityKind::Func(_) => todo!(),
            EntityKind::Proc(_) => todo!(),
            EntityKind::Ty(ty) => match ty.kind {
                ty::TyKind::Enum => todo!(),
                ty::TyKind::Struct { ref memb_vars } => {
                    memb_vars.iter().map(|memb_var| memb_var.ty.scope).into()
                }
            },
        }
    }

    pub(crate) fn new(
        ident: CustomIdentifier,
        kind: EntityKind,
        subentities: Arc<Vec<Arc<Entity>>>,
        scope: ScopePtr,
        vc: &EntityVersionControl,
    ) -> Entity {
        use std::borrow::Borrow;
        let kind = Arc::new(kind);
        vc.update(scope, kind.clone(), &Self::dependees(&kind));
        Self {
            ident,
            kind,
            subentities,
            scope,
        }
    }
}
