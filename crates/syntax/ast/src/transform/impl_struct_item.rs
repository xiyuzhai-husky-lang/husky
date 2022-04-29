use super::utils::*;
use crate::*;
use atom::{
    parser::AtomLRParser,
    symbol::{Symbol, SymbolKind},
};
use text::TextRanged;
use token::{Special, Token, TokenKind};
use vm::{FieldContract, InputContract};
use word::RoutineKeyword;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_struct_item(
        &mut self,
        token_group: &[Token],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstKind> {
        match token_group[0].kind {
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::Config(_) => todo!(),
                Keyword::Routine(routine_keyword) => match routine_keyword {
                    RoutineKeyword::Test => todo!(),
                    RoutineKeyword::Proc => todo!(),
                    RoutineKeyword::Func => {
                        self.parse_struct_field_func(token_group, 1, enter_block)
                    }
                },
                Keyword::Type(_) => todo!(),
                Keyword::Stmt(_) => todo!(),
                Keyword::Def => todo!(),
                Keyword::Use => todo!(),
                Keyword::Mod => todo!(),
                Keyword::Main => todo!(),
            },
            TokenKind::Identifier(_) => self.parse_struct_field_var(token_group),
            TokenKind::Special(_) => todo!(),
            TokenKind::PrimitiveLiteral(_) => todo!(),
        }
    }

    pub(super) fn parse_struct_field_var(&mut self, token_group: &[Token]) -> AstResult<AstKind> {
        Ok(
            if token_group.len() >= 2 && token_group[1].kind == TokenKind::Special(Special::Colon) {
                if token_group.len() == 2 {
                    todo!()
                }
                let ident = match token_group[0].kind {
                    TokenKind::Identifier(ident) => match ident {
                        Identifier::Builtin(_) => err!(
                            "expect custom identifier but got builtin",
                            token_group[0].text_range()
                        )?,
                        Identifier::Contextual(_) => err!(
                            "expect custom identifier but got contextual",
                            token_group[0].text_range()
                        )?,
                        Identifier::Custom(custom_ident) => custom_ident,
                    },
                    _ => err!("expect custom identifier", token_group[0].text_range())?,
                };
                let ty = atom::parse_ty(&self.symbol_context(), &token_group[2..])?;
                AstKind::FieldDefnHead(FieldDefnHead {
                    ident,
                    contract: FieldContract::Own,
                    ty,
                    kind: FieldKind::StructOriginal,
                })
            } else {
                todo!()
            },
        )
    }

    pub(super) fn parse_struct_field_func(
        &mut self,
        token_group: &[Token],
        funcname_idx: usize,
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstKind> {
        enter_block(self);
        self.env.set_value(AstContext::Func);
        expect_at_least!(token_group, token_group.into(), 5);
        expect_block_head!(token_group);
        let head = self.parse_atoms(&token_group[funcname_idx..], |parser| {
            parser.method_decl(InputContract::Pure, RoutineContextKind::Func)
        })?;
        self.symbols.extend(
            head.input_placeholders
                .iter()
                .map(|input_placeholder| Symbol {
                    ident: input_placeholder.ident,
                    kind: SymbolKind::Variable {
                        init_row: input_placeholder.ranged_ty.row(),
                    },
                }),
        );
        Ok(AstKind::TypeMethodDefnHead(head))
    }

    // pub(super) fn parse_struct_static_func() {}
}
