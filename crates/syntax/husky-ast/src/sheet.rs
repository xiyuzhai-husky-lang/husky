use crate::*;
use husky_source_path::SourcePath;
use husky_token::{AbsSemanticToken, TokenGroupSheet};
use husky_vfs::VfsResult;

#[salsa::tracked(jar = AstJar, return_ref)]
pub(crate) fn ast_sheet(db: &dyn AstDb, entity_path: EntityPath) -> VfsResult<AstSheet> {
    let token_sheet = db.token_sheet(entity_path).as_ref()?;
    Ok(AstParser::new(db.word_db(), token_sheet).parse_all())
}

#[derive(Debug, PartialEq, Eq)]
pub struct AstSheet {
    arena: AstArena,
    top_level_asts: AstIdxRange,
}

impl AstSheet {
    pub(crate) fn new(arena: AstArena, top_level_asts: AstIdxRange) -> Self {
        Self {
            arena,
            top_level_asts,
        }
    }

    pub fn errors(&self) -> Vec<&AstError> {
        todo!()
        // self.folded_results
        //     .nodes
        //     .iter()
        //     .filter_map(|node| node.value.as_ref().err())
        //     .collect()
    }

    pub fn summarize(&self) -> String {
        todo!()
        // let mut summary = String::new();
        // for (i, folded_result) in self.folded_results.nodes.iter().enumerate() {
        //     write!(
        //         summary,
        //         "#{}, {}{:?}, {:?}\n",
        //         i,
        //         &((0..folded_result.indent)
        //             .into_iter()
        //             .map(|_| ' ')
        //             .collect::<String>()),
        //         folded_result.folding_end,
        //         folded_result.value.as_ref().map(|ast| ast.range)
        //     )
        //     .unwrap();
        // }
        // summary
    }

    // pub fn find_last_expr_before(&self, pos: TextPosition) -> Option<&Expr> {
    //     self.arena
    //         .data()
    //         .iter()
    //         .filter(|expr| expr.range.end <= pos)
    //         .max_by_key(|expr| expr.range.end)
    // }
    // pub fn find_first_expr_with_end_after(&self, pos: TextPosition) -> Option<(ExprIdx, &Expr)> {
    //     self.arena
    //         .indexed_iter()
    //         .filter(|(_, expr)| expr.range.end >= pos)
    //         .next()
    // }
}

// impl ArenaKeyQuery<Expr> for AstSheet {
//     fn write_key(&self, config: HuskyDisplayConfig, raw_expr_idx: ExprIdx, result: &mut String) {
//         let expr = &self.arena[raw_expr_idx];
//         let range = expr.text_range();
//         if config.colored {
//             result.push_str(husky_print_utils::GREEN);
//         }
//         write!(result, "{: <15?}", range).unwrap();
//         if config.colored {
//             result.push_str(husky_print_utils::CYAN);
//         }
//         write!(result, "{: <20}", self.text.text_within(range)).unwrap();
//         if config.colored {
//             result.push_str(husky_print_utils::RESET);
//         }
//     }
// }
