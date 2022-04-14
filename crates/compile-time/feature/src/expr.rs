mod impl_opn;

use entity_route::EntityRouteKind;
use entity_route::{EntityRoutePtr, RangedEntityRoute};
use file::FilePtr;
use semantics_eager::*;
use semantics_entity::*;
use semantics_lazy::*;
use std::sync::Arc;
use text::TextRange;
use vm::{EnumLiteralValue, InstructionSheet, LazyContract, MembAccessFp, RoutineLinkage};
use word::{ContextualIdentifier, RootIdentifier};

use crate::{eval::FeatureEvalId, *};

#[derive(Debug, Clone)]
pub struct FeatureExpr {
    pub kind: FeatureExprKind,
    pub(crate) feature: FeaturePtr,
    pub(crate) eval_id: FeatureEvalId,
    pub range: TextRange,
    pub file: FilePtr,
    pub contract: LazyContract,
    pub ty: EntityRoutePtr,
}

impl std::hash::Hash for FeatureExpr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.eval_id.hash(state)
    }
}

impl PartialEq for FeatureExpr {
    fn eq(&self, other: &Self) -> bool {
        self.eval_id == other.eval_id
    }
}

impl Eq for FeatureExpr {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureExprKind {
    PrimitiveLiteral(PrimitiveValue),
    EnumLiteral {
        value: EnumLiteralValue,
        uid: EntityUid,
    },
    PrimitiveBinaryOpr {
        opr: PureBinaryOpr,
        lopd: Arc<FeatureExpr>,
        ropd: Arc<FeatureExpr>,
    },
    Variable {
        varname: CustomIdentifier,
        value: Arc<FeatureExpr>,
    },
    This {
        repr: FeatureRepr,
    },
    FuncCall {
        func_ranged_scope: RangedEntityRoute,
        inputs: Vec<Arc<FeatureExpr>>,
        uid: EntityUid,
        callee_file: FilePtr,
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        compiled: Option<RoutineLinkage>,
        instruction_sheet: Arc<InstructionSheet>,
        stmts: Arc<Vec<Arc<FuncStmt>>>,
    },
    ProcCall {
        proc_ranged_scope: RangedEntityRoute,
        inputs: Vec<Arc<FeatureExpr>>,
        uid: EntityUid,
        callee_file: FilePtr,
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        opt_compiled: Option<RoutineLinkage>,
        instruction_sheet: Arc<InstructionSheet>,
        stmts: Arc<Vec<Arc<ProcStmt>>>,
    },
    StructMembVarAccess {
        this: Arc<FeatureExpr>,
        field_ident: RangedCustomIdentifier,
        field_idx: usize,
        contract: LazyContract,
        opt_compiled: Option<MembAccessFp>,
    },
    RecordMembAccess {
        this: Arc<FeatureExpr>,
        field_ident: RangedCustomIdentifier,
        repr: FeatureRepr,
    },
    MethodCall {
        field_ident: CustomIdentifier,
        opds: Vec<Arc<FeatureExpr>>,
        instruction_sheet: Arc<InstructionSheet>,
        opt_compiled: Option<RoutineLinkage>,
        stmts: Arc<Vec<Arc<FuncStmt>>>,
    },
    MembProcCall {
        field_ident: RangedCustomIdentifier,
        opds: Vec<Arc<FeatureExpr>>,
        instruction_sheet: Arc<InstructionSheet>,
        opt_compiled: Option<RoutineLinkage>,
        stmts: Arc<Vec<Arc<ProcStmt>>>,
    },
    MembPattCall {
        field_ident: RangedCustomIdentifier,
        opds: Vec<Arc<FeatureExpr>>,
        instruction_sheet: Arc<InstructionSheet>,
        stmts: Arc<Vec<Arc<ProcStmt>>>,
    },
    FeatureBlock {
        scope: EntityRoutePtr,
        block: Arc<FeatureBlock>,
    },
    GlobalInput,
    NewRecord {
        ty: RangedEntityRoute,
        entity: Arc<EntityDefn>,
        opds: Vec<Arc<FeatureExpr>>,
    },
}

impl FeatureExpr {
    pub fn new(
        db: &dyn FeatureQueryGroup,
        this: Option<FeatureRepr>,
        expr: &LazyExpr,
        symbols: &[FeatureSymbol],
        features: &FeatureUniqueAllocator,
    ) -> Arc<Self> {
        FeatureExprBuilder {
            db,
            symbols,
            features,
            this,
        }
        .new_expr(expr)
    }
}

struct FeatureExprBuilder<'a> {
    db: &'a dyn FeatureQueryGroup,
    symbols: &'a [FeatureSymbol],
    features: &'a FeatureUniqueAllocator,
    this: Option<FeatureRepr>,
}

impl<'a> FeatureExprBuilder<'a> {
    fn new_expr(&self, expr: &LazyExpr) -> Arc<FeatureExpr> {
        let (kind, feature) = match expr.kind {
            LazyExprKind::Variable(varname) => self
                .symbols
                .iter()
                .rev()
                .find_map(|symbol| {
                    if symbol.varname == varname {
                        Some((
                            FeatureExprKind::Variable {
                                varname,
                                value: symbol.value.clone(),
                            },
                            symbol.feature,
                        ))
                    } else {
                        None
                    }
                })
                .unwrap(),
            LazyExprKind::Scope { scope, compiled } => todo!(),
            LazyExprKind::PrimitiveLiteral(value) => (
                FeatureExprKind::PrimitiveLiteral(value),
                self.features.alloc(Feature::PrimitiveLiteral(value)),
            ),
            LazyExprKind::Bracketed(_) => todo!(),
            LazyExprKind::Opn {
                opn_kind,
                compiled,
                ref opds,
            } => self.new_opn(opn_kind, compiled, opds, expr.contract),
            LazyExprKind::Lambda(_, _) => todo!(),
            LazyExprKind::EnumLiteral { scope, ref value } => (
                FeatureExprKind::EnumLiteral {
                    value: value.clone(),
                    uid: self.db.entity_uid(scope),
                },
                self.features.alloc(Feature::EnumLiteral(scope)),
            ),
            LazyExprKind::This => (
                FeatureExprKind::This {
                    repr: self.this.as_ref().unwrap().clone(),
                },
                self.this.as_ref().unwrap().feature(),
            ),
            LazyExprKind::ScopedFeature { scope } => match scope.kind {
                EntityRouteKind::Root { .. } | EntityRouteKind::Package { .. } => panic!(),
                EntityRouteKind::ChildScope { .. } => {
                    let uid = self.db.entity_uid(scope);
                    let feature = self.features.alloc(Feature::ScopedFeature { scope, uid });
                    let kind = FeatureExprKind::FeatureBlock {
                        scope,
                        block: self.db.scoped_feature_block(scope).unwrap(),
                    };
                    (kind, feature)
                }
                EntityRouteKind::Input { main } => {
                    let feature = self.features.alloc(Feature::Input);
                    let kind = FeatureExprKind::GlobalInput;
                    (kind, feature)
                }
                EntityRouteKind::Generic { ident, .. } => todo!(),
                EntityRouteKind::ThisType => todo!(),
            },
        };
        Arc::new(FeatureExpr {
            kind,
            feature,
            eval_id: Default::default(),
            range: expr.range,
            file: expr.file,
            contract: expr.contract,
            ty: expr.ty,
        })
    }
}
