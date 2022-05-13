use crate::*;

pub trait BindTextRange: Sized {
    type Target;

    fn bind_text_range(self, range: TextRange) -> Self::Target {
        self.ref_bind_text_range(range)
    }

    fn ref_bind_text_range(&self, range: TextRange) -> Self::Target;

    fn bind<T>(self, t: &T) -> Self::Target
    where
        T: TextRanged,
    {
        self.bind_text_range(t.text_range())
    }
}

impl<T, E1, E2> BindTextRange for Result<T, E1>
where
    E1: BindTextRange<Target = E2>,
    T: Clone,
{
    type Target = Result<T, E2>;

    fn bind_text_range(self, range: TextRange) -> Self::Target {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.bind_text_range(range)),
        }
    }

    fn ref_bind_text_range(&self, range: TextRange) -> Self::Target {
        match self {
            Ok(t) => Ok(t.clone()),
            Err(e) => Err(e.ref_bind_text_range(range)),
        }
    }
}

impl<T> BindTextRange for Arc<T>
where
    T: BindTextRange,
{
    type Target = T::Target;

    fn bind_text_range(self, range: TextRange) -> Self::Target {
        T::ref_bind_text_range(&self, range)
    }

    fn ref_bind_text_range(&self, range: TextRange) -> Self::Target {
        T::ref_bind_text_range(self, range)
    }
}
