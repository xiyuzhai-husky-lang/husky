use crate::ast::{
    math::LxMathAstData, LxAstArena, LxAstArenaMap, LxAstArenaRef, LxAstData, LxAstIdx,
};
use latex_token::idx::LxTokenIdxRange;

pub type LxAstTokenIdxRangeMap = LxAstArenaMap<LxTokenIdxRange>;

pub fn calc_ast_token_idx_range_map(db: &salsa::Db, arena: &LxAstArena) -> LxAstTokenIdxRangeMap {
    let mut calculator = LxAstTokenIdxRangeCalculator::new(db, arena);
    calculator.infer_all();
    calculator.data
}

struct LxAstTokenIdxRangeCalculator<'a> {
    db: &'a ::salsa::Db,
    ast_arena: LxAstArenaRef<'a>,
    data: LxAstArenaMap<LxTokenIdxRange>,
}

impl<'a> LxAstTokenIdxRangeCalculator<'a> {
    fn new(db: &'a ::salsa::Db, arena: &'a LxAstArena) -> Self {
        Self {
            db,
            ast_arena: arena.as_arena_ref(),
            data: LxAstArenaMap::new(arena),
        }
    }
}

impl<'a> LxAstTokenIdxRangeCalculator<'a> {
    fn infer_all(&mut self) {
        self.ast_arena.indexed_iter().for_each(|(idx, ast)| {
            self.data.insert_new(idx, self.calc_ast(ast));
        });
    }

    fn calc_ast(&self, data: &LxAstData) -> LxTokenIdxRange {
        match *data {
            LxAstData::Math(ref data) => match *data {
                LxMathAstData::Letter(idx, _) => LxTokenIdxRange::new_single(idx),
                LxMathAstData::Opr(idx, _) => LxTokenIdxRange::new_single(idx),
                LxMathAstData::Digit(idx, _) => LxTokenIdxRange::new_single(idx),
                LxMathAstData::TextEdit { ref buffer } => todo!(),
                LxMathAstData::Attach { base, ref scripts } => todo!(),
                LxMathAstData::Delimited {
                    left_delimiter_token_idx,
                    left_delimiter,
                    asts,
                    right_delimiter_token_idx,
                    right_delimiter,
                } => todo!(),
            },
            LxAstData::Rose(ref data) => todo!(),
        }
    }
}
