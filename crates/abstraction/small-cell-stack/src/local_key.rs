use super::*;

/// This trait is sealed and shouldn't be implemented for types outside this crate.
pub trait SmallCellStackLocalKey<A: Array>: crate::sealed::Sealed {
    fn push(&'static self, item: A::Item);

    fn pop(&'static self) -> Option<A::Item>;
}

impl<A: Array> crate::sealed::Sealed for LocalKey<SmallCellStack<A>> {}

impl<A: Array> SmallCellStackLocalKey<A> for LocalKey<SmallCellStack<A>> {
    fn push(&'static self, item: <A as Array>::Item) {
        self.with(|stack| stack.push(item))
    }

    fn pop(&'static self) -> Option<A::Item> {
        self.with(|stack| stack.pop())
    }
}
