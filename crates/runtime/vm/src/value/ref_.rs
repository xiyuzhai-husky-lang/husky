use super::*;

pub enum RefValue<'vm, 'eval: 'vm> {
    Thin(&'vm dyn AnyValueDyn<'eval>),
    Slice {
        slice: &'vm [()],
        void_caster: VoidCaster<'eval>,
    },
    CyclicSlice {
        slice: &'vm [()],
        prototype: VoidCaster<'eval>,
        range: std::ops::Range<usize>,
    },
}

pub type VoidCaster<'eval> = for<'a> unsafe fn(&'a ()) -> &'a dyn AnyValueDyn<'eval>;
