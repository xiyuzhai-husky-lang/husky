use crate::*;

pub trait ParseFromWithRollback<Context>: ParseFrom<Context>
where
    Context: Parser + ?Sized,
{
    type Error;
    fn parse_from_with_rollback_when_no_error<'a>(
        stream: &mut Context,
    ) -> Result<Option<Self>, <Self as ParseFromWithRollback<Context>>::Error>;

    fn try_parse_from_with_rollback<'a>(stream: &mut Context) -> Option<Self>;
}

impl<Context, P> ParseFromWithRollback<Context> for P
where
    Context: Parser + ?Sized,
    P: ParseFrom<Context>,
{
    type Error = <P as ParseFrom<Context>>::Error;
    fn parse_from_with_rollback_when_no_error<'a>(
        stream: &mut Context,
    ) -> Result<Option<Self>, <P as ParseFrom<Context>>::Error> {
        let state = stream.save_state();
        let result = Self::parse_from_without_guaranteed_rollback(stream);
        match result {
            // rollback for no pattern
            Ok(None) => stream.rollback(state),
            _ => (),
        }
        result
    }

    fn try_parse_from_with_rollback<'a>(stream: &mut Context) -> Option<Self> {
        let state = stream.save_state();
        let result = Self::parse_from_without_guaranteed_rollback(stream);
        match result {
            Ok(Some(patt)) => Some(patt),
            Ok(None) | Err(_) => {
                stream.rollback(state);
                None
            }
        }
    }
}

pub trait ParseFromWithRollback2<P>: ParseFrom2<P>
where
    P: Parser + ?Sized,
{
    type Error;
    fn parse_from_with_rollback_when_no_error2<'a>(
        stream: &mut P,
        ctx: Self::Context,
    ) -> Result<Option<Self>, <Self as ParseFromWithRollback2<P>>::Error>;

    fn try_parse_from_with_rollback<'a>(stream: &mut P, ctx: Self::Context) -> Option<Self>;
}

impl<P, T> ParseFromWithRollback2<P> for T
where
    P: Parser + ?Sized,
    T: ParseFrom2<P>,
{
    type Error = <T as ParseFrom2<P>>::Error;
    fn parse_from_with_rollback_when_no_error2<'a>(
        parser: &mut P,
        ctx: Self::Context,
    ) -> Result<Option<Self>, <T as ParseFrom2<P>>::Error> {
        let state = parser.save_state();
        let result = Self::parse_from_without_guaranteed_rollback(parser, ctx);
        match result {
            // rollback for no pattern
            Ok(None) => parser.rollback(state),
            _ => (),
        }
        result
    }

    fn try_parse_from_with_rollback<'a>(stream: &mut P, ctx: Self::Context) -> Option<Self> {
        let state = stream.save_state();
        let result = Self::parse_from_without_guaranteed_rollback(stream, ctx);
        match result {
            Ok(Some(patt)) => Some(patt),
            Ok(None) | Err(_) => {
                stream.rollback(state);
                None
            }
        }
    }
}
