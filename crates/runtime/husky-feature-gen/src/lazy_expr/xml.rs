use crate::*;
use husky_lazy_semantics::{XmlExpr, XmlExprVariant};
use husky_vm::__VMResult;
use husky_word::{IdentDict, IdentPairDict};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureXmlExpr {
    pub variant: FeatureXmlExprVariant,
    pub feature: FeaturePtr,
    pub eval_id: FeatureEvalId,
    pub xml_expr: Arc<XmlExpr>,
}

impl FeatureXmlExpr {
    pub fn new(
        db: &dyn FeatureGenQueryGroup,
        this: Option<FeatureRepr>,
        xml_expr: Arc<XmlExpr>,
        symbols: &[FeatureSymbol],
        opt_arrival_indicator: Option<&Arc<FeatureArrivalIndicator>>,
        feature_interner: &FeatureInterner,
    ) -> Arc<Self> {
        let variant = match xml_expr.variant {
            XmlExprVariant::Value(ref value_expr) => {
                FeatureXmlExprVariant::Value(FeatureLazyExpr::new(
                    db,
                    this.clone(),
                    value_expr.clone(),
                    symbols,
                    opt_arrival_indicator,
                    feature_interner,
                ))
            }
            XmlExprVariant::Tag {
                tag_kind,
                ref props,
            } => FeatureXmlExprVariant::Tag {
                tag_kind,
                props: props
                    .iter()
                    .map(|(ident, argument)| {
                        (
                            *ident,
                            FeatureLazyExpr::new(
                                db,
                                this.clone(),
                                argument.clone(),
                                symbols,
                                opt_arrival_indicator,
                                feature_interner,
                            ),
                        )
                    })
                    .collect(),
            },
        };
        Arc::new(FeatureXmlExpr {
            feature: variant.feature(db.feature_interner()),
            variant,
            eval_id: Default::default(),
            xml_expr,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureXmlExprVariant {
    Value(Arc<FeatureLazyExpr>),
    Tag {
        tag_kind: XmlTagKind,
        props: IdentPairDict<Arc<FeatureLazyExpr>>,
    },
}

impl FeatureXmlExprVariant {
    pub fn feature(&self, feature_interner: &FeatureInterner) -> FeaturePtr {
        match self {
            FeatureXmlExprVariant::Value(value) => feature_interner.intern(Feature::XmlFromValue {
                value: value.feature,
            }),
            FeatureXmlExprVariant::Tag { tag_kind, props } => {
                feature_interner.intern(Feature::XmlFromTag {
                    tag_kind: *tag_kind,
                    props: props
                        .iter()
                        .map(|(ident, argument)| (*ident, argument.feature))
                        .collect(),
                })
            }
        }
    }
}
