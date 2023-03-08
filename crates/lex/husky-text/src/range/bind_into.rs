use crate::*;

pub trait BindWithTextRange: Sized {
    type Output;

    fn bind_with_text_range(self, range: TextRange) -> Self::Output;

    fn bind_with_text_ranged<S>(self, s: &S) -> Self::Output
    where
        S: HasTextRange,
    {
        self.bind_with_text_range(s.text_range())
    }
}

impl<T, E1, E2> BindWithTextRange for Result<T, E1>
where
    E1: BindWithTextRange<Output = E2>,
{
    type Output = Result<T, E2>;
    fn bind_with_text_range(self, range: TextRange) -> Result<T, E2> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.bind_with_text_range(range)),
        }
    }
}
