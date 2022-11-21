use crate::*;
use fold::Transformer;
use fold::{FoldableList, FoldableStorage};
use husky_display_utils::HuskyDisplayConfig;
use husky_entity_syntax::{EntitySyntaxQueryGroup, EntitySyntaxResultArc};
use husky_path::PathItd;
use husky_text::{HuskyText, TextQueryGroup};
use husky_token::AbsSemanticToken;
use idx_arena::map::ArenaKeyQuery;
use std::fmt::Write;
use std::sync::Arc;
use upcast::Upcast;

#[salsa::query_group(AstQueryGroupStorage)]
pub trait AstSalsaQueryGroup:
    EntitySyntaxQueryGroup + Upcast<dyn EntitySyntaxQueryGroup> + TextQueryGroup
{
    fn ast_text(&self, file: PathItd) -> EntitySyntaxResultArc<AstText>;
}

pub trait AstQueryGroup: AstSalsaQueryGroup {
    fn parse_route_from_text(&self, text: &str) -> Ty {
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

fn ast_text(this: &dyn AstSalsaQueryGroup, id: PathItd) -> EntitySyntaxResultArc<AstText> {
    todo!()
    // let tokenized_text = this.tokenized_text(id)?;
    // let mut parser = AstTransformer::new(this, this.module(id)?)?;
    // parser.transform_all_recr(tokenized_text.iter());
    // Ok(Arc::new(parser.finish()))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstText {
    pub file: PathItd,
    pub arena: ExprArena,
    pub folded_results: FoldableList<AstResult<Ast>>,
    pub semantic_tokens: Vec<AbsSemanticToken>,
    pub text: Arc<HuskyText>,
    pub infer_roots: Vec<AstEntrance>,
}

impl AstText {
    pub fn errors(&self) -> Vec<&AstError> {
        self.folded_results
            .nodes
            .iter()
            .filter_map(|node| node.value.as_ref().err())
            .collect()
    }

    pub fn summarize(&self) -> String {
        let mut summary = String::new();
        for (i, folded_result) in self.folded_results.nodes.iter().enumerate() {
            write!(
                summary,
                "#{}, {}{:?}, {:?}\n",
                i,
                &((0..folded_result.indent)
                    .into_iter()
                    .map(|_| ' ')
                    .collect::<String>()),
                folded_result.folding_end,
                folded_result.value.as_ref().map(|ast| ast.range)
            )
            .unwrap();
        }
        summary
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
            .enum_iter()
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
        write!(result, "{: <20}", self.text.ranged(range)).unwrap();
        if config.colored {
            result.push_str(husky_print_utils::RESET);
        }
    }
}
