use crate::*;
use std::str::Chars;

pub(crate) struct CharStream<'a> {
    iter: Chars<'a>,
}

impl<'a> CharStream<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            iter: input.chars(),
        }
    }
}

impl<'a> std::ops::Deref for CharStream<'a> {
    type Target = Chars<'a>;

    fn deref(&self) -> &Self::Target {
        &self.iter
    }
}

impl<'a> std::ops::DerefMut for CharStream<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.iter
    }
}

impl<'a> HasParseState for CharStream<'a> {
    type State = Chars<'a>;

    fn save_state(&self) -> Self::State {
        self.iter.clone()
    }

    fn rollback(&mut self, state: Self::State) {
        self.iter = state
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct A {}

impl<'a> ParseFrom<CharStream<'a>> for A {
    type Error = ();
    fn parse_from_without_guaranteed_rollback<'b>(
        ctx: &mut CharStream<'b>,
    ) -> Result<Option<Self>, ()> {
        if let Some(c) = ctx.next() {
            Ok((c == 'a').then_some(A {}))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct B {}

impl<'a> ParseFrom<CharStream<'a>> for B {
    type Error = ();
    fn parse_from_without_guaranteed_rollback<'b>(
        ctx: &mut CharStream<'b>,
    ) -> Result<Option<Self>, ()> {
        if let Some(c) = ctx.next() {
            Ok((c == 'b').then_some(B {}))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Comma {}

impl<'a> ParseFrom<CharStream<'a>> for Comma {
    type Error = ();
    fn parse_from_without_guaranteed_rollback<'b>(
        ctx: &mut CharStream<'b>,
    ) -> Result<Option<Self>, ()> {
        if let Some(c) = ctx.next() {
            Ok((c == ',').then_some(Comma {}))
        } else {
            Ok(None)
        }
    }
}
