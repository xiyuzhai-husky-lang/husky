pub trait IdxOrBool: Copy {
    fn is_bool_ty() -> bool;

    fn into_usize(self) -> usize;

    fn into_bool(self) -> bool;
}

impl IdxOrBool for bool {
    fn is_bool_ty() -> bool {
        true
    }

    #[track_caller]
    fn into_usize(self) -> usize {
        unreachable!()
    }

    fn into_bool(self) -> bool {
        self
    }
}
