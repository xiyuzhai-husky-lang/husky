use super::*;
use crate::transform::utils::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_morphism_decl(&mut self, token_group: &[Token]) -> AstResult<AstKind> {
        expect_at_least!(token_group, Some(self.file), token_group.into(), 3);
        expect_head!(Some(self.file), token_group);
        expect_at_least!(token_group, Some(self.file), token_group.into(), 5);
        match token_group[2].kind {
            TokenKind::Special(Special::LightArrow) => self.parse_feature_decl(token_group),
            _ => todo!(),
        }
    }

    fn parse_feature_decl(&mut self, token_group: &[Token]) -> AstResult<AstKind> {
        let ident = identify!(Some(self.file), token_group[1]);
        let scope = atom::parse_ty(self.symbol_proxy(), &token_group[3..], Some(self.file))?;
        self.env.set_value(Env::Morphism);
        Ok(AstKind::FeatureDecl {
            ident,
            ty: RangedEntityRoute {
                route: scope,
                range: token_group[3..(token_group.len() - 1)].into(),
            },
        })
    }
}
