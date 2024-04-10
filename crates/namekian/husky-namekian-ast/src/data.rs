pub mod notion;
pub mod semantics;
pub mod statement;
pub mod syntax;

use self::{
    notion::NamNotionAst, paragraph::NamParagraphLead, semantics::NamSemanticsAst,
    statement::NamStatementAst, syntax::NamSyntaxAst,
};
use crate::parser::NamParser;
use crate::*;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub enum NamAstData {
    Division {
        kind: NamDivisionKind,
        items: NamAstIdxRange,
    },
    Syntax(NamSyntaxAst),
    Notion(NamNotionAst),
    Semantics(NamSemanticsAst),
    Statement(NamStatementAst),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum NamDivisionKind {
    Book,
    Chapter,
    Section,
    Subsection,
    Subsubsection,
}

pub type NamAstArena = Arena<NamAstData>;
pub type NamAstIdx = ArenaIdx<NamAstData>;
pub type NamAstIdxRange = ArenaIdxRange<NamAstData>;

impl<'a> NamParser<'a> {
    fn parse_asts(&mut self) -> NamAstIdxRange {
        let mut asts: Vec<NamAstData> = vec![];
        while let Some(ast) = self.parse_ast() {
            asts.push(ast)
        }
        self.alloc_asts(asts)
    }

    fn parse_ast(&mut self) -> Option<NamAstData> {
        let paragraph = self.next_paragraph_within_current_level()?;
        match paragraph.lead {
            NamParagraphLead::Chapter => todo!(),
            NamParagraphLead::Section => todo!(),
            NamParagraphLead::Subsection => todo!(),
            NamParagraphLead::Subsubsection => todo!(),
            NamParagraphLead::Notion => todo!(),
            NamParagraphLead::Semantics => todo!(),
            NamParagraphLead::Proposition => todo!(),
            NamParagraphLead::Theorem => todo!(),
            NamParagraphLead::HeavyArrow => todo!(),
            NamParagraphLead::Other => todo!(),
        }
    }
}

#[test]
fn nam_ast_parsing_expects() {
    fn t(input: &str, expected: &Expect) {
        let mut parser = NamParser::new(input);
        let root_asts = parser.parse_asts();
        let arena = parser.finish();
        expected.assert_debug_eq(&(root_asts, arena))
    }
    t(
        "",
        &expect![[r#"
        (
            ArenaIdxRange(
                0..0,
            ),
            Arena {
                data: [],
            },
        )
    "#]],
    );
}
