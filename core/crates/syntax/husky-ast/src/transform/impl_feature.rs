use super::*;
use husky_token::*;
use husky_word::Paradigm;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_feature_defn_head(
        &mut self,
        paradigm: Paradigm,
        token_group: &[HuskyToken],
    ) -> AstResult<AstVariant> {
        expect_head!(token_group);
        expect_at_least!(token_group, token_group.text_range(), 5);
        let ident = identify_token!(
            self,
            token_group[1],
            SemanticTokenKind::Entity(EntityKind::Feature)
        );
        let return_ty = husky_atom::parse_route(self, &token_group[3..])?;
        self.context.set(AstContext::Stmt {
            paradigm,
            return_context: Some(ReturnContext {
                return_ty,
                kind: ReturnContextKind::Feature,
            }),
        });
        Ok(AstVariant::FeatureDefnHead {
            paradigm,
            ident,
            return_ty,
        })
    }
}
