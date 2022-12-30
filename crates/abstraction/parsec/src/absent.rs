use crate::HasParseState;

pub trait FromAbsent<P, Context: HasParseState + ?Sized> {
    fn new_absent_error(state: Context::State) -> Self;
}
