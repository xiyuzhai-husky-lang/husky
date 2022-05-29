use super::*;
use token::*;
use word::Paradigm;

impl<'a> AstTransformer<'a> {
    // pub(super) fn parse_morphism_decl(&mut self, token_group: &[Token]) -> AstResult<AstKind> {
    //     expect_at_least!(token_group, token_group.text_range(), 3);
    //     expect_head!(token_group);
    //     expect_at_least!(token_group, token_group.text_range(), 5);
    //     match token_group[2].kind {
    //         TokenKind::Special(Special::LightArrow) => self.parse_feature_decl(token_group),
    //         _ => todo!(),
    //     }
    // }

    fn parse_feature_defn_head(&mut self, token_group: &[Token]) -> AstResult<AstKind> {
        let ident = identify_token!(
            self,
            token_group[1],
            SemanticTokenKind::Entity(EntityKind::Feature)
        );
        let ty = atom::parse_route(self, &token_group[3..])?;
        self.context.set(AstContext::Stmt(Paradigm::LazyFunctional));
        Ok(AstKind::FeatureDecl { ident, ty })
    }
}
