mod impl_opn;
mod xml;

use husky_primitive_literal_semantics::{
    convert_primitive_literal_to_register, convert_primitive_literal_to_value,
};
pub use xml::*;

use vm::{__Linkage, __Register};

use husky_entity_route::EntityRouteVariant;
use husky_entity_route::{EntityRoutePtr, RangedEntityRoute};
use husky_entity_semantics::*;
use husky_lazy_semantics::*;
use husky_word::RootIdentifier;
use std::sync::Arc;
use vm::{Binding, InstructionSheet, __LinkageFp, __VMResult};

use crate::{eval_id::FeatureEvalId, *};

#[derive(Clone)]
pub struct FeatureExpr {
    pub variant: FeatureExprVariant,
    pub feature: FeaturePtr,
    pub eval_id: FeatureEvalId,
    pub expr: Arc<LazyExpr>,
}

impl std::fmt::Debug for FeatureExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FeatureExpr")
            .field("variant", &self.variant.kind())
            .field("eval_id", &self.eval_id)
            .field("file", &self.expr.file)
            .field("range", &self.expr.range)
            .finish()
    }
}

impl std::hash::Hash for FeatureExpr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.eval_id.hash(state)
    }
}

impl<'eval> PartialEq for FeatureExpr {
    fn eq(&self, other: &Self) -> bool {
        self.eval_id == other.eval_id
    }
}

impl<'eval> Eq for FeatureExpr {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureExprVariant {
    PrimitiveLiteral(__Register<'static>),
    EnumKindLiteral {
        entity_route: EntityRoutePtr,
        uid: EntityUid,
    },
    PrimitiveBinaryOpr {
        opr: PureBinaryOpr,
        opds: Vec<Arc<FeatureExpr>>,
        linkage: __Linkage,
    },
    CustomBinaryOpr {
        opr: PureBinaryOpr,
        opds: Vec<Arc<FeatureExpr>>,
        opt_linkage: Option<__Linkage>,
        opt_instruction_sheet: Option<Arc<InstructionSheet>>,
    },
    Variable {
        varname: CustomIdentifier,
        value: Arc<FeatureExpr>,
    },
    ThisValue {
        repr: FeatureRepr,
    },
    StructOriginalField {
        this: FeatureRepr,
        field_ident: RangedCustomIdentifier,
        field_idx: u8,
        field_binding: Binding,
        opt_linkage: Option<__LinkageFp>,
    },
    RecordOriginalField {
        this: FeatureRepr,
        field_ident: RangedCustomIdentifier,
        repr: FeatureRepr,
    },
    StructDerivedLazyField {
        this: FeatureRepr,
        field_ident: RangedCustomIdentifier,
        field_uid: EntityUid,
        repr: FeatureRepr,
    },
    RecordDerivedField {
        this: FeatureRepr,
        field_ident: RangedCustomIdentifier,
        field_uid: EntityUid,
        repr: FeatureRepr,
    },
    ElementAccess {
        opds: Vec<Arc<FeatureExpr>>,
        linkage: __LinkageFp,
    },
    ModelCall {
        opds: Vec<Arc<FeatureExpr>>,
        has_this: bool,
        model_defn: Arc<EntityDefn>,
        opt_arrival_indicator: Option<Arc<FeatureArrivalIndicator>>,
        internal: __VMResult<__Register<'static>>,
    },
    RoutineCall {
        opds: Vec<Arc<FeatureExpr>>,
        has_this: bool,
        opt_instruction_sheet: Option<Arc<InstructionSheet>>,
        opt_linkage: Option<__Linkage>,
        routine_defn: Arc<EntityDefn>,
    },
    EntityFeature {
        repr: FeatureRepr,
    },
    EvalInput,
    NewRecord {
        ty: RangedEntityRoute,
        entity: Arc<EntityDefn>,
        opds: Vec<Arc<FeatureExpr>>,
    },
    NewVecFromList {
        elements: Vec<Arc<FeatureExpr>>,
        linkage: __Linkage,
    },
}

