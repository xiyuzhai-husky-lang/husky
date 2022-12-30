use crate::*;
use std::str::Chars;

pub struct A {}

struct CharStream<'a> {
    iter: Chars<'a>,
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

impl<'a> ParseStream for CharStream<'a> {
    type State = Chars<'a>;

    fn save_state(&self) -> Self::State {
        self.iter.clone()
    }

    fn rollback(&mut self, state: Self::State) {
        self.iter = state
    }
}

impl<'a> ParseFrom<CharStream<'a>> for A {
    type Error = ();

    fn parse_from<'b>(ctx: &mut CharStream<'b>) -> Result<Option<Self>, Self::Error> {
        todo!()
    }
}
