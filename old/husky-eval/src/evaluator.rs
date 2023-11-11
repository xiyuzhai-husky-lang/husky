mod config;
mod eval_arrival;
mod eval_block;
mod eval_branch;
mod eval_visual;
mod impl_cached;
mod impl_eval_context;
mod impl_expr;
mod impl_repr;
mod impl_serialize;
mod impl_stmt;
mod indicator;
mod sheet;

use std::panic::RefUnwindSafe;

pub use config::*;
use husky_ast::AstDb;
use husky_val_repr::db::ValReprDb;
pub use indicator::EvalIndicator;
pub use sheet::*;

use crate::*;
use husky_val::Val;
use husky_vm::{c_void, VMResult};
use husky_vm::{EntityUid, RegularValue, VMConfig, __EvalContext};

pub struct Evaluator<'a> {
    pub(crate) sample_id: SampleId,
    pub target_input: RegularValue,
    pub(crate) sheet: &'a EvalSheet,
    pub(crate) db: &'a (dyn ValReprDb + RefUnwindSafe),
    pub(crate) evaluator_config: &'a EvaluatorConfig,
    pub(crate) opt_static_husky_feature_eval: Option<&'a dyn Runtime>,
}

impl<'a> __EvalContext for Evaluator<'a> {
    fn item_uid(&self, item_route_text: &str) -> u64 {
        todo!()
        // let route = self.db.parse_route_from_text(item_route_text);
        // self.db.item_uid(route).raw()
    }

    fn opt_cached_lazy_field(
        &self,
        this: *const c_void,
        uid: u32,
    ) -> Option<VMResult<RegularValue>> {
        self.sheet.cached_value(EvalKey::StructDerivedField {
            this,
            field_uid: unsafe { EntityUid::from_raw(uid) },
        })
    }

    fn opt_cached_feature(&self, feature: u32) -> Option<VMResult<RegularValue>> {
        self.sheet
            .cached_value(EvalKey::Feature(unsafe { Val::from_raw(feature) }))
    }

    fn cache_feature(&self, feature: u32, value: VMResult<RegularValue>) -> VMResult<RegularValue> {
        self.sheet
            .cache(EvalKey::Feature(unsafe { Val::from_raw(feature) }), value)
    }

    fn cache_lazy_field(
        &self,
        this: *const std::ffi::c_void,
        uid: u32,
        value: VMResult<RegularValue>,
    ) -> VMResult<RegularValue> {
        self.sheet.cache(
            EvalKey::StructDerivedField {
                this,
                field_uid: unsafe { EntityUid::from_raw(uid) },
            },
            value,
        )
    }

    fn feature_raw_id(&self, feature_route_text: &str) -> u32 {
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

    fn eval_feature_from_uid(&self, uid_raw: u32) -> VMResult<RegularValue> {
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

    fn target_input(&self) -> &RegularValue {
        &self.target_input
    }
}

impl<'a> Evaluator<'a> {
    pub unsafe fn some_ctx(&'a self) -> Option<&'a dyn __EvalContext> {
        Some(self)
    }

    fn vm_config(&self) -> &'a VMConfig {
        &self.evaluator_config.vm
    }

    fn cache(
        &self,
        eval_key: EvalKey,
        compute_value: impl FnOnce(&Self) -> VMResult<RegularValue>,
    ) -> VMResult<RegularValue> {
        if let Some(result) = self.sheet.cached_value(eval_key) {
            result
        } else {
            let result = compute_value(self);
            self.sheet.try_cache(eval_key, result)
        }
    }

    fn as_static(&self) -> Evaluator<'a> {
        self.opt_static_husky_feature_eval
            .unwrap()
            .evaluator(self.sample_id)
    }
}
