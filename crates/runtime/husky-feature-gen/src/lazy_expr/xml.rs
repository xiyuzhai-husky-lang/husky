use crate::*;
use husky_coword::IdentPairMap;
use husky_lazy_semantics::{HtmlExpr, HtmlExprVariant};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureHtmlExpr {
    pub variant: FeatureHtmlExprVariant,
    pub feature: FeatureItd,
    pub eval_id: FeatureEvalId,
    pub xml_expr: Arc<HtmlExpr>,
}

impl FeatureHtmlExpr {
    pub fn new(
        db: &dyn FeatureGenQueryGroup,
        this: Option<FeatureRepr>,
        xml_expr: Arc<HtmlExpr>,
        symbols: &[FeatureSymbol],
        opt_arrival_indicator: Option<&Arc<FeatureDomainIndicator>>,
        feature_interner: &FeatureInterner,
    ) -> Arc<Self> {
        let variant = match xml_expr.variant {
            HtmlExprVariant::Value(ref value_expr) => {
                FeatureHtmlExprVariant::Value(FeatureLazyExpr::new(
                    db,
                    this.clone(),
                    value_expr.clone(),
                    symbols,
                    opt_arrival_indicator,
                    feature_interner,
                ))
            }
            HtmlExprVariant::Tag {
                tag_kind,
                ref props,
            } => FeatureHtmlExprVariant::Tag {
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
        Arc::new(FeatureHtmlExpr {
            feature: variant.feature(db.feature_interner()),
            variant,
            eval_id: Default::default(),
            xml_expr,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureHtmlExprVariant {
    Value(Arc<FeatureLazyExpr>),
    Tag {
        tag_kind: HtmlTagKind,
        props: IdentPairMap<Arc<FeatureLazyExpr>>,
    },
}

impl FeatureHtmlExprVariant {
    pub fn feature(&self, feature_interner: &FeatureInterner) -> FeatureItd {
        match self {
            FeatureHtmlExprVariant::Value(value) => {
                feature_interner.intern(Feature::HtmlFromValue {
                    value: value.feature,
                })
            }
            FeatureHtmlExprVariant::Tag { tag_kind, props } => {
                feature_interner.intern(Feature::HtmlFromTag {
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
