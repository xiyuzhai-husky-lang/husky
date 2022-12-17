use super::*;
use husky_token::Token;

impl<'a> AstParser<'a> {
    pub(super) fn parse_impls(&mut self, token_group: TokenGroupIdx, indent: u32) -> Ast {
        let mut parser = ImplHeadParser {
            token_iter: self.token_sheet[token_group].iter().peekable(),
            num_of_langles: 0,
            num_of_pars: 0,
        };
        parser.eat_keyword();
        parser.eat_generics();
        let entity_path = parser.parse_entity_path();
        parser.eat_generics();
        match parser.try_eat_for() {
            Ok(Some(())) => Ast::TraitImpl {
                token_group: todo!(),
                body: todo!(),
            },
            Ok(None) => {
                let x = todo!();
                Ast::TypeImpl {
                    token_group: todo!(),
                    body: todo!(),
                    ty: todo!(),
                }
            }
            Err(e) => Ast::Err(token_group, e),
        }
    }
}

struct ImplHeadParser<'a> {
    token_iter: core::iter::Peekable<core::slice::Iter<'a, Token>>,
    num_of_langles: usize,
    num_of_pars: usize,
}

impl<'a> ImplHeadParser<'a> {
    fn eat_keyword(&mut self) {
        self.token_iter.next();
    }

    fn eat_generics(&mut self) -> AstResult<()> {
        match self.token_iter.peek().unwrap().kind {
            TokenKind::Special(SpecialToken::LAngle) => todo!(),
            _ => Ok(()),
        }
    }

    fn parse_entity_path(&mut self) -> AstResult<EntityPath> {
        todo!()
    }

    fn try_eat_for(&mut self) -> AstResult<Option<()>> {
        todo!()
    }
}
