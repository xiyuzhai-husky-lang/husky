use super::utils::*;
use crate::{
    atom::{
        parser::AtomLRParser,
        symbol_proxy::{Symbol, SymbolKind},
    },
    *,
};
use text::TextRanged;
use token::{Special, Token, TokenKind};
use vm::{InputContract, MembAccessContract};
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
                        self.parse_struct_memb_func(token_group, 1, enter_block)
                    }
                },
                Keyword::Type(_) => todo!(),
                Keyword::Stmt(_) => todo!(),
                Keyword::Def => todo!(),
                Keyword::Use => todo!(),
                Keyword::Mod => todo!(),
                Keyword::Main => todo!(),
            },
            TokenKind::Identifier(_) => self.parse_struct_memb_var(token_group),
            TokenKind::Special(_) => todo!(),
            TokenKind::I32Literal(_) => todo!(),
            TokenKind::F32Literal(_) => todo!(),
        }
    }

    pub(super) fn parse_struct_memb_var(&mut self, token_group: &[Token]) -> AstResult<AstKind> {
        Ok(
            if token_group.len() >= 2 && token_group[1].kind == TokenKind::Special(Special::Colon) {
                if token_group.len() == 2 {
                    todo!()
                }
                let ident = match token_group[0].kind {
                    TokenKind::Identifier(ident) => match ident {
                        Identifier::Builtin(_) => err!(
                            Some(self.file),
                            token_group[0].text_range(),
                            "expect custom identifier but got builtin"
                        )?,
                        Identifier::Contextual(_) => err!(
                            Some(self.file),
                            token_group[0].text_range(),
                            "expect custom identifier but got contextual"
                        )?,
                        Identifier::Custom(custom_ident) => custom_ident,
                    },
                    _ => err!(
                        Some(self.file),
                        token_group[0].text_range(),
                        "expect custom identifier"
                    )?,
                };
                let ty = atom::parse_ty(self.symbol_proxy(), &token_group[2..], Some(self.file))?;
                AstKind::MembVarDefn {
                    ident,
                    signature: MembAccessDecl {
                        contract: MembAccessContract::Own,
                        ty,
                    },
                }
            } else {
                todo!()
            },
        )
    }

    pub(super) fn parse_struct_memb_func(
        &mut self,
        token_group: &[Token],
        funcname_idx: usize,
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstKind> {
        enter_block(self);
        self.env.set_value(Env::Func);
        expect_at_least!(token_group, Some(self.file), token_group.into(), 5);
        expect_block_head!(Some(self.file), token_group);
        let head = AtomLRParser::new(
            Some(self.file),
            self.symbol_proxy(),
            &token_group[funcname_idx..],
        )
        .memb_routine_decl(InputContract::Pure, RawMembRoutineKind::Func)?;
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
        Ok(AstKind::MembRoutineDefnHead {
            routine_kind: RoutineKind::Func,
            memb_routine_head: head,
        })
    }

    // pub(super) fn parse_struct_static_func() {}
}
