use crate::ast::{
    math::LxMathAstData, LxAstArena, LxAstArenaMap, LxAstArenaRef, LxAstData, LxAstIdx,
};
use latex_token::idx::LxTokenIdxRange;

pub struct LxAstRangeMap {
    data: LxAstArenaMap<LxTokenIdxRange>,
}

struct LxAstOffsetRangeCalculator<'a> {
    db: &'a ::salsa::Db,
    ast_arena: LxAstArenaRef<'a>,
    data: LxAstArenaMap<LxTokenIdxRange>,
}

impl<'a> LxAstOffsetRangeCalculator<'a> {
    fn new(db: &'a ::salsa::Db, arena: &'a LxAstArena) -> Self {
        Self {
            db,
            ast_arena: arena.as_arena_ref(),
            data: LxAstArenaMap::new(arena),
        }
    }
}

impl<'a> LxAstOffsetRangeCalculator<'a> {
    fn infer_all(&mut self) {
        self.ast_arena.indexed_iter().for_each(|(idx, ast)| {
            self.data[idx] = self.calc_ast(ast);
        });
    }

    fn calc_ast(&self, data: &LxAstData) -> LxTokenIdxRange {
        match *data {
            LxAstData::Math(ref data) => match data {
                LxMathAstData::Letter(lx_math_letter) => todo!(),
                LxMathAstData::Opr(lx_math_opr) => todo!(),
                LxMathAstData::Digit(lx_math_digit) => todo!(),
                LxMathAstData::TextEdit { buffer } => todo!(),
                LxMathAstData::Attach { base, scripts } => todo!(),
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
