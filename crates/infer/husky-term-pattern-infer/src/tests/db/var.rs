pub(crate) struct FakeVariable {
    varname: &'static str,
}

pub(crate) static A: &[FakeVariable] = &[];
