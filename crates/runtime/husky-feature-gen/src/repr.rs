use husky_entity_semantics::DefinitionRepr;
use husky_instruction_gen::{new_func_instruction_sheet, new_proc_instruction_sheet};
use husky_linkage_table::ResolveLinkage;
use husky_vm::__Register;
use EntityPath;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FeatureRepr {
    Value {
        value: __Register<'static>,
        ty: Term,
        file: AbsolutePath,
        range: TextRange,
        feature: FeatureItd,
    },
    LazyExpr(Arc<FeatureLazyExpr>),
    LazyBody(Arc<FeatureLazyBody>),
    FuncBody(Arc<FeatureFuncBody>),
    ProcBody(Arc<FeatureProcBody>),
    TargetInput {
        main_file: AbsolutePath,
        ty: Term,
        feature: FeatureItd,
    },
}

impl FeatureRepr {
    pub fn is_lazy(&self) -> bool {
        match self {
            FeatureRepr::LazyExpr(_) => true,
            FeatureRepr::LazyBody(_) => true,
            FeatureRepr::Value { .. } => false,
            FeatureRepr::FuncBody(_) => false,
            FeatureRepr::ProcBody(_) => false,
            FeatureRepr::TargetInput { .. } => false,
        }
    }

    pub fn opt_leading_keyword(&self) -> Option<&'static str> {
        match self {
            FeatureRepr::Value { .. } => panic!(),
            FeatureRepr::LazyExpr(_) => Some("def "),
            FeatureRepr::LazyBody(_) => Some("def "),
            FeatureRepr::FuncBody(_) => Some("func "),
            FeatureRepr::ProcBody(_) => Some("proc "),
            FeatureRepr::TargetInput { .. } => None,
        }
    }

    pub fn ty(&self) -> Term {
        todo!()
        // match self {
        //     FeatureRepr::Value { ty, .. } => *ty,
        //     FeatureRepr::LazyExpr(expr) => expr.expr.intrinsic_ty(),
        //     FeatureRepr::LazyBody(block) => block.return_ty.route,
        //     FeatureRepr::FuncBody(block) => block.ty.route,
        //     FeatureRepr::ProcBody(block) => block.return_ty.route,
        //     FeatureRepr::TargetInput { ty, .. } => *ty,
        // }
    }
    pub fn feature(&self) -> FeatureItd {
        match self {
            FeatureRepr::Value { feature, .. } => *feature,
            FeatureRepr::LazyExpr(expr) => expr.feature,
            FeatureRepr::LazyBody(block) => block.feature,
            FeatureRepr::FuncBody(block) => block.feature,
            FeatureRepr::ProcBody(block) => block.feature,
            FeatureRepr::TargetInput { feature, .. } => *feature,
        }
    }

    pub fn file(&self) -> AbsolutePath {
        match self {
            FeatureRepr::Value { file, .. } => *file,
            FeatureRepr::LazyExpr(expr) => expr.expr.file,
            FeatureRepr::LazyBody(block) => block.file,
            FeatureRepr::FuncBody(block) => block.file,
            FeatureRepr::ProcBody(block) => block.file,
            FeatureRepr::TargetInput { main_file, .. } => *main_file,
        }
    }

    pub fn text_range(&self) -> TextRange {
        match self {
            FeatureRepr::Value { range, .. } => *range,
            FeatureRepr::LazyExpr(expr) => expr.expr.range,
            FeatureRepr::LazyBody(block) => block.range,
            FeatureRepr::FuncBody(block) => block.range,
            FeatureRepr::ProcBody(block) => block.range,
            FeatureRepr::TargetInput { .. } => Default::default(),
        }
    }

    pub fn from_defn(
        db: &dyn FeatureGenQueryGroup,
        opt_this: Option<FeatureRepr>,
        defn_repr: &DefinitionRepr,
        feature_interner: &FeatureInterner,
    ) -> Self {
        todo!()
        // let result =
        //     match defn_repr {
        //         DefinitionRepr::LazyExpr { expr } => FeatureRepr::LazyExpr(FeatureLazyExpr::new(
        //             db,
        //             opt_this,
        //             expr.clone(),
        //             &[],
        //             None,
        //             feature_interner,
        //         )),
        //         DefinitionRepr::LazyBlock { stmts, ty } => FeatureRepr::LazyBody(
        //             FeatureLazyBody::new(db, opt_this, stmts, &[], None, feature_interner, *ty),
        //         ),
        //         DefinitionRepr::FuncBlock {
        //             stmts,
        //             file,
        //             range,
        //             route,
        //             return_ty,
        //         } => FeatureRepr::FuncBody(Arc::new(FeatureFuncBody {
        //             file: *file,
        //             range: *range,
        //             eval_id: Default::default(),
        //             stmts: stmts.clone(),
        //             instruction_sheet: {
        //                 new_func_instruction_sheet(
        //                     db.upcast(),
        //                     [].into_iter(),
        //                     stmts,
        //                     opt_this.is_some(),
        //                 )
        //             },
        //             feature: {
        //                 feature_interner.intern(match opt_this {
        //                     Some(ref this) => Feature::FieldAccess {
        //                         this: this.feature(),
        //                         field_ident: route.ident().custom(),
        //                     },
        //                     None => Feature::EntityFeature {
        //                         route: *route,
        //                         uid: db.entity_uid(*route),
        //                     },
        //                 })
        //             },
        //             ty: *return_ty,
        //             opt_linkage: {
        //                 match opt_this {
        //                     Some(ref this) => db.field_linkage(this.ty(), route.ident().custom()),
        //                     None => db.feature_eager_block_linkage(*route),
        //                 }
        //             },
        //             opt_this,
        //         })),
        //         DefinitionRepr::ProcBlock {
        //             stmts,
        //             file,
        //             range,
        //             route,
        //             return_ty,
        //         } => FeatureRepr::ProcBody(Arc::new(FeatureProcBody {
        //             file: *file,
        //             range: *range,
        //             eval_id: Default::default(),
        //             stmts: stmts.clone(),
        //             instruction_sheet: {
        //                 new_proc_instruction_sheet(
        //                     db.upcast(),
        //                     [].into_iter(),
        //                     stmts,
        //                     opt_this.is_some(),
        //                 )
        //             },
        //             feature: {
        //                 feature_interner.intern(match opt_this {
        //                     Some(ref this) => Feature::FieldAccess {
        //                         this: this.feature(),
        //                         field_ident: route.ident().custom(),
        //                     },
        //                     None => Feature::EntityFeature {
        //                         route: *route,
        //                         uid: db.entity_uid(*route),
        //                     },
        //                 })
        //             },
        //             return_ty: *return_ty,
        //             opt_linkage: {
        //                 match opt_this {
        //                     Some(ref this) => db.field_linkage(this.ty(), route.ident().custom()),
        //                     None => db.feature_eager_block_linkage(*route),
        //                 }
        //             },
        //             opt_this,
        //         })),
        //     };
        // result
    }

    pub fn opt_domain_indicator(&self) -> Option<&Arc<FeatureDomainIndicator>> {
        match self {
            FeatureRepr::Value { .. } => None,
            FeatureRepr::LazyExpr(expr) => expr.opt_arrival_indicator.as_ref(),
            // ad hoc
            // todo: rename `Body` to `Block` and add opt_domain_indicator
            FeatureRepr::LazyBody(_) | FeatureRepr::FuncBody(_) | FeatureRepr::ProcBody(_) => None,
            FeatureRepr::TargetInput { .. } => None,
        }
    }
}

impl<'eval> From<Arc<FeatureLazyExpr>> for FeatureRepr {
    fn from(expr: Arc<FeatureLazyExpr>) -> Self {
        Self::LazyExpr(expr)
    }
}

impl<'eval> From<Arc<FeatureLazyBody>> for FeatureRepr {
    fn from(block: Arc<FeatureLazyBody>) -> Self {
        Self::LazyBody(block)
    }
}
