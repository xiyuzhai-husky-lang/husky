use crate::*;
use atom::symbol::{Symbol, SymbolKind};
use text::TextRanged;
use token::*;
use vm::{FieldLiason, InputLiason};
use word::RoutineKeyword;

// the order is:
// 1. original fields
// 2. derived fields
// 3. static callables
// 4. methods
// 5. trait impls
// 6. visualization
impl<'a> AstTransformer<'a> {
    pub(super) fn parse_struct_item(
        &mut self,
        token_group: &[Token],
        struct_item_context: StructItemContext,
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstKind> {
        match token_group[0].kind {
            TokenKind::Keyword(keyword) => {
                self.abs_semantic_tokens.push(AbsSemanticToken::new(
                    SemanticTokenKind::Keyword,
                    token_group[0].range,
                ));
                match keyword {
                    Keyword::Routine(routine_keyword) => {
                        self.parse_struct_method(token_group, struct_item_context, enter_block)
                    },
                    Keyword::Visual => self.parse_visual(token_group, struct_item_context, enter_block),
                    Keyword::Config(_)
                    | Keyword::Type(_)
                    | Keyword::Stmt(_)
                    | Keyword::Def
                    | Keyword::Use
                    | Keyword::Mod
                    | Keyword::Main => {
                        p!(self.context, keyword);
                        todo!()
                    },
                }
            },
            TokenKind::Identifier(_) => {
                self.parse_struct_field(token_group, struct_item_context, )
            },
            TokenKind::Decorator(_) => {
                self.parse_struct_associated_routine(token_group, struct_item_context, enter_block)
            },
            _ => err!(
                format!("expect <keyword> or <identifier> or <decorator> at the beginning of each struct item"),
                token_group.text_range()
            ),
        }
    }

    fn update_struct_item_context(
        &mut self,
        old_struct_item_context: StructItemContext,
        new_struct_item_context: StructItemContext,
        token_group: &[Token],
    ) -> AstResult<()> {
        if old_struct_item_context < new_struct_item_context {
            self.context
                .set(AstContext::Struct(new_struct_item_context));
            Ok(())
        } else if old_struct_item_context > new_struct_item_context {
            err!(
                format!("old context is `{:?}`", old_struct_item_context),
                token_group.text_range()
            )
        } else {
            Ok(())
        }
    }

    pub(super) fn parse_struct_field(
        &mut self,
        token_group: &[Token],
        struct_item_context: StructItemContext,
    ) -> AstResult<AstKind> {
        msg_once!("derived field");
        self.update_struct_item_context(
            struct_item_context,
            StructItemContext::OriginalField,
            token_group,
        )?;
        if token_group.len() > 2 && token_group[1].kind == TokenKind::Special(Special::Colon) {
            let ident = identify_token!(self, token_group[0], SemanticTokenKind::Field);
            let ty = atom::parse_route(&self.symbol_context(), &token_group[2..])?;
            Ok(AstKind::FieldDefnHead(FieldDefnHead {
                ident,
                contract: FieldLiason::Own,
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
        struct_item_context: StructItemContext,
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstKind> {
        // even if context update is not successful, must set context correctly for children
        let context_update_result = self.update_struct_item_context(
            struct_item_context,
            StructItemContext::Method,
            token_group,
        );
        enter_block(self);
        let routine_keyword = match token_group[0].kind {
            TokenKind::Keyword(Keyword::Routine(routine_keyword)) => routine_keyword,
            _ => todo!(),
        };
        self.context.set(AstContext::Routine(routine_keyword));
        context_update_result?;
        expect_at_least!(token_group, token_group.text_range(), 5);
        expect_block_head!(token_group);
        const funcname_idx: usize = 1;
        let head = self.parse_atoms(&token_group[funcname_idx..], |parser| {
            parser.method_decl(InputLiason::Pure, RoutineKeyword::Func)
        })?;
        self.opt_this_contract.set(Some(head.this_contract));
        self.symbols.extend(
            head.input_placeholders
                .iter()
                .map(|parameter| Symbol::variable(parameter.ranged_ident)),
        );
        Ok(AstKind::TypeMethodDefnHead(head))
    }

    pub(super) fn parse_struct_associated_routine(
        &mut self,
        token_group: &[Token],
        struct_item_context: StructItemContext,
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstKind> {
        let context_update_result = self.update_struct_item_context(
            struct_item_context,
            StructItemContext::AssociatedCall,
            token_group,
        )?;
        enter_block(self);
        let routine_keyword = match token_group[1].kind {
            TokenKind::Keyword(Keyword::Routine(routine_keyword)) => routine_keyword,
            _ => todo!(),
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
                .map(|parameter| Symbol::variable(parameter.ranged_ident)),
        );
        Ok(AstKind::TypeAssociatedRoutineDefnHead(head))
    }

    fn parse_visual(
        &mut self,
        token_group: &[Token],
        struct_item_context: StructItemContext,
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstKind> {
        let context_update_result = self.update_struct_item_context(
            struct_item_context,
            StructItemContext::Visual,
            token_group,
        );
        expect_len!(token_group, 2);
        expect_block_head!(token_group);
        enter_block(self);
        self.context.set(AstContext::Visual);
        self.opt_this_contract.set(Some(InputLiason::Pure));
        context_update_result?;
        Ok(AstKind::Visual)
    }
}
