mod config;
mod impl_arrival;
mod impl;
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
pub use indicator::FeatureEvalIndicator;
pub use sheet::*;

use crate::*;
use husky_entity_semantics::*;
use husky_trace_protocol::SampleId;
use husky_vm::{EntityUid, VMConfig, __EvalContext, __Register};
use husky_vm::{__VMResult, c_void};

pub struct FeatureEvaluator<'a, 'eval: 'a> {
    pub(crate) sample_id: SampleId,
    pub target_input: __Register<'eval>,
    pub(crate) sheet: &'a EvalSheet<'eval>,
    pub(crate) db: &'a dyn FeatureGenQueryGroup,
    pub(crate) evaluator_config: &'a EvaluatorConfig,
    pub(crate) opt_static_husky_feature_eval: Option<&'a dyn EvalFeature<'static>>,
}

impl<'a, 'eval: 'a> __EvalContext<'eval> for FeatureEvaluator<'a, 'eval> {
    fn entity_uid(&self, entity_route_text: &str) -> u64 {
        let route = self.db.parse_route_from_text(entity_route_text);
        self.db.entity_uid(route).raw()
    }

    fn opt_cached_lazy_field(
        &self,
        this: *const c_void,
        uid: u64,
    ) -> Option<__VMResult<__Register<'eval>>> {
        self.sheet.cached_value(EvalKey::StructDerivedField {
            this,
            field_uid: unsafe { EntityUid::from_raw(uid) },
        })
    }

    fn opt_cached_feature(&self, feature: usize) -> Option<__VMResult<__Register<'eval>>> {
        self.sheet
            .cached_value(EvalKey::Feature(unsafe { FeatureItd::from_raw(feature) }))
    }

    fn cache_feature(
        &self,
        feature: usize,
        value: __VMResult<__Register<'eval>>,
    ) -> __VMResult<__Register<'eval>> {
        self.sheet.cache(
            EvalKey::Feature(unsafe { FeatureItd::from_raw(feature) }),
            value,
        )
    }

    fn cache_lazy_field(
        &self,
        this: *const std::ffi::c_void,
        uid: u64,
        value: __VMResult<__Register<'eval>>,
    ) -> __VMResult<__Register<'eval>> {
        self.sheet.cache(
            EvalKey::StructDerivedField {
                this,
                field_uid: unsafe { EntityUid::from_raw(uid) },
            },
            value,
        )
    }

    fn feature_ptr(&self, feature_route_text: &str) -> usize {
        todo!()
        // let route = self.db.parse_route_from_text(feature_route_text);
        // let uid = self.db.entity_uid(route);
        // unsafe {
        //     self.db
        //         .feature_interner()
        //         .intern(Feature::EntityFeature { route, uid })
        //         .id()
        //         .raw()
        // }
    }

    fn eval_feature_from_uid(&self, uid_raw: u64) -> __VMResult<__Register<'eval>> {
        todo!()
        // let uid = unsafe { EntityUid::from_raw(uid_raw) };
        // let route = self.db.entity_route_by_uid(uid);
        // let feature = self
        //     .db
        //     .feature_interner()
        //     .intern(Feature::EntityFeature { route, uid });
        // if let Some(result) = self.sheet.cached_value(EvalKey::Feature(feature)) {
        //     result
        // } else {
        //     let repr = self.db.entity_feature_repr(route);
        //     self.eval_feature_repr_cached(&repr)
        // }
    }

    fn target_input(&self) -> &__Register<'eval> {
        &self.target_input
    }
}

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub unsafe fn some_ctx(&'a self) -> Option<&'a dyn __EvalContext<'eval>> {
        Some(self)
    }

    fn vm_config(&self) -> &'a VMConfig {
        &self.evaluator_config.vm
    }

    fn cache(
        &self,
        eval_key: EvalKey,
        compute_value: impl FnOnce(&Self) -> __VMResult<__Register<'eval>>,
    ) -> __VMResult<__Register<'eval>> {
        if let Some(result) = self.sheet.cached_value(eval_key) {
            result
        } else {
            let result = compute_value(self);
            self.sheet.try_cache(eval_key, result)
        }
    }

    fn as_static(&self) -> FeatureEvaluator<'a, 'static> {
        self.opt_static_husky_feature_eval
            .unwrap()
            .evaluator(self.sample_id)
    }
}
