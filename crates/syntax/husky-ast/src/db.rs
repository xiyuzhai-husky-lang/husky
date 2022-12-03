use crate::*;
use husky_display_utils::HuskyDisplayConfig;
use husky_entity_tree::{EntityTreeDb, EntityTreeResultArc};
use husky_source_path::SourcePath;
use husky_text::{Text, TextDb};
use husky_token::AbsSemanticToken;
use idx_arena::map::ArenaKeyQuery;
use std::fmt::Write;
use std::sync::Arc;
use upcast::Upcast;

pub trait AstDb: DbWithJar<AstJar> + EntityTreeDb + Upcast<dyn EntityTreeDb> + TextDb {
    fn ast_text(&self, file: SourcePath) -> EntityTreeResultArc<AstText>;

    fn parse_route_from_text(&self, text: &str) -> Term {
        todo!()
        // let root_symbols = self.root_symbols();
        // let mut context = AtomContextStandalone {
        //     db: self.upcast(),
        //     opt_this_ty: None,
        //     opt_this_contract: None,
        //     symbols: (&root_symbols as &[_]).into(),
        //     kind: AtomContextKind::Normal,
        //     opt_file: None,
        // };
        // match context.parse_entity_route(text) {
        //     Ok(route) => route,
        //     Err(e) => {
        //         panic!("can't parse entity route from text `{text}`,\n    due to error: {e:?}")
        //     }
        // }
    }
}

impl<T> AstDb for T
where
    T: DbWithJar<AstJar> + EntityTreeDb + Upcast<dyn EntityTreeDb> + TextDb,
{
    fn ast_text(&self, file: SourcePath) -> EntityTreeResultArc<AstText> {
        todo!()
    }
}

fn ast_text(this: &dyn AstDb, id: SourcePath) -> EntityTreeResultArc<AstText> {
    todo!()
    // let tokenized_text = this.tokenized_text(id)?;
    // let mut parser = AstTransformer::new(this, this.module(id)?)?;
    // parser.transform_all_recr(tokenized_text.iter());
    // Ok(Arc::new(parser.finish()))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstText {
    pub file: SourcePath,
    pub arena: ExprArena,
    // pub folded_results: FoldableList<AstResult<DeprecatedAst>>,
    pub semantic_tokens: Vec<AbsSemanticToken>,
    pub text: Arc<Text>,
    pub infer_roots: Vec<AstEntrance>,
}

impl AstText {
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

    pub fn find_last_expr_before(&self, pos: TextPosition) -> Option<&Expr> {
        self.arena
            .data()
            .iter()
            .filter(|expr| expr.range.end <= pos)
            .max_by_key(|expr| expr.range.end)
    }
    pub fn find_first_expr_with_end_after(&self, pos: TextPosition) -> Option<(ExprIdx, &Expr)> {
        self.arena
            .indexed_iter()
            .filter(|(_, expr)| expr.range.end >= pos)
            .next()
    }
}

impl ArenaKeyQuery<Expr> for AstText {
    fn write_key(&self, config: HuskyDisplayConfig, raw_expr_idx: ExprIdx, result: &mut String) {
        let expr = &self.arena[raw_expr_idx];
        let range = expr.text_range();
        if config.colored {
            result.push_str(husky_print_utils::GREEN);
        }
        write!(result, "{: <15?}", range).unwrap();
        if config.colored {
            result.push_str(husky_print_utils::CYAN);
        }
        write!(result, "{: <20}", self.text.text_within(range)).unwrap();
        if config.colored {
            result.push_str(husky_print_utils::RESET);
        }
    }
}
