use crate::*;

impl TraceVariant {
    pub fn kind(&self) -> TraceKind {
        match self {
            TraceVariant::Main(..) => TraceKind::Main,
            TraceVariant::Module { .. } => TraceKind::Module,
            TraceVariant::EntityVal { 
                // ref repr,
                 .. } =>todo!(),
            //       match repr.is_lazy() {
            //     true => TraceKind::EntityFeatureLazy,
            //     false => TraceKind::EntityFeatureEager,
            // },
            TraceVariant::ValStmt(_) => TraceKind::FeatureStmt,
            TraceVariant::ValBranch(_) => TraceKind::FeatureBranch,
            TraceVariant::LazyExpr(expr) => {
                todo!()
                //     match expr.variant {
                //     FeatureLazyExprVariant::StructDerivedLazyField { ref repr, .. } => {
                //         match repr.is_lazy() {
                //             true => TraceKind::FeatureExprLazy,
                //             false => TraceKind::FeatureExprEager,
                //         }
                //     }
                //     FeatureLazyExprVariant::RecordDerivedField { .. } => TraceKind::FeatureExprLazy,
                //     FeatureLazyExprVariant::ModelCall { .. } => TraceKind::FeatureExprLazy,
                //     FeatureLazyExprVariant::EntityFeature { ref repr, .. } => match repr.is_lazy() {
                //         true => TraceKind::FeatureExprLazy,
                //         false => TraceKind::FeatureExprEager,
                //     },
                //     FeatureLazyExprVariant::EvalInput => TraceKind::FeatureExprEager,
                //     FeatureLazyExprVariant::NewRecord { .. } => TraceKind::FeatureExprLazy,
                //     FeatureLazyExprVariant::NewVecFromList { .. } => TraceKind::FeatureExprEager,
                //     _ => TraceKind::FeatureExprEager,
                // }
            }
            TraceVariant::ValCallArgument { .. } => TraceKind::FeatureCallArgument, 
            TraceVariant::EagerStmt { .. } => TraceKind::EagerStmt,
            TraceVariant::EagerBranch { .. } => TraceKind::EagerBranch, 
            TraceVariant::LoopFrame { .. } => TraceKind::LoopFrame,
            TraceVariant::EagerExpr { .. } => TraceKind::EagerExpr,
            TraceVariant::CallHead { .. } => TraceKind::CallHead,
            TraceVariant::EagerCallArgument { .. } => TraceKind::EagerCallArgument,
        }
    }
}
