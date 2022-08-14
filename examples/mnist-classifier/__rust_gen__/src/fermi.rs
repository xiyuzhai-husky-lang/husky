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
}

impl<'eval> __StaticInfo for FermiMatchResult<'eval> {
    type __StaticSelf = FermiMatchResult<'static>;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "mnist_classifier::fermi::FermiMatchResult".into()
    }
}

impl<'eval> __Registrable<'eval> for FermiMatchResult<'eval> {
    unsafe fn __to_register(self) -> __Register<'eval> {
        todo!()
    }
}

pub(crate) fn fermi_match<'eval>(
    concave_components: &'eval Vec<
        crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>,
    >,
    templates: &Vec<
        fn(
            &'eval crate::line_segment_sketch::concave_component::ConcaveComponent<'eval>,
        ) -> Option<f32>,
    >,
) -> FermiMatchResult<'eval> {
    let mut others = concave_components.collect_refs();
    let mut matches = Vec::<
        Option<&'eval crate::line_segment_sketch::concave_component::ConcaveComponent>,
    >::__call__(vec![]);
    for i in 0..templates.ilen() {
        let template = templates[(i) as usize];
        matches.push(others.pop_with_largest_opt_f32_copyable(template));
    }
    return FermiMatchResult::__call__(matches, others);
}
