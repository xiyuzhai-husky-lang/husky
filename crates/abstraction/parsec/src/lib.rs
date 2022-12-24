mod chain;
mod rollback;
#[cfg(test)]
mod tests;

pub use rollback::*;

pub trait ParseInput {
    type Stream<'a>: Clone
    where
        Self: 'a;

    fn stream<'a>(&'a self) -> Self::Stream<'a>;

    fn parse<'a, P: ParseFrom<Self>>(&'a self) -> (Result<Option<P>, P::Error>, Self::Stream<'a>) {
        let mut stream = self.stream();
        let p = P::parse_from(&mut stream);
        todo!()
    }
}

pub trait ParseFrom<Input>: Sized
where
    Input: ParseInput + ?Sized,
{
    type Error;
    fn parse_from<'a>(stream: &mut Input::Stream<'a>) -> Result<Option<Self>, Self::Error>;
}

// impl<Input, A, B> ParseFrom<Input> for (A, B)
// where
//     Input: ParseInput + ?Sized,
//     A: ParseFrom<Input>,
//     B: ParseFrom<Input>,
// {
//     fn parse_from<'a>(
//         stream: &mut <Input as ParseInput>::Stream<'a>,
//     ) -> (Self, <Input as ParseInput>::Stream<'a>) {
//         todo!()
//     }
// }
