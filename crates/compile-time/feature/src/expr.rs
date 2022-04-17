mod impl_opn;

use entity_route::EntityRouteKind;
use entity_route::{EntityRoutePtr, RangedEntityRoute};
use file::FilePtr;
use semantics_eager::*;
use semantics_entity::*;
use semantics_lazy::*;
use std::sync::Arc;
use text::TextRange;
use vm::{EnumLiteralValue, InstructionSheet, LazyContract, Linkage};
use word::{ContextualIdentifier, RootIdentifier};

use crate::{eval::FeatureEvalId, *};

#[derive(Debug, Clone)]
pub struct FeatureExpr {
    pub kind: FeatureExprKind,
    pub(crate) feature: FeaturePtr,
    pub(crate) eval_id: FeatureEvalId,
    pub expr: Arc<LazyExpr>,
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
    StructFieldAccess {
        this: Arc<FeatureExpr>,
        field_ident: RangedCustomIdentifier,
        field_idx: usize,
        contract: LazyContract,
        opt_linkage: Option<Linkage>,
    },
    RecordFieldAccess {
        this: Arc<FeatureExpr>,
        field_ident: RangedCustomIdentifier,
        repr: FeatureRepr,
    },
    RoutineCall {
        opds: Vec<Arc<FeatureExpr>>,
        instruction_sheet: Arc<InstructionSheet>,
        opt_linkage: Option<Linkage>,
        routine_defn: Arc<EntityDefn>,
    },
    PatternCall {},
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
        expr: Arc<LazyExpr>,
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
    fn new_expr(&self, expr: Arc<LazyExpr>) -> Arc<FeatureExpr> {
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
            LazyExprKind::EntityFeature { route } => match route.kind {
                EntityRouteKind::Root { .. } | EntityRouteKind::Package { .. } => panic!(),
                EntityRouteKind::Child { .. } => {
                    let uid = self.db.entity_uid(route);
                    let feature = self.features.alloc(Feature::EntityFeature { route, uid });
                    let kind = FeatureExprKind::FeatureBlock {
                        scope: route,
                        block: self.db.scoped_feature_block(route).unwrap(),
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
                EntityRouteKind::TraitMember {
                    ty: parent,
                    trai,
                    ident,
                } => todo!(),
            },
        };
        Arc::new(FeatureExpr {
            kind,
            feature,
            eval_id: Default::default(),
            expr,
        })
    }
}
