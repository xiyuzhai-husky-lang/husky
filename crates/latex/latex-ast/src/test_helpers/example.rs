use crate::{
    ast::{parse_latex_input_into_asts, LxAstArena, LxAstIdxRange},
    range::{calc_ast_token_idx_range_map, LxAstTokenIdxRangeMap},
};
use latex_prelude::mode::LxMode;
use latex_token::storage::LxTokenStorage;

#[derive(Debug)]
pub struct LxAstsExample {
    pub token_storage: LxTokenStorage,
    pub ast_arena: LxAstArena,
    pub asts: LxAstIdxRange,
    pub ast_token_idx_range_map: LxAstTokenIdxRangeMap,
}

impl LxAstsExample {
    pub fn new(input: &str, root_mode: LxMode, db: &salsa::Db) -> Self {
        let mut ast_arena = LxAstArena::default();
        let mut token_storage = LxTokenStorage::default();
        let asts =
            parse_latex_input_into_asts(db, input, root_mode, &mut token_storage, &mut ast_arena);
        let ast_token_idx_range_map = calc_ast_token_idx_range_map(db, &ast_arena);
        Self {
            token_storage: todo!(),
            ast_arena,
            asts,
            ast_token_idx_range_map,
        }
    }
}
