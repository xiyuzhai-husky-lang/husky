use crate::{
    ast::{
        lisp::{helpers::LxLispAstChild, LxLispAstData, LxLispAstIdx},
        math::{
            helpers::LxMathAstChild, LxMathAstData, LxMathAstIdx, LxMathAstIdxRange,
            LxMathCommandArgumentData, LxMathCompleteCommandArgument,
        },
        rose::{helpers::LxRoseAstChild, LxRoseAstData, LxRoseAstIdx},
        LxAstArenaRef, LxAstIdx, LxAstIdxRange,
    },
    range::LxAstTokenIdxRangeMap,
};
use husky_tree_utils::display::DisplayTree;
use latex_token::storage::LxTokenStorage;

pub struct LxAstDisplayTreeBuilder<'a> {
    db: &'a salsa::Db,
    input: &'a str,
    ast_arena: LxAstArenaRef<'a>,
    ast_token_idx_range_map: &'a LxAstTokenIdxRangeMap,
    token_storage: &'a LxTokenStorage,
}

/// # construction
impl<'a> LxAstDisplayTreeBuilder<'a> {
    pub fn new(
        db: &'a salsa::Db,
        input: &'a str,
        token_storage: &'a LxTokenStorage,
        ast_arena: LxAstArenaRef<'a>,
        ast_token_idx_range_map: &'a LxAstTokenIdxRangeMap,
    ) -> Self {
        Self {
            db,
            input,
            ast_arena,
            ast_token_idx_range_map,
            token_storage,
        }
    }
}

/// # actions
impl<'a> LxAstDisplayTreeBuilder<'a> {
    pub fn render_all(&self, asts: LxAstIdxRange) -> DisplayTree {
        DisplayTree::new(
            format!("{:?} all input", self.input),
            self.render_asts(asts),
        )
    }

    pub fn render_asts(&self, asts: LxAstIdxRange) -> Vec<DisplayTree> {
        match asts {
            LxAstIdxRange::Math(asts) => self.render_math_asts(asts),
            LxAstIdxRange::Rose(asts) => self.render_rose_asts(asts),
            LxAstIdxRange::Lisp(asts) => self.render_lisp_asts(asts),
        }
    }

    fn render_math_asts(&self, asts: impl IntoIterator<Item = LxMathAstIdx>) -> Vec<DisplayTree> {
        asts.into_iter()
            .map(|ast| self.render_math_ast(ast))
            .collect()
    }

    fn render_math_ast(&self, ast: LxMathAstIdx) -> DisplayTree {
        let ast_token_idx_range = self.ast_token_idx_range_map[ast];
        let offset_range = self
            .token_storage
            .token_idx_range_offset_range(ast_token_idx_range);
        let value = &self.input[offset_range];
        let value = match self.ast_arena.math()[ast] {
            LxMathAstData::TextEdit { .. } => format!("{:?} text edit", value),
            LxMathAstData::Attach { .. } => format!("{:?} attach", value),
            LxMathAstData::Delimited { .. } => format!("{:?} delimited", value),
            LxMathAstData::CompleteCommand { .. } => format!("{:?} complete command", value),
            LxMathAstData::Environment { .. } => format!("{:?} environment", value),
            LxMathAstData::PlainLetter(_, _) => format!("{:?} plain letter", value),
            LxMathAstData::StyledLetter { .. } => format!("{:?} styled letter", value),
            LxMathAstData::Punctuation(_, _) => format!("{:?} punctuation", value),
            LxMathAstData::Digit(_, _) => format!("{:?} digit", value),
        };
        DisplayTree::new(
            value,
            self.render_math_children(self.ast_arena.math()[ast].children()),
        )
    }

    fn render_math_children(
        &self,
        children: impl IntoIterator<Item = LxMathAstChild>,
    ) -> Vec<DisplayTree> {
        children
            .into_iter()
            .map(|child| self.render_math_child(child))
            .collect()
    }

    fn render_math_child(&self, child: LxMathAstChild) -> DisplayTree {
        match child {
            LxMathAstChild::Ast(ast) => self.render_math_ast(ast),
            LxMathAstChild::CommandArgument(argument) => {
                self.render_math_command_argument(argument)
            }
        }
    }

