use crate::db::ValReprDb;
use crate::*;
use husky_hir_lazy_expr::HirLazyExprMap;
use husky_val::ValOpr;

#[derive(Debug, PartialEq, Eq)]
pub struct ValReprExpansion {
    map: HirLazyExprMap<ValRepr>,
}

#[salsa::tracked(jar = ValReprJar, return_ref)]
pub(crate) fn val_repr_expansion(
    db: &dyn ValReprDb,
    val_repr: ValRepr,
) -> Option<ValReprExpansion> {
    match val_repr.opr(db) {
        ValOpr::Fugitive(_) => todo!(),
        ValOpr::Require => None,
    }
}

// pub fn from_defn(
//     db: &dyn ValReprDb,
//     opt_this: Option<ValRepr>,
//     defn_repr: HirDefn,
//     // feature_interner: &FeatureInterner,
// ) -> Self {
//     todo!()
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
