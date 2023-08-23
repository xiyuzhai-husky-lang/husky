mod config;
mod impl_arrival;
mod impl_block;
mod impl_branch;
mod impl_cached;
mod impl_eval_context;
mod impl_expr;
mod impl_repr;
mod impl_serialize;
mod impl_stmt;
mod impl_visualize;
mod indicator;
mod sheet;

pub use config::*;
use husky_ast::AstDb;
use husky_val_repr::db::ValReprDb;
pub use indicator::FeatureEvalIndicator;
pub use sheet::*;

use crate::*;
use husky_trace_protocol::SampleId;
use husky_val::Val;
use husky_vm::{EntityUid, VMConfig, __EvalContext, __RegularValue};
use husky_vm::{__VMResult, c_void};

pub struct FeatureEvaluator<'a> {
    pub(crate) sample_id: SampleId,
    pub target_input: __RegularValue,
    pub(crate) sheet: &'a EvalSheet,
    pub(crate) db: &'a dyn ValReprDb,
    pub(crate) evaluator_config: &'a EvaluatorConfig,
    pub(crate) opt_static_husky_feature_eval: Option<&'a dyn EvalFeature>,
}

impl<'a> __EvalContext for FeatureEvaluator<'a> {
    fn item_uid(&self, item_route_text: &str) -> u64 {
        let route = self.db.parse_route_from_text(item_route_text);
        self.db.item_uid(route).raw()
    }

    fn opt_cached_lazy_field(
        &self,
        this: *const c_void,
        uid: u64,
    ) -> Option<__VMResult<__RegularValue>> {
        self.sheet.cached_value(EvalKey::StructDerivedField {
            this,
            field_uid: unsafe { EntityUid::from_declarative(uid) },
        })
    }

    fn opt_cached_feature(&self, feature: usize) -> Option<__VMResult<__RegularValue>> {
        self.sheet
            .cached_value(EvalKey::Feature(unsafe { Val::from_declarative(feature) }))
    }

    fn cache_feature(
        &self,
        feature: usize,
        value: __VMResult<__RegularValue>,
    ) -> __VMResult<__RegularValue> {
        self.sheet.cache(
            EvalKey::Feature(unsafe { Val::from_declarative(feature) }),
            value,
        )
    }

    fn cache_lazy_field(
        &self,
        this: *const std::ffi::c_void,
        uid: u64,
        value: __VMResult<__RegularValue>,
    ) -> __VMResult<__RegularValue> {
        self.sheet.cache(
            EvalKey::StructDerivedField {
                this,
                field_uid: unsafe { EntityUid::from_declarative(uid) },
            },
            value,
        )
    }

    fn feature_ptr(&self, feature_route_text: &str) -> usize {
        todo!()
        // let route = self.db.parse_route_from_text(feature_route_text);
        // let uid = self.db.item_uid(route);
        // unsafe {
        //     self.db
        //         .feature_interner()
        //         .intern(Feature::EntityFeature { route, uid })
        //         .id()
        //         .raw()
        // }
    }

    fn eval_feature_from_uid(&self, uid_raw: u64) -> __VMResult<__RegularValue> {
        todo!()
        // let uid = unsafe { EntityUid::from_declarative(uid_raw) };
        // let route = self.db.item_route_by_uid(uid);
        // let feature = self
        //     .db
        //     .feature_interner()
        //     .intern(Feature::EntityFeature { route, uid });
        // if let Some(result) = self.sheet.cached_value(EvalKey::Feature(feature)) {
        //     result
        // } else {
        //     let repr = self.db.item_feature_repr(route);
        //     self.eval_feature_repr_cached(&repr)
        // }
    }

    fn target_input(&self) -> &__RegularValue {
        &self.target_input
    }
}

impl<'a> FeatureEvaluator<'a> {
    pub unsafe fn some_ctx(&'a self) -> Option<&'a dyn __EvalContext> {
        Some(self)
    }

    fn vm_config(&self) -> &'a VMConfig {
        &self.evaluator_config.vm
    }

    fn cache(
        &self,
        eval_key: EvalKey,
        compute_value: impl FnOnce(&Self) -> __VMResult<__RegularValue>,
    ) -> __VMResult<__RegularValue> {
        if let Some(result) = self.sheet.cached_value(eval_key) {
            result
        } else {
            let result = compute_value(self);
            self.sheet.try_cache(eval_key, result)
        }
    }

    fn as_static(&self) -> FeatureEvaluator<'a> {
        self.opt_static_husky_feature_eval
            .unwrap()
            .evaluator(self.sample_id)
    }
}