    fn render_math_command_argument(&self, argument: LxMathCompleteCommandArgument) -> DisplayTree {
        match argument.data() {
            LxMathCommandArgumentData::Math(range) => {
                let value = if range.is_empty() {
                    "".to_string()
                } else {
                    let range = self.ast_token_idx_range_map[range.start()]
                        .join(self.ast_token_idx_range_map[range.last().unwrap()]);
                    self.input[self.token_storage.token_idx_range_offset_range(range)].to_string()
                };
                let grandchildren = self.render_math_asts(range);
                DisplayTree::new(value, grandchildren)
            }
            LxMathCommandArgumentData::Rose(range) => todo!(),
            LxMathCommandArgumentData::Letter(_, _) => todo!(),
        }
    }

    fn render_rose_asts(&self, asts: impl IntoIterator<Item = LxRoseAstIdx>) -> Vec<DisplayTree> {
        asts.into_iter()
            .map(|ast| self.render_rose_ast(ast))
            .collect()
    }

    fn render_rose_ast(&self, ast: LxRoseAstIdx) -> DisplayTree {
        let ast_token_idx_range = self.ast_token_idx_range_map[ast];
        let offset_range = self
            .token_storage
            .token_idx_range_offset_range(ast_token_idx_range);
        let value = &self.input[offset_range];
        let value = match self.ast_arena.rose()[ast] {
            LxRoseAstData::TextEdit { .. } => format!("{:?} text edit", value),
            LxRoseAstData::Word(_, _) => format!("{:?} word", value),
            LxRoseAstData::Punctuation(_, _) => format!("{:?} punctuation", value),
            LxRoseAstData::Math { .. } => format!("{:?} math", value),
            LxRoseAstData::NewParagraph(_) => format!("{:?} new paragraph", value),
            LxRoseAstData::Delimited { .. } => format!("{:?} delimited", value),
            LxRoseAstData::CompleteCommand { .. } => format!("{:?} complete command", value),
            LxRoseAstData::Environment { .. } => format!("{:?} environment", value),
        };
        DisplayTree::new(
            value,
            self.ast_arena.rose()[ast]
                .children()
                .into_iter()
                .map(|child| self.render_rose_child(child))
                .collect(),
        )
    }

    fn render_rose_child(&self, child: LxRoseAstChild) -> DisplayTree {
        match child {
            LxRoseAstChild::RoseAst(ast) => self.render_rose_ast(ast),
            LxRoseAstChild::MathAst(ast) => self.render_math_ast(ast),
        }
    }

    fn render_lisp_asts(&self, asts: impl IntoIterator<Item = LxLispAstIdx>) -> Vec<DisplayTree> {
        asts.into_iter()
            .map(|ast| self.render_lisp_ast(ast))
            .collect()
    }

    fn render_lisp_ast(&self, ast: LxLispAstIdx) -> DisplayTree {
        let ast_token_idx_range = self.ast_token_idx_range_map[ast];
        let offset_range = self
            .token_storage
            .token_idx_range_offset_range(ast_token_idx_range);
        let value = &self.input[offset_range];
        let value = match self.ast_arena.lisp()[ast] {
            LxLispAstData::Literal(_, _) => format!("{:?} literal", value),
            LxLispAstData::Ident(_, _) => format!("{:?} ident", value),
            LxLispAstData::Xlabel(_, _) => format!("{:?} xlabel", value),
            LxLispAstData::CompleteCommand { .. } => format!("{:?} complete command", value),
            LxLispAstData::Parenthesized { .. } => format!("{:?} parenthesized", value),
            LxLispAstData::BoxedList { .. } => format!("{:?} boxed list", value),
        };
        DisplayTree::new(
            value,
            self.render_lisp_children(self.ast_arena.lisp()[ast].children()),
        )
    }

    fn render_lisp_children(
        &self,
        children: impl IntoIterator<Item = LxLispAstChild>,
    ) -> Vec<DisplayTree> {
        children
            .into_iter()
            .map(|child| self.render_lisp_child(child))
            .collect()
    }

    fn render_lisp_child(&self, child: LxLispAstChild) -> DisplayTree {
        match child {
            LxLispAstChild::LispAst(ast) => self.render_lisp_ast(ast),
            LxLispAstChild::Item(asts) => {
                DisplayTree::new("item".to_string(), self.render_lisp_asts(asts.into_iter()))
            }
        }
    }
}
