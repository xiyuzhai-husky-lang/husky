use crate::*;

pub trait BindTextRangeInto<T>: Sized {
    fn bind_text_range_into(self, range: TextRange) -> T {
        self.ref_bind_text_range_into(range)
    }

    fn ref_bind_text_range_into(&self, range: TextRange) -> T;

    fn bind_into<S>(self, t: &S) -> T
    where
        S: TextRanged,
    {
        self.bind_text_range_into(t.text_range())
    }
}

impl<T, E1, E2> BindTextRangeInto<Result<T, E2>> for Result<T, E1>
where
    E1: BindTextRangeInto<E2>,
    T: Clone,
{
    fn bind_text_range_into(self, range: TextRange) -> Result<T, E2> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(e.bind_text_range_into(range)),
        }
    }

    fn ref_bind_text_range_into(&self, range: TextRange) -> Result<T, E2> {
        match self {
            Ok(t) => Ok(t.clone()),
            Err(e) => Err(e.ref_bind_text_range_into(range)),
        }
    }
}
