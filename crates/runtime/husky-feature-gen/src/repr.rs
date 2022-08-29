use husky_entity_semantics::DefinitionRepr;
use husky_file::FilePtr;
use husky_instruction_gen::new_func_instruction_sheet;
use husky_linkage_table::ResolveLinkage;
use husky_vm::__Register;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FeatureRepr {
    Value {
        value: __Register<'static>,
        ty: EntityRoutePtr,
        file: FilePtr,
        range: TextRange,
        feature: FeaturePtr,
    },
    LazyExpr(Arc<FeatureLazyExpr>),
    LazyBlock(Arc<FeatureLazyBlock>),
    FuncBlock(Arc<FeatureFuncBlock>),
    ProcBlock(Arc<FeatureProcBlock>),
    TargetInput {
        main_file: FilePtr,
        ty: EntityRoutePtr,
        feature: FeaturePtr,
    },
}

impl FeatureRepr {
    pub fn is_lazy(&self) -> bool {
        match self {
            FeatureRepr::LazyExpr(_) => true,
            FeatureRepr::LazyBlock(_) => true,
            FeatureRepr::Value { .. } => false,
            FeatureRepr::FuncBlock(_) => false,
            FeatureRepr::ProcBlock(_) => false,
            FeatureRepr::TargetInput { .. } => false,
        }
    }

    pub fn opt_leading_keyword(&self) -> Option<&'static str> {
        match self {
            FeatureRepr::Value { .. } => panic!(),
            FeatureRepr::LazyExpr(_) => Some("def "),
            FeatureRepr::LazyBlock(_) => Some("def "),
            FeatureRepr::FuncBlock(_) => Some("func "),
            FeatureRepr::ProcBlock(_) => Some("proc "),
            FeatureRepr::TargetInput { .. } => None,
        }
    }

    pub fn ty(&self) -> EntityRoutePtr {
        match self {
            FeatureRepr::Value { ty, .. } => *ty,
            FeatureRepr::LazyExpr(expr) => expr.expr.intrinsic_ty(),
            FeatureRepr::LazyBlock(block) => block.return_ty.route,
            FeatureRepr::FuncBlock(block) => block.ty.route,
            FeatureRepr::ProcBlock(block) => block.ty.route,
            FeatureRepr::TargetInput { ty, .. } => *ty,
        }
    }
    pub fn feature(&self) -> FeaturePtr {
        match self {
            FeatureRepr::Value { feature, .. } => *feature,
            FeatureRepr::LazyExpr(expr) => expr.feature,
            FeatureRepr::LazyBlock(block) => block.feature,
            FeatureRepr::FuncBlock(block) => block.feature,
            FeatureRepr::ProcBlock(block) => block.feature,
            FeatureRepr::TargetInput { feature, .. } => *feature,
        }
    }

    pub fn file(&self) -> FilePtr {
        match self {
            FeatureRepr::Value { file, .. } => *file,
            FeatureRepr::LazyExpr(expr) => expr.expr.file,
            FeatureRepr::LazyBlock(block) => block.file,
            FeatureRepr::FuncBlock(block) => block.file,
            FeatureRepr::ProcBlock(block) => block.file,
            FeatureRepr::TargetInput { main_file, .. } => *main_file,
        }
    }

    pub fn text_range(&self) -> TextRange {
        match self {
            FeatureRepr::Value { range, .. } => *range,
            FeatureRepr::LazyExpr(expr) => expr.expr.range,
            FeatureRepr::LazyBlock(block) => block.range,
            FeatureRepr::FuncBlock(block) => block.range,
            FeatureRepr::ProcBlock(block) => block.range,
            FeatureRepr::TargetInput { .. } => Default::default(),
        }
    }

    pub fn from_defn(
        db: &dyn FeatureGenQueryGroup,
        opt_this: Option<FeatureRepr>,
        defn_repr: &DefinitionRepr,
        feature_interner: &FeatureInterner,
    ) -> Self {
        let result = match defn_repr {
            DefinitionRepr::LazyExpr { expr } => FeatureRepr::LazyExpr(FeatureLazyExpr::new(
                db,
                opt_this,
                expr.clone(),
                &[],
                None,
                feature_interner,
            )),
            DefinitionRepr::LazyBlock { stmts, ty } => FeatureRepr::LazyBlock(
                FeatureLazyBlock::new(db, opt_this, stmts, &[], None, feature_interner, *ty),
            ),
            DefinitionRepr::FuncBlock {
                stmts,
                file,
                range,
                route,
                return_ty: output_ty,
            } => FeatureRepr::FuncBlock(Arc::new(FeatureFuncBlock {
                file: *file,
                range: *range,
                eval_id: Default::default(),
                stmts: stmts.clone(),
                instruction_sheet: {
                    new_func_instruction_sheet(
                        db.upcast(),
                        [].into_iter(),
                        stmts,
                        opt_this.is_some(),
                    )
                },
                feature: {
                    feature_interner.intern(match opt_this {
                        Some(ref this) => Feature::FieldAccess {
                            this: this.feature(),
                            field_ident: route.ident().custom(),
                        },
                        None => Feature::EntityFeature {
                            route: *route,
                            uid: db.comptime().entity_uid(*route),
                        },
                    })
                },
                opt_this,
                ty: *output_ty,
                opt_linkage: { db.comptime().feature_eager_block_linkage(*route) },
            })),
            DefinitionRepr::ProcBlock { .. } => {
                todo!()
            }
        };
        result
    }
}

impl<'eval> From<Arc<FeatureLazyExpr>> for FeatureRepr {
    fn from(expr: Arc<FeatureLazyExpr>) -> Self {
        Self::LazyExpr(expr)
    }
}

impl<'eval> From<Arc<FeatureLazyBlock>> for FeatureRepr {
    fn from(block: Arc<FeatureLazyBlock>) -> Self {
        Self::LazyBlock(block)
    }
}
