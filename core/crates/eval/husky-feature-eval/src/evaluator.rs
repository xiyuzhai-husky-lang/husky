mod config;
mod impl_arrival;
mod impl_block;
mod impl_cached;
mod impl_eval_context;
mod impl_expr;
mod impl_repr;
mod impl_stmt;
mod impl_visualize;
mod indicator;
mod sheet;

pub use config::*;
use husky_ast::AstQueryGroup;
pub use indicator::FeatureEvalIndicator;
pub use sheet::*;

use crate::*;
use husky_feature_gen::FeatureEvalId;
use husky_trace_protocol::SampleId;
use vm::{EntityUid, EvalContextDeprecated, VMConfig, __AnyValueDyn, __EvalRef, __EvalValue};
use vm::{__EvalResult, __EvalValueResult};

pub struct FeatureEvaluator<'a, 'eval: 'a> {
    pub(crate) sample_id: SampleId,
    pub eval_input: __EvalValue<'eval>,
    pub(crate) sheet: &'a EvalSheet<'eval>,
    pub(crate) db: &'a dyn FeatureGenQueryGroup,
    pub(crate) evaluator_config: &'a EvaluatorConfig,
    pub(crate) opt_static_husky_feature_eval: Option<&'a dyn EvalFeature<'static>>,
}

impl<'a, 'eval: 'a> EvalContextDeprecated<'eval> for FeatureEvaluator<'a, 'eval> {
    fn entity_uid(&self, entity_route_text: &str) -> vm::EntityUid {
        use husky_entity_semantics::EntityDefnQueryGroup;
        let route = self
            .db
            .compile_time()
            .parse_route_from_text(entity_route_text);
        self.db.compile_time().entity_uid(route)
    }

    fn opt_cached_lazy_field(
        &self,
        this: &'eval dyn __AnyValueDyn<'eval>,
        uid: EntityUid,
    ) -> Option<__EvalValueResult<'eval>> {
        self.sheet.cached_value(EvalKey::StructDerivedField {
            parent: __EvalRef(this),
            field_uid: uid,
        })
    }

    fn opt_cached_feature(&self, feature: *const ()) -> Option<__EvalValueResult<'eval>> {
        self.sheet
            .cached_value(EvalKey::Feature(unsafe { FeaturePtr::from_raw(feature) }))
    }

    fn cache_feature(
        &self,
        feature: *const (),
        value: __EvalValueResult<'eval>,
    ) -> __EvalValueResult<'eval> {
        self.sheet.cache(
            EvalKey::Feature(unsafe { FeaturePtr::from_raw(feature) }),
            value,
        )
    }

    fn cache_lazy_field(
        &self,
        this: &'eval dyn __AnyValueDyn<'eval>,
        uid: EntityUid,
        value: __EvalValueResult<'eval>,
    ) -> __EvalValueResult<'eval> {
        self.sheet.cache(
            EvalKey::StructDerivedField {
                parent: __EvalRef(this),
                field_uid: uid,
            },
            value,
        )
    }

    fn get_feature_ptr(&self, feature_route_text: &str) -> *const () {
        use husky_entity_semantics::EntityDefnQueryGroup;
        let route = self
            .db
            .compile_time()
            .parse_route_from_text(feature_route_text);
        let uid = self.db.compile_time().entity_uid(route);
        unsafe {
            self.db
                .feature_interner()
                .intern(Feature::EntityFeature { route, uid })
                .to_raw()
        }
    }
}

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub unsafe fn some_ctx(&'a self) -> Option<&'a dyn EvalContextDeprecated<'eval>> {
        Some(self)
    }

    fn vm_config(&self) -> &'a VMConfig {
        &self.evaluator_config.vm
    }

    fn cache(
        &mut self,
        eval_key: EvalKey<'eval>,
        compute_value: impl FnOnce(&mut Self) -> __EvalValueResult<'eval>,
    ) -> __EvalValueResult<'eval> {
        if let Some(value) = self.sheet.cached_value(eval_key) {
            value
        } else {
            let value = compute_value(self);
            self.sheet.cache(eval_key, value)
        }
    }

    fn as_static(&self) -> FeatureEvaluator<'a, 'static> {
        self.opt_static_husky_feature_eval
            .unwrap()
            .evaluator(self.sample_id)
    }
}
