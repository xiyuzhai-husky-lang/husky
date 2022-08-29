use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FermiMatchResult<'eval> {
    pub(crate) matches:
        Vec<Option<&'eval crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>>>,
    pub(crate) others:
        Vec<&'eval crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>>,
}

impl<'eval> FermiMatchResult<'eval> {
    pub(crate) fn __call__(
        matches: Vec<
            Option<&'eval crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>>,
        >,
        others: Vec<&'eval crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>>,
    ) -> Self {
        Self { matches, others }
    }
    pub(crate) fn norm(&'eval self, __ctx: &dyn __EvalContext<'eval>) -> &'eval f32 {
        let __uid = entity_uid!(__ctx, "mnist_classifier::fermi::FermiMatchResult::norm");
        if let Some(__result) =
            __ctx.opt_cached_lazy_field(self as *const _ as *const std::ffi::c_void, __uid)
        {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__F32_VTABLE);
        }
        let mut norm = 0f32;
        for i in 0..self.others.ilen() {
            norm = norm.max(*self.others[(i) as usize].norm(__ctx));
        }
        __ctx
            .cache_lazy_field(
                self as *const _ as *const std::ffi::c_void,
                __uid,
                Ok(__Register::new_box::<f32>(
                    norm,
                    &__registration__::__F32_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__F32_VTABLE)
    }
}

impl<'eval> __StaticInfo for FermiMatchResult<'eval> {
    type __StaticSelf = FermiMatchResult<'static>;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::fermi::FermiMatchResult".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}

pub(crate) fn fermi_match<'eval>(
    concave_components: &'eval Vec<
        crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>,
    >,
    templates: &Vec<
        ThickFp<
            fn(
                &'static crate::line_segment_sketch::concave_component::ConcaveComponent<'static>,
            ) -> Option<f32>,
        >,
    >,
    __ctx: &dyn __EvalContext<'eval>,
) -> FermiMatchResult<'eval> {
    let mut others = concave_components.collect_refs();
    let mut matches = Vec::<
        Option<&'eval crate::line_segment_sketch::concave_component::ConcaveComponent>,
    >::__call__(vec![]);
    for i in 0..templates.ilen() {
        let template = templates[(i) as usize];
        matches.push(others.pop_with_largest_opt_f32_copyable(template, __ctx));
    }
    return FermiMatchResult::__call__(matches, others);
}
