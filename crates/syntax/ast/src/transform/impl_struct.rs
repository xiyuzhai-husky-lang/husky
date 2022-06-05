mod impl_associated_call;
mod impl_field;
mod impl_method;
mod impl_visual;

use crate::*;
use atom::context::{Symbol, SymbolKind};
use text::TextRanged;
use token::*;
use word::Paradigm;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_struct_item(
        &mut self,
        token_group: &[Token],
        struct_item_context: StructItemContext,
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstVariant> {
        match token_group[0].kind {
            TokenKind::Keyword(keyword) => {
                self.abs_semantic_tokens.push(AbsSemanticToken::new(
                    SemanticTokenKind::Keyword,
                    token_group[0].range,
                ));
                match keyword {
                    Keyword::Paradigm(routine_keyword) => {
                        match token_group[2].kind {
                            TokenKind::Special(SpecialToken::LPar) => {
                                self.parse_struct_method(token_group, struct_item_context, enter_block)
                            },
                            TokenKind::Special(SpecialToken::LightArrow) =>{
                                self.parse_struct_derived_lazy_field(token_group, struct_item_context, enter_block)
                            },
                            _=> todo!(),
                        }
                    },
                    Keyword::Visual => self.parse_visual(token_group, struct_item_context, enter_block),
                    Keyword::Config(_)
                    | Keyword::Type(_)
                    | Keyword::Stmt(_)
                    | Keyword::Use
                    | Keyword::Mod
                    | Keyword::Main => {
                        p!(self.context, keyword);
                        todo!()
                    },
                    Keyword::Liason(_) =>  self.parse_struct_eager_field(token_group, struct_item_context, enter_block),
                }
            },
            TokenKind::Identifier(_) => {
                self.parse_struct_eager_field(token_group, struct_item_context, enter_block)
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

    fn parse_visual(
        &mut self,
        token_group: &[Token],
        struct_item_context: StructItemContext,
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstVariant> {
        let context_update_result = self.update_struct_item_context(
            struct_item_context,
            StructItemContext::Visual,
            token_group,
        );
        expect_len!(token_group, 2);
        expect_block_head!(token_group);
        enter_block(self);
        self.context.set(AstContext::Visual);
        self.opt_this_liason.set(Some(ParameterLiason::Pure));
        context_update_result?;
        Ok(AstVariant::Visual)
    }
}
