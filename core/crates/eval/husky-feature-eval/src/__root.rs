use crate::*;
use husky_ast::AstQueryGroup;
pub use vm::EntityUid as __EntityUid;
use vm::{__AnyValue, __AnyValueDyn, __EvalContext, __EvalRef, __EvalValue};
use wild_utils::wild_arb_ref;

pub fn __cache_feature<'eval, T>(
    __ctx: &__EvalContext<'eval>,
    feature: FeaturePtr,
    value: __EvalValueResult<'eval>,
) -> __EvalResult<&'eval T>
where
    T: __AnyValue<'eval>,
{
    let evaluator = unsafe { __evaluator(__ctx) };
    evaluator
        .sheet
        .cache(EvalKey::Feature(feature), value)
        .map(|v| v.eval_ref().0.__downcast_ref())
}

pub fn __opt_cached_feature<'eval, T>(
    __ctx: &__EvalContext<'eval>,
    feature: FeaturePtr,
) -> Option<__EvalResult<&'eval T>>
where
    T: __AnyValue<'eval>,
{
    let evaluator = unsafe { __evaluator(__ctx) };
    evaluator
        .sheet
        .cached_value(EvalKey::Feature(feature))
        .map(|result| result.map(|v| v.eval_ref().0.__downcast_ref()))
}

#[macro_export]
macro_rules! feature_ptr {
    ($ctx: ident, $text: expr) => {{
        unsafe {
            static mut __OPT_FEATURE_PTR: Option<__FeaturePtr> = None;
            if let Some(__feature_ptr) = __OPT_FEATURE_PTR {
                __feature_ptr
            } else {
                let __feature_ptr = __get_feature_ptr($ctx, $text);
                __OPT_FEATURE_PTR = Some(__feature_ptr);
                __feature_ptr
            }
        }
    }};
}

pub fn __get_feature_ptr<'eval>(
    __ctx: &__EvalContext<'eval>,
    feature_route_text: &str,
) -> FeaturePtr {
    use husky_entity_semantics::EntityDefnQueryGroup;
    let evaluator = unsafe { __evaluator(__ctx) };
    let route = evaluator
        .db
        .compile_time()
        .parse_route_from_text(feature_route_text);
    let uid = evaluator.db.compile_time().entity_uid(route);
    evaluator
        .db
        .feature_interner()
        .intern(Feature::EntityFeature { route, uid })
}

pub unsafe fn __evaluator<'a, 'eval: 'a>(
    __ctx: &'a __EvalContext<'eval>,
) -> &'a FeatureEvaluator<'a, 'eval> {
    wild_arb_ref(__ctx)
}

pub fn __cache_lazy_field<'eval, T>(
    __ctx: &__EvalContext<'eval>,
    this: &'eval dyn __AnyValueDyn<'eval>,
    uid: __EntityUid,
    value: __EvalValueResult<'eval>,
) -> __EvalResult<&'eval T>
where
    T: __AnyValue<'eval>,
{
    let evaluator = unsafe { __evaluator(__ctx) };
    evaluator
        .sheet
        .cache(
            EvalKey::StructDerivedField {
                parent: __EvalRef(this),
                field_uid: uid,
            },
            value,
        )
        .map(|v| v.eval_ref().0.__downcast_ref())
}

pub fn __opt_cached_lazy_field<'eval, T>(
    __ctx: &__EvalContext<'eval>,
    this: &'eval dyn __AnyValueDyn<'eval>,
    uid: __EntityUid,
) -> Option<__EvalResult<&'eval T>>
where
    T: __AnyValue<'eval>,
{
    let evaluator = unsafe { __evaluator(__ctx) };
    evaluator
        .sheet
        .cached_value(EvalKey::StructDerivedField {
            parent: __EvalRef(this),
            field_uid: uid,
        })
        .map(|result| result.map(|v| v.eval_ref().0.__downcast_ref()))
}

#[macro_export]
macro_rules! entity_uid {
    ($ctx: ident, $text: expr) => {{
        unsafe {
            static mut __OPT_ENTITY_UID: Option<__EntityUid> = None;
            if let Some(__entity_uid) = __OPT_ENTITY_UID {
                __entity_uid
            } else {
                let __entity_uid = __get_entity_uid($ctx, $text);
                __OPT_ENTITY_UID = Some(__entity_uid);
                __entity_uid
            }
        }
    }};
}

pub fn __get_entity_uid<'eval>(
    __ctx: &__EvalContext<'eval>,
    entity_route_text: &str,
) -> __EntityUid {
    use husky_entity_semantics::EntityDefnQueryGroup;
    let evaluator = unsafe { __evaluator(__ctx) };
    let route = evaluator
        .db
        .compile_time()
        .parse_route_from_text(entity_route_text);
    evaluator.db.compile_time().entity_uid(route)
}
