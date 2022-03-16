mod kind;
mod query;

pub use kind::{EntityKind, Main};
pub use query::{EntityQueryGroup, EntityQueryGroupStorage};

use file::FilePtr;
use kind::*;
use scope::{InputPlaceholder, RangedScope, ScopePtr};
use semantics_eager::*;
use std::sync::Arc;
use text::TextRange;
use unique_vector::UniqVec;
use vc::{Uid, VersionControl};
use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    pub ident: CustomIdentifier,
    pub kind: Arc<EntityKind>,
    pub subentities: Arc<Vec<Arc<Entity>>>,
    pub scope: ScopePtr,
    pub file: FilePtr,
    pub range: TextRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityInnerId {
    raw: usize,
}

pub type EntityVersionControl = VersionControl<ScopePtr, EntityKind>;
pub type EntityUid = Uid;

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
        return match kind {
            EntityKind::Module(_) => Default::default(),
            EntityKind::Feature(_) => todo!(),
            EntityKind::Pattern(_) => todo!(),
            EntityKind::Func {
                input_placeholders: inputs,
                output,
                stmts,
            } => {
                let mut v = UniqVec::new();
                extract_routine_head_dependees(inputs, output, &mut v);
                extract_decl_stmts_dependees(stmts, &mut v);
                v
            }
            EntityKind::Proc {
                input_placeholders: inputs,
                output,
                stmts,
            } => {
                let mut v = UniqVec::new();
                extract_routine_head_dependees(inputs, output, &mut v);
                extract_impr_stmts_dependees(stmts, &mut v);
                v
            }
            EntityKind::Ty(ty) => match ty.kind {
                ty::TyKind::Enum => todo!(),
                ty::TyKind::Struct { ref memb_vars } => {
                    memb_vars.iter().map(|memb_var| memb_var.ty.scope).into()
                }
            },
            EntityKind::Main(_) => todo!(),
        };

        fn extract_routine_head_dependees(
            inputs: &[InputPlaceholder],
            output: &RangedScope,
            v: &mut UniqVec<ScopePtr>,
        ) {
            for input_placeholder in inputs.iter() {
                v.push(input_placeholder.ranged_ty.scope)
            }
            v.push(output.scope);
        }

        fn extract_decl_stmts_dependees(stmts: &[Arc<DeclStmt>], v: &mut UniqVec<ScopePtr>) {
            for stmt in stmts {
                match stmt.kind {
                    DeclStmtKind::Init { varname, ref value } => extract_expr_dependees(value, v),
                    DeclStmtKind::Assert { ref condition } => extract_expr_dependees(condition, v),
                    DeclStmtKind::Return { ref result } => extract_expr_dependees(result, v),
                    DeclStmtKind::Branches { kind, ref branches } => {
                        for branch in branches {
                            extract_decl_stmts_dependees(&branch.stmts, v)
                        }
                    }
                }
            }
        }

        fn extract_impr_stmts_dependees(stmts: &[Arc<ImprStmt>], v: &mut UniqVec<ScopePtr>) {
            for stmt in stmts {
                match stmt.kind {
                    ImprStmtKind::Init {
                        varname,
                        ref initial_value,
                        ..
                    } => extract_expr_dependees(initial_value, v),
                    ImprStmtKind::Assert { ref condition } => extract_expr_dependees(condition, v),
                    ImprStmtKind::Return { ref result } => extract_expr_dependees(result, v),
                    ImprStmtKind::Execute { ref expr } => extract_expr_dependees(expr, v),
                    ImprStmtKind::BranchGroup { kind, ref branches } => {
                        for branch in branches {
                            extract_impr_stmts_dependees(&branch.stmts, v)
                        }
                    }
                    ImprStmtKind::Loop {
                        ref loop_kind,
                        ref stmts,
                    } => {
                        match loop_kind {
                            LoopKind::For {
                                ref initial_boundary,
                                ref final_boundary,
                                ..
                            } => {
                                extract_boundary_dependees(initial_boundary, v);
                                extract_boundary_dependees(final_boundary, v);
                            }
                            LoopKind::ForExt {
                                ref final_boundary, ..
                            } => {
                                extract_boundary_dependees(final_boundary, v);
                            }
                            LoopKind::While { condition } => extract_expr_dependees(condition, v),
                            LoopKind::DoWhile { condition } => extract_expr_dependees(condition, v),
                        }
                        extract_impr_stmts_dependees(stmts, v)
                    }
                }
            }
        }

        fn extract_expr_dependees(expr: &EagerExpr, v: &mut UniqVec<ScopePtr>) {
            match expr.kind {
                EagerExprKind::Variable(_) => (),
                EagerExprKind::Scope { scope, compiled } => v.push(scope),
                EagerExprKind::Literal(_) => (),
                EagerExprKind::Bracketed(ref expr) => extract_expr_dependees(expr, v),
                EagerExprKind::Opn {
                    opn_kind: ref opn,
                    compiled,
                    ref opds,
                } => match opn {
                    EagerOpnKind::Binary { opr, this } => v.push(*this),
                    EagerOpnKind::Prefix(_) => todo!(),
                    EagerOpnKind::Suffix(_) => todo!(),
                    EagerOpnKind::RoutineCall(routine) => v.push(routine.scope),
                    EagerOpnKind::PatternCall => todo!(),
                    EagerOpnKind::MembVarAccess => todo!(),
                    EagerOpnKind::MembFuncCall(_) => todo!(),
                    EagerOpnKind::ElementAccess => todo!(),
                    EagerOpnKind::TypeCall(_) => todo!(),
                },
                EagerExprKind::Lambda(_, _) => todo!(),
            }
        }

        fn extract_boundary_dependees(boundary: &Boundary, v: &mut UniqVec<ScopePtr>) {
            boundary
                .opt_bound
                .as_ref()
                .map(|bound| extract_expr_dependees(bound, v));
        }
    }

    pub(crate) fn new(
        ident: CustomIdentifier,
        kind: EntityKind,
        subentities: Arc<Vec<Arc<Entity>>>,
        scope: ScopePtr,
        file: FilePtr,
        range: TextRange,
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
            file,
            range,
        }
    }
}
