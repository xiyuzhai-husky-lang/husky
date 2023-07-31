mod impl_opn;
mod xml;

use husky_primitive_literal_semantics::{
    convert_primitive_literal_to_register, convert_primitive_literal_to_value,
};
pub use xml::*;
use EntityPath;

use husky_vm::{__LinkageGroup, __RegistrableSafe, __RegularValue, __VirtualEnum};

use husky_coword::RootBuiltinIdent;
use husky_ethereal_term::EtherealTerm;
use husky_item_semantics::*;
use husky_lazy_semantics::*;
use husky_vm::{Binding, Instructions, __ResolvedLinkage, __VMResult};
use std::sync::Arc;

use crate::{eval_id::FeatureEvalId, *};

#[derive(Clone)]
pub struct FeatureLazyExpr {
    pub variant: FeatureLazyExprVariant,
    pub feature: Val,
    pub eval_id: FeatureEvalId,
    pub expr: Arc<LazyExpr>,
    pub opt_arrival_indicator: Option<ValDomain>,
}

impl HasTextRange for FeatureLazyExpr {
    fn text_range(&self) -> TextRange {
        self.expr.text_range()
    }
}

impl HasSourceRange for FeatureLazyExpr {
    fn source(&self) -> DiffPath {
        self.expr.file
    }
}

impl std::fmt::Debug for FeatureLazyExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FeatureExpr")
            .field("variant", &self.variant.kind())
            .field("eval_id", &self.eval_id)
            .field("file", &self.expr.file)
            .field("range", &self.expr.range)
            .finish()
    }
}

impl std::hash::Hash for FeatureLazyExpr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.eval_id.hash(state)
    }
}

impl PartialEq for FeatureLazyExpr {
    fn eq(&self, other: &Self) -> bool {
        self.eval_id == other.eval_id
    }
}

impl Eq for FeatureLazyExpr {}

#[derive(PartialEq, Eq, Clone)]
pub enum FeatureLazyExprVariant {
    Literal(__RegularValue),
    PrefixOpr {
        opr: PrefixOpr,
        opds: Vec<ValExpr>,
        linkage: __LinkageGroup,
    },
    PrimitiveBinaryOpr {
        opr: BinaryPureClosedOpr,
        opds: Vec<ValExpr>,
        linkage: __LinkageGroup,
    },
    ShortCircuitBinaryOpr {
        opr: BinaryPureClosedOpr,
        opds: Vec<ValExpr>,
    },
    CustomBinaryOpr {
        opr: BinaryPureClosedOpr,
        opds: Vec<ValExpr>,
        opt_linkage: Option<__LinkageGroup>,
        opt_instruction_sheet: Option<InstructionRegion>,
    },
    Variable {
        varname: Ident,
        value: ValExpr,
    },
    ThisValue {
        repr: ValRepr,
    },
    StructOriginalField {
        this: ValRepr,
        field_ident: RangedIdent,
        field_idx: u8,
        field_binding: Binding,
        opt_linkage: Option<__ResolvedLinkage>,
    },
    RecordOriginalField {
        this: ValRepr,
        field_ident: RangedIdent,
        repr: ValRepr,
    },
    StructDerivedLazyField {
        this: ValRepr,
        field_ident: RangedIdent,
        field_uid: EntityUid,
        repr: ValRepr,
    },
    RecordDerivedField {
        this: ValRepr,
        field_ident: RangedIdent,
        field_uid: EntityUid,
        repr: ValRepr,
    },
    Index {
        opds: Vec<ValExpr>,
        linkage: __ResolvedLinkage,
    },
    ModelCall {
        opds: Vec<ValExpr>,
        has_this: bool,
        model_defn: Arc<EntityDefn>,
        opt_arrival_indicator: Option<ValDomain>,
        internal: __VMResult<__RegularValue>,
    },
    RoutineCall {
        opds: Vec<ValExpr>,
        has_this: bool,
        opt_instruction_sheet: Option<InstructionRegion>,
        opt_linkage: Option<__LinkageGroup>,
        routine_defn: Arc<EntityDefn>,
    },
    EntityFeature {
        repr: ValRepr,
    },
    EvalInput,
    NewRecord {
        ty: HirType,
        item: Arc<EntityDefn>,
        opds: Vec<ValExpr>,
    },
    NewVecFromList {
        elements: Vec<ValExpr>,
        linkage: __LinkageGroup,
    },
    BePattern {
        this: ValExpr,
        patt: Arc<PurePattern>,
    },
}

