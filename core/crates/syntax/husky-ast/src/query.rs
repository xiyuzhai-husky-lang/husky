use crate::*;
use arena::map::ArenaKeyQuery;
use fold::Transformer;
use fold::{FoldableList, FoldableStorage};
use husky_entity_syntax::{EntitySyntaxQueryGroup, EntitySyntaxResultArc};
use husky_file::FilePtr;
use husky_test_utils::TestDisplayConfig;
use husky_text::{HuskyText, TextQueryGroup};
use husky_token::AbsSemanticToken;
use lsp_types::FoldingRange;
use std::{collections::HashMap, sync::Arc};
use std::{fmt::Write, sync::Mutex};
use upcast::Upcast;

#[salsa::query_group(AstQueryGroupStorage)]
pub trait AstSalsaQueryGroup:
    EntitySyntaxQueryGroup + Upcast<dyn EntitySyntaxQueryGroup> + TextQueryGroup
{
    fn ast_text(&self, file: FilePtr) -> EntitySyntaxResultArc<AstText>;

    fn root_symbols(&self) -> Arc<Vec<Symbol>>;
}

fn root_symbols(db: &dyn AstSalsaQueryGroup) -> Arc<Vec<Symbol>> {
    Arc::new(
        db.all_main_files()
            .into_iter()
            .map(|main_file| -> Symbol {
                let module = db.module(main_file).unwrap();
                Symbol {
                    init_ident: RangedCustomIdentifier {
                        ident: module.ident().custom(),
                        range: Default::default(),
                    },
                    kind: SymbolKind::EntityRoute(module),
                }
            })
            .collect::<Vec<_>>(),
    )
}

pub trait AstQueryGroup: AstSalsaQueryGroup {
    fn parse_route_from_text(&self, text: &str) -> EntityRoutePtr {
        let root_symbols = self.root_symbols();
        let mut context = AtomContextStandalone {
            opt_package_main: self.opt_package_main(),
            db: self.upcast(),
            opt_this_ty: None,
            opt_this_contract: None,
            symbols: (&root_symbols as &[_]).into(),
            kind: AtomContextKind::Normal,
        };
        match context.parse_entity_route(text) {
            Ok(route) => route,
            Err(e) => {
                panic!("can't parse entity route from text `{text}`,\n    due to error: {e:?}")
            }
        }
    }
}

fn ast_text(this: &dyn AstSalsaQueryGroup, id: FilePtr) -> EntitySyntaxResultArc<AstText> {
    let tokenized_text = this.tokenized_text(id)?;
    let mut parser = AstTransformer::new(this, this.module(id)?)?;
    parser.transform_all_recr(tokenized_text.iter());
    Ok(Arc::new(parser.finish()))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstText {
    pub file: FilePtr,
    pub arena: RawExprArena,
    pub folded_results: FoldableList<AstResult<Ast>>,
    pub semantic_tokens: Vec<AbsSemanticToken>,
    pub text: Arc<HuskyText>,
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
            );
        }
        summary
    }
}

impl ArenaKeyQuery<RawExpr> for AstText {
    fn write_key(&self, config: TestDisplayConfig, raw_expr_idx: RawExprIdx, result: &mut String) {
        let expr = &self.arena[raw_expr_idx];
        let range = expr.range();
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
