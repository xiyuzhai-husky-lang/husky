use crate::*;
use husky_entity_path::FugitivePath;
use husky_ethereal_term::EtherealTerm;
use husky_hir_defn::HirDefn;
use husky_text_protocol::range::TextRange;
use husky_val::Val;
use husky_vfs::DiffPath;
use husky_vm::RegularValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValRepr {
    Expr(ValExpr),
    Stmt(ValStmt),
    Fugitive(FugitivePath),
    // AdHocConstant(ConstantVal),
    // Value {
    //     value: __RegularValue,
    //     ty: HirType,
    //     file: DiffPath,
    //     range: TextRange,
    //     feature: Val,
    // },
    // LazyBody(ValBlock),
    // FuncBody(Arc<FeatureFuncBody>),
    // ProcBody(Arc<FeatureProcBody>),
    // TargetInput {
    //     main_file: DiffPath,
    //     ty: HirType,
    //     feature: Val,
    // },
}

impl ValRepr {
    pub fn is_lazy(&self) -> bool {
        todo!()
        // match self {
        //     ValRepr::LazyExpr(_) => true,
        //     ValRepr::LazyBody(_) => true,
        //     ValRepr::Value { .. } => false,
        //     ValRepr::FuncBody(_) => false,
        //     ValRepr::ProcBody(_) => false,
        //     ValRepr::TargetInput { .. } => false,
        // }
    }

    pub fn opt_leading_keyword(&self) -> Option<&'static str> {
        todo!()
        // match self {
        //     ValRepr::Value { .. } => panic!(),
        //     ValRepr::LazyExpr(_) => Some("def "),
        //     ValRepr::LazyBody(_) => Some("def "),
        //     ValRepr::FuncBody(_) => Some("func "),
        //     ValRepr::ProcBody(_) => Some("proc "),
        //     ValRepr::TargetInput { .. } => None,
        // }
    }

    pub fn ty(&self) -> EtherealTerm {
        todo!()
        // match self {
        //     ValRepr::Value { ty, .. } => *ty,
        //     ValRepr::LazyExpr(expr) => expr.expr.intrinsic_ty(),
        //     ValRepr::LazyBody(block) => block.return_ty.route,
        //     ValRepr::FuncBody(block) => block.ty.route,
        //     ValRepr::ProcBody(block) => block.return_ty.route,
        //     ValRepr::TargetInput { ty, .. } => *ty,
        // }
    }
    pub fn feature(&self) -> Val {
        todo!()
        // match self {
        //     ValRepr::Value { feature, .. } => *feature,
        //     ValRepr::LazyExpr(expr) => expr.feature,
        //     ValRepr::LazyBody(block) => block.feature,
        //     ValRepr::FuncBody(block) => block.feature,
        //     ValRepr::ProcBody(block) => block.feature,
        //     ValRepr::TargetInput { feature, .. } => *feature,
        // }
    }

    pub fn file(&self) -> DiffPath {
        todo!()
        // match self {
        //     ValRepr::Value { file, .. } => *file,
        //     ValRepr::LazyExpr(expr) => expr.expr.file,
        //     ValRepr::LazyBody(block) => block.file,
        //     ValRepr::FuncBody(block) => block.file,
        //     ValRepr::ProcBody(block) => block.file,
        //     ValRepr::TargetInput { main_file, .. } => *main_file,
        // }
    }

    pub fn text_range(&self) -> TextRange {
        todo!()
        // match self {
        //     ValRepr::Value { range, .. } => *range,
        //     ValRepr::LazyExpr(expr) => expr.expr.range,
        //     ValRepr::LazyBody(block) => block.range,
        //     ValRepr::FuncBody(block) => block.range,
        //     ValRepr::ProcBody(block) => block.range,
        //     ValRepr::TargetInput { .. } => Default::default(),
        // }
    }

    pub fn from_defn(
        db: &dyn ValReprDb,
        opt_this: Option<ValRepr>,
        defn_repr: HirDefn,
        // feature_interner: &FeatureInterner,
    ) -> Self {
        todo!()
        // let result =
        //     match defn_repr {
        //         DefinitionRepr::LazyExpr { expr } => ValRepr::LazyExpr(FeatureLazyExpr::new(
        //             db,
        //             opt_this,
        //             expr.clone(),
        //             &[],
        //             None,
        //             feature_interner,
        //         )),
        //         DefinitionRepr::LazyBlock { stmts, ty } => ValRepr::LazyBody(
        //             ValBlock::new(db, opt_this, stmts, &[], None, feature_interner, *ty),
        //         ),
        //         DefinitionRepr::FuncBlock {
        //             stmts,
        //             file,
        //             range,
        //             route,
        //             return_ty,
        //         } => ValRepr::FuncBody(Arc::new(FeatureFuncBody {
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
        //                         uid: db.item_uid(*route),
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
        //         } => ValRepr::ProcBody(Arc::new(FeatureProcBody {
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
        //                         uid: db.item_uid(*route),
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

    pub fn opt_domain_indicator(&self) -> Option<&ValDomain> {
        todo!()
        // match self {
        //     ValRepr::Value { .. } => None,
        //     ValRepr::LazyExpr(expr) => expr.opt_arrival_indicator.as_ref(),
        //     // ad hoc
        //     // todo: rename `Body` to `Block` and add opt_domain_indicator
        //     ValRepr::LazyBody(_) | ValRepr::FuncBody(_) | ValRepr::ProcBody(_) => None,
        //     ValRepr::TargetInput { .. } => None,
        // }
    }
}

impl From<ValExpr> for ValRepr {
    fn from(expr: ValExpr) -> Self {
        todo!()
        // Self::LazyExpr(expr)
    }
}

impl From<ValBlock> for ValRepr {
    fn from(block: ValBlock) -> Self {
        todo!()
        // Self::LazyBody(block)
    }
}