impl std::fmt::Debug for FeatureLazyExprVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeatureLazyExprVariant::Literal(arg0) => f.debug_tuple("Literal").field(arg0).finish(),
            FeatureLazyExprVariant::PrimitiveBinaryOpr { .. } => {
                f.debug_struct("PrimitiveBinaryOpr").finish()
            }
            FeatureLazyExprVariant::CustomBinaryOpr { .. } => {
                f.debug_struct("CustomBinaryOpr").finish()
            }
            FeatureLazyExprVariant::Variable { varname, value } => f
                .debug_struct("Variable")
                .field("varname", varname)
                .field("value", value)
                .finish(),
            FeatureLazyExprVariant::ThisValue { .. } => f.debug_struct("ThisValue").finish(),
            FeatureLazyExprVariant::StructOriginalField { .. } => {
                f.debug_struct("StructOriginalField").finish()
            }
            FeatureLazyExprVariant::RecordOriginalField { .. } => {
                f.debug_struct("RecordOriginalField").finish()
            }
            FeatureLazyExprVariant::StructDerivedLazyField { .. } => {
                f.debug_struct("StructDerivedLazyField").finish()
            }
            FeatureLazyExprVariant::RecordDerivedField { .. } => {
                f.debug_struct("RecordDerivedField").finish()
            }
            FeatureLazyExprVariant::Index { .. } => f.debug_struct("Index").finish(),
            FeatureLazyExprVariant::ModelCall { .. } => f.debug_struct("ModelCall").finish(),
            FeatureLazyExprVariant::RoutineCall { .. } => f.debug_struct("RoutineCall").finish(),
            FeatureLazyExprVariant::EntityFeature { repr } => {
                f.debug_struct("EntityFeature").field("repr", repr).finish()
            }
            FeatureLazyExprVariant::EvalInput => write!(f, "EvalInput"),
            FeatureLazyExprVariant::NewRecord { .. } => f.debug_struct("NewRecord").finish(),
            FeatureLazyExprVariant::NewVecFromList { .. } => {
                f.debug_struct("NewVecFromList").finish()
            }
            FeatureLazyExprVariant::BePattern { .. } => f.debug_struct("BePattern").finish(),
            FeatureLazyExprVariant::Literal(_) => todo!(),
            FeatureLazyExprVariant::PrimitiveBinaryOpr {
                opr: _,
                opds: _,
                linkage: _,
            } => todo!(),
            FeatureLazyExprVariant::ShortCircuitBinaryOpr {
                opr: _kind,
                opds: _,
            } => todo!(),
        }
    }
}

impl FeatureLazyExprVariant {
    pub fn kind(&self) -> &'static str {
        match self {
            FeatureLazyExprVariant::Literal(_) => "Literal",
            FeatureLazyExprVariant::PrimitiveBinaryOpr { .. } => "PrimitiveBinaryOpr",
            FeatureLazyExprVariant::Variable { .. } => "Variable",
            FeatureLazyExprVariant::ThisValue { .. } => "ThisValue",
            FeatureLazyExprVariant::StructOriginalField { .. } => "StructOriginalFieldAccess",
            FeatureLazyExprVariant::RecordOriginalField { .. } => "RecordOriginalFieldAccess",
            FeatureLazyExprVariant::StructDerivedLazyField { .. } => "StructDerivedFieldAccess",
            FeatureLazyExprVariant::RecordDerivedField { .. } => "RecordDerivedFieldAccess",
            FeatureLazyExprVariant::Index { .. } => "Index",
            FeatureLazyExprVariant::ModelCall { .. } => "ModelCall",
            FeatureLazyExprVariant::RoutineCall { .. } => "RoutineCall",
            FeatureLazyExprVariant::EntityFeature { .. } => "EntityFeature",
            FeatureLazyExprVariant::EvalInput => "EvalInput",
            FeatureLazyExprVariant::NewRecord { .. } => "NewRecord",
            FeatureLazyExprVariant::NewVecFromList { .. } => "NewVecFromList",
            FeatureLazyExprVariant::CustomBinaryOpr { .. } => "CustomBinaryOpr",
            FeatureLazyExprVariant::BePattern { .. } => "BePattern",
            FeatureLazyExprVariant::ShortCircuitBinaryOpr {
                opr: _kind,
                opds: _,
            } => "ShortCircuitBinaryOpr",
        }
    }
}

impl FeatureLazyExpr {
    pub fn new(
        db: &(dyn ValReprDb),
        this: Option<ValRepr>,
        expr: Arc<LazyExpr>,
        symbols: &[ValSymbol],
        opt_arrival_indicator: Option<&ValDomain>,
        interner: &FeatureInterner,
    ) -> Arc<Self> {
        FeatureExprBuilder {
            db,
            symbols,
            feature_interner: interner,
            opt_this: this,
            opt_arrival_indicator,
        }
        .new_expr(expr)
    }
}

