mod impl_opn;

use std::sync::Arc;

use file::FilePtr;
use scope::InputPlaceholder;
use scope::{RangedScope, ScopePtr};
use semantics_eager::*;
use semantics_entity::*;
use semantics_lazy::*;
use text::TextRange;
use vm::{
    AnyValueDyn, BinaryOpr, EagerContract, EnumLiteralValue, InstructionSheet, LazyContract,
    MembVarAccessCompiled,
};
use word::BuiltinIdentifier;

use crate::{eval::FeatureEvalId, *};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureExpr {
    pub kind: FeatureExprKind,
    pub(crate) feature: FeaturePtr,
    pub(crate) eval_id: FeatureEvalId,
    pub range: TextRange,
    pub file: FilePtr,
    pub contract: LazyContract,
}

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
    FuncCall {
        ranged_scope: RangedScope,
        inputs: Vec<Arc<FeatureExpr>>,
        uid: EntityUid,
        callee_file: FilePtr,
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        compiled: Option<()>,
        instruction_sheet: Arc<InstructionSheet>,
        stmts: Arc<Vec<Arc<DeclStmt>>>,
    },
    ProcCall {
        ranged_scope: RangedScope,
        inputs: Vec<Arc<FeatureExpr>>,
        uid: EntityUid,
        callee_file: FilePtr,
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        compiled: Option<()>,
        instruction_sheet: Arc<InstructionSheet>,
        stmts: Arc<Vec<Arc<ImprStmt>>>,
    },
    MembVarAccess {
        this: Arc<FeatureExpr>,
        memb_var_ident: CustomIdentifier,
        contract: LazyContract,
        opt_compiled: Option<MembVarAccessCompiled>,
    },
}

impl FeatureExpr {
    pub fn new(
        db: &dyn EntityQueryGroup,
        expr: &LazyExpr,
        symbols: &[FeatureSymbol],
        features: &FeatureUniqueAllocator,
    ) -> Arc<Self> {
        FeatureExprBuilder {
            db,
            symbols,
            features,
        }
        .new_expr(expr)
    }
}

struct FeatureExprBuilder<'a> {
    db: &'a dyn EntityQueryGroup,
    symbols: &'a [FeatureSymbol],
    features: &'a FeatureUniqueAllocator,
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
        };
        Arc::new(FeatureExpr {
            kind,
            feature,
            eval_id: Default::default(),
            range: expr.range,
            file: expr.file,
            contract: expr.contract,
        })
    }
}
