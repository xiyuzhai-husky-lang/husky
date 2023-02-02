// use super::*;

// pub trait BindTextRangeFrom<S>: Sized {
//     fn bind_text_range_from(src: S, range: TextRange) -> Self {
//         Self::bind_text_range_from_ref(&src, range)
//     }

//     fn bind_text_range_from_ref(src: &S, range: TextRange) -> Self;
// }

// impl<T, A> BindTextRangeFrom<Arc<A>> for T
// where
//     T: BindTextRangeFrom<A>,
// {
//     fn bind_text_range_from_ref(src: &Arc<A>, range: TextRange) -> Self {
//         Self::bind_text_range_from_ref(&src as &A, range)
//     }
// }

// impl<T, S> BindTextRangeInto for S
// where
//     T: BindTextRangeFrom<S>,
// {
//     type Output = T;

//     fn bind_text_range_into(self, range: TextRange) -> T {
//         T::bind_text_range_from(self, range)
//     }
//     fn ref_bind_text_range_into(&self, range: TextRange) -> T {
//         T::bind_text_range_from_ref(self, range)
//     }
// }
