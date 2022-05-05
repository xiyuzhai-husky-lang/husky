use crate::*;
use arena::map::ArenaKeyQuery;
use entity_route_query::{EntityRouteQueryGroup, EntityRouteResultArc};
use file::FilePtr;
use fold::Transformer;
use fold::{FoldStorage, FoldedList};
use lsp_types::FoldingRange;
use std::fmt::Write;
use std::sync::Arc;
use text::{Text, TextQueryGroup};
use token::AbsSemanticToken;
use upcast::Upcast;

#[salsa::query_group(AstQueryGroupStorage)]
pub trait AstSalsaQueryGroup:
    EntityRouteQueryGroup + Upcast<dyn EntityRouteQueryGroup> + TextQueryGroup
{
    fn ast_text(&self, file: FilePtr) -> EntityRouteResultArc<AstText>;
}

pub trait AstQueryGroup: AstSalsaQueryGroup {
    // fn ast(&self, file: file::FilePtr, token_group_index: usize) -> ScopeResult<&AstResult<Ast>> {
    //     todo!()
    //     // Ok(self
    //     //     .ast_text(file)?
    //     //     .folded_results
    //     //     .fold_iter(token_group_index)
    //     //     .next()
    //     //     .unwrap()
    //     //     .value)
    // }
}

fn ast_text(this: &dyn AstSalsaQueryGroup, id: FilePtr) -> EntityRouteResultArc<AstText> {
    let tokenized_text = this.tokenized_text(id)?;
    let mut parser = AstTransformer::new(this, this.module(id)?);
    parser.transform_all(tokenized_text.iter());
    Ok(Arc::new(parser.finish()))
}

// fn parse_ty(db: &dyn AstSalsaQueryGroup, code: &'static str) -> AstResult<EntityRoutePtr> {
//     let tokens = db.tokenize(code);
//     let symbols = fold::LocalStack::<Symbol>::new();
//     let proxy = SymbolProxy {
//         main: None,
//         db,
//         this_ty: None,
//         symbols: &symbols,
//     };
//     atom::parser::parse_ty(proxy, &tokens, None)
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstText {
    pub file: FilePtr,
    pub arena: RawExprArena,
    pub folded_results: FoldedList<AstResult<Ast>>,
    pub semantic_tokens: Vec<AbsSemanticToken>,
    pub text: Arc<Text>,
}

impl AstText {
    pub fn errors(&self) -> Vec<&AstError> {
        self.folded_results
            .nodes
            .iter()
            .filter_map(|node| node.value.as_ref().err())
            .collect()
    }
}

impl ArenaKeyQuery<RawExpr> for AstText {
    fn write_key(&self, raw_expr_idx: RawExprIdx, result: &mut String) {
        let range = self.arena[raw_expr_idx].range();
        write!(result, "{: <15?}", range).unwrap();
        write!(result, "{: <50} ", self.text.ranged(range)).unwrap();
    }
}
