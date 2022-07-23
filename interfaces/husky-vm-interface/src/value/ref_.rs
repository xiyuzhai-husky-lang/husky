use super::*;

pub enum RefValue<'temp, 'eval: 'temp> {
    Thin(&'temp dyn __AnyValueDyn<'eval>),
    Slice {
        slice: &'temp [()],
        void_caster: VoidCaster<'eval>,
    },
    CyclicSlice {
        slice: &'temp [()],
        prototype: VoidCaster<'eval>,
        range: std::ops::Range<usize>,
    },
}

pub type VoidCaster<'eval> = for<'a> unsafe fn(&'a ()) -> &'a dyn __AnyValueDyn<'eval>;
