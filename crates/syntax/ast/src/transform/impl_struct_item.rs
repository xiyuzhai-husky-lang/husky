use crate::*;
use atom::symbol::{Symbol, SymbolKind};
use text::TextRanged;
use token::*;
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
                Keyword::Routine(routine_keyword) => self.parse_struct_method(token_group, enter_block),
                Keyword::Type(_) => todo!(),
                Keyword::Stmt(_) => todo!(),
                Keyword::Def => todo!(),
                Keyword::Use => todo!(),
                Keyword::Mod => todo!(),
                Keyword::Main => todo!(),
                Keyword::Visual => {
                    expect_len!(token_group, 2);
                    expect_block_head!(token_group);
                    enter_block(self);
                    self.context.set(AstContext::Visual);
                    Ok(AstKind::Visual)},
            },
            TokenKind::Identifier(_) => self.parse_struct_field(token_group, ),
            TokenKind::Decorator(_) => self.parse_struct_associated_routine(token_group, enter_block), 
            _ => err!(
                format!("expect <keyword> or <identifier> or <decorator> at the beginning of each struct item"),
                token_group.text_range()
            ),
        }
    }

    pub(super) fn parse_struct_field(&mut self, token_group: &[Token]) -> AstResult<AstKind> {
        if token_group.len() > 2 && token_group[1].kind == TokenKind::Special(Special::Colon) {
            let ident = identify_token!(self, token_group[0], SemanticTokenKind::Field);
            let ty = atom::parse_route(&self.symbol_context(), &token_group[2..])?;
            Ok(AstKind::FieldDefnHead(FieldDefnHead {
                ident,
                contract: FieldContract::Own,
                ty,
                kind: FieldKind::StructOriginal,
            }))
        } else {
            err!("expect <identifier>: <type>", token_group.text_range())
        }
    }

    pub(super) fn parse_struct_method(
        &mut self,
        token_group: &[Token],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstKind> {
        enter_block(self);
        let routine_keyword = match token_group[0].kind { 
            TokenKind::Keyword(Keyword::Routine(routine_keyword)) => routine_keyword,
            _=> todo!(),
        };
        self.context.set(AstContext::Routine(routine_keyword));
        expect_at_least!(token_group, token_group.text_range(), 5);
        expect_block_head!(token_group);
        const funcname_idx: usize = 1;
        let head = self.parse_atoms(&token_group[funcname_idx..], |parser| {
            parser.method_decl(InputContract::Pure, RoutineKeyword::Func)
        })?;
        self.opt_this_contract.set(Some(head.this_contract));
        self.symbols.extend(
            head.input_placeholders
                .iter()
                .map(|parameter| Symbol::variable(parameter.ident)),
        );
        Ok(AstKind::TypeMethodDefnHead(head))
    }

    pub(super) fn parse_struct_associated_routine(
        &mut self,
        token_group: &[Token], 
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstKind> {
        enter_block(self);
        let routine_keyword = match token_group[1].kind { 
            TokenKind::Keyword(Keyword::Routine(routine_keyword)) => routine_keyword,
            _=> todo!(),
        };
        self.context.set(AstContext::Routine(routine_keyword));
        expect_at_least!(token_group, token_group.text_range(), 5);
        expect_block_head!(token_group);
        const funcname_idx: usize = 2;
        let head = self.parse_atoms(&token_group[funcname_idx..], |parser| {
            parser.routine_defn_head(routine_keyword)
        })?;
        self.opt_this_contract.set(None);
        self.opt_this_ty.set(None);
        self.symbols.extend(
            head.parameters
                .iter()
                .map(|parameter| Symbol::variable(parameter.ident)),
        );
        Ok(AstKind::TypeAssociatedRoutineDefnHead(head))
    }
}
