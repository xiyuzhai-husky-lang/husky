#[macro_export]
macro_rules! feature_eager_block_linkage {
    ($route: expr) => {{
        fn __wrapper<'temp, 'eval>(
            __opt_ctx: Option<&__EvalContext<'eval>>,
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            __EvalRef($route(__opt_ctx.unwrap())).into()
        }
        __Linkage::SpecificTransfer(__SpecificRoutineLinkage {
            fp: __SpecificRoutineFp(__wrapper),
            nargs: 0,
            dev_src: __static_dev_src!(),
        })
    }};
}