impl FeatureExprVariant {
    pub fn kind(&self) -> &'static str {
        match self {
            FeatureExprVariant::PrimitiveLiteral(_) => "PrimitiveLiteral",
            FeatureExprVariant::EnumKindLiteral { .. } => "EnumKindLiteral",
            FeatureExprVariant::PrimitiveBinaryOpr { .. } => "PrimitiveBinaryOpr",
            FeatureExprVariant::Variable { .. } => "Variable",
            FeatureExprVariant::ThisValue { .. } => "ThisValue",
            FeatureExprVariant::StructOriginalField { .. } => "StructOriginalFieldAccess",
            FeatureExprVariant::RecordOriginalField { .. } => "RecordOriginalFieldAccess",
            FeatureExprVariant::StructDerivedLazyField { .. } => "StructDerivedFieldAccess",
            FeatureExprVariant::RecordDerivedField { .. } => "RecordDerivedFieldAccess",
            FeatureExprVariant::ElementAccess { .. } => "ElementAccess",
            FeatureExprVariant::ModelCall { .. } => "ModelCall",
            FeatureExprVariant::RoutineCall { .. } => "RoutineCall",
            FeatureExprVariant::EntityFeature { .. } => "EntityFeature",
            FeatureExprVariant::EvalInput => "EvalInput",
            FeatureExprVariant::NewRecord { .. } => "NewRecord",
            FeatureExprVariant::NewVecFromList { .. } => "NewVecFromList",
            FeatureExprVariant::CustomBinaryOpr { .. } => "CustomBinaryOpr",
        }
    }
}

impl FeatureExpr {
    pub fn new(
        db: &(dyn FeatureGenQueryGroup),
        this: Option<FeatureRepr>,
        expr: Arc<LazyExpr>,
        symbols: &[FeatureSymbol],
        opt_arrival_indicator: Option<&Arc<FeatureArrivalIndicator>>,
        interner: &FeatureInterner,
    ) -> Arc<Self> {
        FeatureExprBuilder {
            db,
            symbols,
            features: interner,
            opt_this: this,
            opt_arrival_indicator,
        }
        .new_expr(expr)
    }
}

struct FeatureExprBuilder<'a> {
    db: &'a dyn FeatureGenQueryGroup,
    symbols: &'a [FeatureSymbol],
    features: &'a FeatureInterner,
    opt_this: Option<FeatureRepr>,
    opt_arrival_indicator: Option<&'a Arc<FeatureArrivalIndicator>>,
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
            LazyExprVariant::PrimitiveLiteral(data) => (
                FeatureExprVariant::PrimitiveLiteral(convert_primitive_literal_to_register(
                    data,
                    expr.ty(),
                )),
                self.features.intern(Feature::PrimitiveLiteral(
                    convert_primitive_literal_to_value(data, expr.ty()),
                )),
            ),
            LazyExprVariant::Bracketed(ref bracketed_expr) => {
                return self.new_expr(bracketed_expr.clone())
            }
            LazyExprVariant::Opn { opn_kind, ref opds } => self.compile_opn(opn_kind, opds, &expr),
            LazyExprVariant::Lambda(_, _) => todo!(),
            LazyExprVariant::EnumLiteral { entity_route } => (
                FeatureExprVariant::EnumKindLiteral {
                    entity_route,
                    uid: self.db.compile_time().entity_uid(entity_route),
                },
                self.features.intern(Feature::EnumLiteral(entity_route)),
            ),
            LazyExprVariant::ThisValue { .. } => (
                FeatureExprVariant::ThisValue {
                    repr: self.opt_this.as_ref().unwrap().clone(),
                },
                self.opt_this.as_ref().unwrap().feature(),
            ),
            LazyExprVariant::ThisField {
                field_ident,
                this_ty,
                this_binding,
                field_binding,
            } => {
                let this_repr = self.opt_this.clone().unwrap();
                self.compile_field_access(field_ident, this_repr, field_binding)
            }
            LazyExprVariant::EntityFeature { entity_route } => match entity_route.variant {
                EntityRouteVariant::Root { .. } | EntityRouteVariant::Package { .. } => panic!(),
                EntityRouteVariant::Child { .. } => {
                    let uid = self.db.compile_time().entity_uid(entity_route);
                    let feature = self.features.intern(Feature::EntityFeature {
                        route: entity_route,
                        uid,
                    });
                    let kind = FeatureExprVariant::EntityFeature {
                        repr: self.db.entity_feature_repr(entity_route),
                    };
                    (kind, feature)
                }
                EntityRouteVariant::Input { main } => {
                    let feature = self.features.intern(Feature::Input);
                    let kind = FeatureExprVariant::EvalInput;
                    (kind, feature)
                }
                EntityRouteVariant::Generic { ident, .. } => todo!(),
                EntityRouteVariant::ThisType => todo!(),
                EntityRouteVariant::TypeAsTraitMember { .. } => todo!(),
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
