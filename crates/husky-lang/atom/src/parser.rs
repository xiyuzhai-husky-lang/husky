use token::{Token, TokenizedText};

use crate::*;

pub struct AtomParser {}

impl folded::Parser<'_, [Token], TokenizedText, (Vec<Atom>, Vec<AtomError>), AtomParser>
    for AtomParser
{
    fn enter_fold(&mut self) {
        todo!()
    }

    fn exit_fold(&mut self) {
        todo!()
    }

    fn push(&mut self, result: (Vec<Atom>, Vec<AtomError>)) {
        todo!()
    }

    fn new() -> AtomParser {
        todo!()
    }

    fn parse_value(&mut self, value: &[Token]) -> (Vec<Atom>, Vec<AtomError>) {
        todo!()
    }
}

#[test]
fn for_fun() {}
