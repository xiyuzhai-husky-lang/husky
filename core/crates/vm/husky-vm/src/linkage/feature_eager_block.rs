#[macro_export]
macro_rules! feature_eager_block_linkage {
    ($route: expr) => {{
        fn __wrapper<'temp, 'eval>(
            __ctx: &__EvalContext<'eval>,
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            $route(__ctx).__into_temp_value()
        }
        __Linkage::SpecificTransfer(__SpecificRoutineLinkage {
            fp: __ContextualSpecificRoutineFp(__wrapper),
            nargs: 0,
            dev_src: __static_dev_src!(),
        })
    }};
}
