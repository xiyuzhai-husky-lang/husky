#[macro_export]
macro_rules! field_copy_fp {
    ($Type: ty, $field: ident) => {{
        fn __wrapper<'temp, 'eval>(
            values: &mut [__TempValue<'temp, 'eval>],
        ) -> __TempValue<'temp, 'eval> {
            let value: &$Type = values[0].downcast_temp_ref();
            value.$field.take_copyable().into()
        }
        __SpecificRoutineFp(__wrapper)
    }};
}
