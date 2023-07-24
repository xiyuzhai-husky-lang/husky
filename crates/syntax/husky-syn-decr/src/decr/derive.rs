use husky_item_tree::RegionPath;
use parsec::{parse_separated_list, parse_separated_list2, StreamParser};

use super::*;

#[salsa::tracked(db = DecrDb, jar = SynDecrJar, constructor = new_inner)]
pub struct DeriveDecr {
    #[id]
    pub decr_id: DecrId,
    pub at_token: AtToken,
    pub derive_token: IdentToken,
    pub lpar_token: LeftParenthesisToken,
    pub trai_exprs: Vec<TraitExpr>,
    pub commas: Vec<CommaToken>,
    pub rpar_token: RightParenthesisToken,
    pub syn_expr_region: SynExprRegion,
}

impl DeriveDecr {
    pub(super) fn new(
        db: &dyn DecrDb,
        path: ItemPath,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        decr_id: DecrId,
    ) -> DecrResult<Self> {
        let parser_factory = DecrParserFactory::new(db, path.module_path(db))?;
        let mut expr_parser =
            parser_factory.expr_parser(decr_id, AllowSelfType::True, AllowSelfValue::False);
        let mut ctx = expr_parser.ctx(None, token_group_idx, None);
        let at_token = ctx
            .try_parse_option()
            .expect("should be guaranteed")
            .expect("should be guaranteed");
        let derive_token = ctx
            .try_parse_option()
            .expect("should be guaranteed")
            .expect("should be guaranteed");
        let lpar_token = ctx.try_parse_expected(OriginalDecrError::ExpectLeftBracketInDerive)?;
        let (traits, commas) = parse_separated_list2(&mut ctx, OriginalDecrError::ExprError)?;
        let rpar_token = ctx.try_parse_expected(OriginalDecrError::ExpectRightBracketInDerive)?;
        Ok(Self::new_inner(
            db,
            decr_id,
            at_token,
            derive_token,
            lpar_token,
            traits,
            commas,
            rpar_token,
            expr_parser.finish(),
        ))
    }
}
