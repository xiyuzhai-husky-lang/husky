use latex_token::idx::{math::LxMathTokenIdxRange, rose::LxRoseTokenIdxRange};

use crate::ast::{
    math::{LxMathAstArenaMap, LxMathAstData, LxMathAstIdx},
    rose::{LxRoseAstArenaMap, LxRoseAstData, LxRoseAstIdx},
    LxAstArena, LxAstArenaMap, LxAstArenaRef, LxAstData, LxAstIdx,
};

#[derive(Debug)]
pub struct LxAstTokenIdxRangeMap {
    pub(crate) math: LxMathAstArenaMap<LxMathTokenIdxRange>,
    pub(crate) rose: LxRoseAstArenaMap<LxRoseTokenIdxRange>,
}

impl std::ops::Index<LxMathAstIdx> for LxAstTokenIdxRangeMap {
    type Output = LxMathTokenIdxRange;

    fn index(&self, index: LxMathAstIdx) -> &Self::Output {
        &self.math[index]
    }
}

impl std::ops::Index<LxRoseAstIdx> for LxAstTokenIdxRangeMap {
    type Output = LxRoseTokenIdxRange;

    fn index(&self, index: LxRoseAstIdx) -> &Self::Output {
        &self.rose[index]
    }
}

pub fn calc_ast_token_idx_range_map(db: &salsa::Db, arena: &LxAstArena) -> LxAstTokenIdxRangeMap {
    let mut calculator = LxAstTokenIdxRangeCalculator::new(db, arena);
    calculator.infer_all();
    LxAstTokenIdxRangeMap {
        math: calculator.math_data,
        rose: calculator.rose_data,
    }
}

struct LxAstTokenIdxRangeCalculator<'a> {
    db: &'a ::salsa::Db,
    ast_arena: LxAstArenaRef<'a>,
    math_data: LxMathAstArenaMap<LxMathTokenIdxRange>,
    rose_data: LxRoseAstArenaMap<LxRoseTokenIdxRange>,
}

impl<'a> LxAstTokenIdxRangeCalculator<'a> {
    fn new(db: &'a ::salsa::Db, arena: &'a LxAstArena) -> Self {
        Self {
            db,
            ast_arena: arena.as_arena_ref(),
            math_data: LxMathAstArenaMap::new(&arena.math),
            rose_data: LxRoseAstArenaMap::new(&arena.rose),
        }
    }
}

impl<'a> LxAstTokenIdxRangeCalculator<'a> {
    fn infer_all(&mut self) {
        self.ast_arena.math().indexed_iter().for_each(|(idx, ast)| {
            self.infer_math_ast(idx, ast);
        });
        self.ast_arena.rose().indexed_iter().for_each(|(idx, ast)| {
            self.rose_data.insert_new(idx, self.calc_rose_ast(ast));
        });
    }

    fn infer_math_ast(&mut self, idx: LxMathAstIdx, ast: &LxMathAstData) {
        if !self.math_data.has(idx) {
            let range = self.calc_math_ast(ast);
            self.math_data.insert_new(idx, range);
        }
    }

    fn calc_math_ast(&mut self, data: &LxMathAstData) -> LxMathTokenIdxRange {
        match *data {
            LxMathAstData::Letter(idx, _) => LxMathTokenIdxRange::new_single(idx),
            LxMathAstData::Opr(idx, _) => LxMathTokenIdxRange::new_single(idx),
            LxMathAstData::Digit(idx, _) => LxMathTokenIdxRange::new_single(idx),
            LxMathAstData::TextEdit { ref buffer } => todo!(),
            LxMathAstData::Attach { base, ref scripts } => {
                let mut range = self.get_math_ast_range(base);
                for &(_, script) in scripts {
                    let script_range = self.get_math_ast_range(script);
                    range = range.join(script_range);
                }
                range
            }
            LxMathAstData::Delimited {
                left_delimiter_token_idx,
                left_delimiter,
                asts,
                right_delimiter_token_idx,
                right_delimiter,
            } => {
                LxMathTokenIdxRange::new_closed(left_delimiter_token_idx, right_delimiter_token_idx)
            }
        }
    }

    fn get_math_ast_range(&mut self, idx: LxMathAstIdx) -> LxMathTokenIdxRange {
        self.infer_math_ast(idx, &self.ast_arena.math()[idx]);
        self.math_data[idx]
    }

    fn infer_rose_ast(&mut self, idx: LxRoseAstIdx, ast: &LxRoseAstData) {
        if !self.rose_data.has(idx) {
            let range = self.calc_rose_ast(ast);
            self.rose_data.insert_new(idx, range);
        }
    }

    fn calc_rose_ast(&self, data: &LxRoseAstData) -> LxRoseTokenIdxRange {
        todo!()
    }

    fn get_rose_ast_range(&mut self, idx: LxRoseAstIdx) -> LxRoseTokenIdxRange {
        self.infer_rose_ast(idx, &self.ast_arena.rose()[idx]);
        self.rose_data[idx]
    }
}