struct FeatureExprBuilder<'a> {
    db: &'a dyn ValReprDb,
    symbols: &'a [ValSymbol],
    feature_interner: &'a FeatureInterner,
    opt_this: Option<ValRepr>,
    opt_arrival_indicator: Option<&'a ValDomain>,
}

impl<'a> FeatureExprBuilder<'a> {
    fn new_expr(&self, expr: Arc<LazyExpr>) -> ValExpr {
        todo!()
        // let (kind, feature) = match expr.variant {
        //     LazyExprVariant::Variable { varname, .. } => self
        //         .symbols
        //         .iter()
        //         .rev()
        //         .find_map(|symbol| {
        //             if symbol.varname == varname {
        //                 Some((
        //                     FeatureLazyExprVariant::Variable {
        //                         varname,
        //                         value: symbol.value.clone(),
        //                     },
        //                     symbol.feature,
        //                 ))
        //             } else {
        //                 None
        //             }
        //         })
        //         .unwrap(),
        //     LazyExprVariant::PrimitiveLiteral(data) => (
        //         FeatureLazyExprVariant::Literal(convert_primitive_literal_to_register(
        //             data,
        //             expr.intrinsic_ty(),
        //         )),
        //         self.feature_interner.intern(Feature::PrimitiveLiteral(
        //             convert_primitive_literal_to_value(data, expr.intrinsic_ty()),
        //         )),
        //     ),
        //     LazyExprVariant::Bracketed(ref bracketed_expr) => {
        //         return self.new_expr(bracketed_expr.clone())
        //     }
        //     LazyExprVariant::Opn { opn_kind, ref opds } => self.compile_opn(opn_kind, opds, &expr),
        //     LazyExprVariant::Lambda(_, _) => todo!(),
        //     LazyExprVariant::EnumLiteral { item_path } => (
        //         FeatureLazyExprVariant::Literal(
        //             __VirtualEnum {
        //                 kind_idx: self.db.enum_literal_to_i32(item_path),
        //             }
        //             .to_register(),
        //         ),
        //         self.feature_interner
        //             .intern(Feature::EnumLiteral(item_path)),
        //     ),
        //     LazyExprVariant::ThisValue { .. } => (
        //         FeatureLazyExprVariant::ThisValue {
        //             repr: self.opt_this.as_ref().unwrap().clone(),
        //         },
        //         self.opt_this.as_ref().unwrap().feature(),
        //     ),
        //     LazyExprVariant::ThisField {
        //         field_ident,
        //         field_binding,
        //         ..
        //     } => {
        //         let this_repr = self.opt_this.clone().unwrap();
        //         self.compile_field_access(field_ident, this_repr, field_binding)
        //     }
        //     LazyExprVariant::EntityFeature { item_path } => todo!(),
        //     // match item_path.variant {
        //     //     EntityRouteVariant::Root { .. } | EntityRouteVariant::Package { .. } => panic!(),
        //     //     EntityRouteVariant::Child { .. } => {
        //     //         let uid = self.db.item_uid(item_path);
        //     //         let feature = self.feature_interner.intern(Feature::EntityFeature {
        //     //             route: item_path,
        //     //             uid,
        //     //         });
        //     //         let variant = FeatureLazyExprVariant::EntityFeature {
        //     //             repr: self.db.item_feature_repr(item_path),
        //     //         };
        //     //         (variant, feature)
        //     //     }
        //     //     EntityRouteVariant::TargetInputValue => {
        //     //         let feature = self.feature_interner.intern(Feature::Input);
        //     //         let variant = FeatureLazyExprVariant::EvalInput;
        //     //         (variant, feature)
        //     //     }
        //     //     EntityRouteVariant::Any { .. } => todo!(),
        //     //     EntityRouteVariant::ThisType { .. } => todo!(),
        //     //     EntityRouteVariant::TraitForTypeMember { .. } => todo!(),
        //     //     EntityRouteVariant::TargetOutputType => todo!(),
        //     // },
        //     LazyExprVariant::BePattern { ref this, ref patt } => {
        //         let this = self.new_expr(this.clone());
        //         let feature = self.feature_interner.intern(Feature::BePattern {
        //             this: this.feature,
        //             expr_pattern: Feature::intern_expr_pattern(self.feature_interner, patt),
        //         });
        //         let variant = FeatureLazyExprVariant::BePattern {
        //             this,
        //             patt: patt.clone(),
        //         };
        //         (variant, feature)
        //     }
        // };
        // Arc::new(FeatureLazyExpr {
        //     variant: kind,
        //     feature,
        //     eval_id: Default::default(),
        //     expr,
        //     opt_domain_indicator: self.opt_arrival_indicator.map(|indi| indi.clone()),
        // })
    }
}
