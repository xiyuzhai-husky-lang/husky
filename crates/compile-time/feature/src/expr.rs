mod impl_opn;

use entity_route::EntityRouteKind;
use entity_route::{EntityRoutePtr, RangedEntityRoute};
use semantics_entity::*;
use semantics_lazy::*;
use std::sync::Arc;
use vm::{Binding, InstructionSheet, Linkage};
use word::RootIdentifier;

use crate::{eval_id::FeatureEvalId, *};

#[derive(Debug, Clone)]
pub struct FeatureExpr {
    pub variant: FeatureExprVariant,
    pub feature: FeaturePtr,
    pub eval_id: FeatureEvalId,
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
pub enum FeatureExprVariant {
    PrimitiveLiteral(CopyableValue),
    EnumKindLiteral {
        entity_route: EntityRoutePtr,
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
    ThisValue {
        repr: FeatureRepr,
    },
    StructOriginalFieldAccess {
        this: Arc<FeatureExpr>,
        field_ident: RangedCustomIdentifier,
        field_idx: usize,
        field_binding: Binding,
        opt_linkage: Option<Linkage>,
    },
    RecordOriginalFieldAccess {
        this: Arc<FeatureExpr>,
        field_ident: RangedCustomIdentifier,
        repr: FeatureRepr,
    },
    StructDerivedLazyFieldAccess {
        this: Arc<FeatureExpr>,
        field_ident: RangedCustomIdentifier,
        repr: FeatureRepr,
    },
    RecordDerivedFieldAccess {
        this: Arc<FeatureExpr>,
        field_ident: RangedCustomIdentifier,
        repr: FeatureRepr,
    },
    ElementAccess {
        opds: Vec<Arc<FeatureExpr>>,
        linkage: Linkage,
    },
    RoutineCall {
        opds: Vec<Arc<FeatureExpr>>,
        has_this: bool,
        opt_instruction_sheet: Option<Arc<InstructionSheet>>,
        opt_linkage: Option<Linkage>,
        routine_defn: Arc<EntityDefn>,
    },
    PatternCall {},
    EntityFeature {
        entity_route: EntityRoutePtr,
        repr: FeatureRepr,
    },
    EvalInput,
    NewRecord {
        ty: RangedEntityRoute,
        entity: Arc<EntityDefn>,
        opds: Vec<Arc<FeatureExpr>>,
    },
}

impl FeatureExpr {
    pub fn new(
        db: &dyn FeatureGenQueryGroup,
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
    db: &'a dyn FeatureGenQueryGroup,
    symbols: &'a [FeatureSymbol],
    features: &'a FeatureUniqueAllocator,
    this: Option<FeatureRepr>,
}

impl<'a> FeatureExprBuilder<'a> {
    fn new_expr(&self, expr: Arc<LazyExpr>) -> Arc<FeatureExpr> {
        let (kind, feature) = match expr.variant {
            LazyExprVariant::Variable { varname, .. } => self
                .symbols
                .iter()
                .rev()
                .find_map(|symbol| {
                    if symbol.varname == varname {
                        Some((
                            FeatureExprVariant::Variable {
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
            LazyExprVariant::EntityRoute { .. } => todo!(),
            LazyExprVariant::PrimitiveLiteral(value) => (
                FeatureExprVariant::PrimitiveLiteral(value),
                self.features.alloc(Feature::PrimitiveLiteral(value)),
            ),
            LazyExprVariant::Bracketed(ref bracketed_expr) => {
                return self.new_expr(bracketed_expr.clone())
            }
            LazyExprVariant::Opn { opn_kind, ref opds } => self.compile_opn(opn_kind, opds, &expr),
            LazyExprVariant::Lambda(_, _) => todo!(),
            LazyExprVariant::EnumLiteral { entity_route } => (
                FeatureExprVariant::EnumKindLiteral {
                    entity_route,
                    uid: self.db.entity_uid(entity_route),
                },
                self.features.alloc(Feature::EnumLiteral(entity_route)),
            ),
            LazyExprVariant::ThisValue { .. } => (
                FeatureExprVariant::ThisValue {
                    repr: self.this.as_ref().unwrap().clone(),
                },
                self.this.as_ref().unwrap().feature(),
            ),
            LazyExprVariant::ThisField { .. } => todo!(),
            LazyExprVariant::EntityFeature { entity_route } => match entity_route.kind {
                EntityRouteKind::Root { .. } | EntityRouteKind::Package { .. } => panic!(),
                EntityRouteKind::Child { .. } => {
                    let uid = self.db.entity_uid(entity_route);
                    let feature = self.features.alloc(Feature::EntityFeature {
                        route: entity_route,
                        uid,
                    });
                    let kind = FeatureExprVariant::EntityFeature {
                        entity_route,
                        repr: self.db.entity_feature_repr(entity_route).unwrap(),
                    };
                    (kind, feature)
                }
                EntityRouteKind::Input { main } => {
                    let feature = self.features.alloc(Feature::Input);
                    let kind = FeatureExprVariant::EvalInput;
                    (kind, feature)
                }
                EntityRouteKind::Generic { ident, .. } => todo!(),
                EntityRouteKind::ThisType => todo!(),
                EntityRouteKind::TypeAsTraitMember { .. } => todo!(),
            },
        };
        Arc::new(FeatureExpr {
            variant: kind,
            feature,
            eval_id: Default::default(),
            expr,
        })
    }
}
