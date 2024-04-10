use crate::parser::NamParser;
use idx_arena::{ArenaIdx, ArenaIdxRange};

pub enum NamAstData {}

pub type NamAstIdx = ArenaIdx<NamAstData>;
pub type NamAstIdxRange = ArenaIdxRange<NamAstData>;

impl<'a> NamParser<'a> {
    fn parse_asts(&mut self) -> NamAstIdxRange {
        let mut asts: Vec<NamAstData> = vec![];
        todo!()
    }

    fn parse_ast(&mut self) -> Option<NamAstData> {
        todo!()
    }
}
