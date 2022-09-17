use super::*;
use crate::*;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_match(&mut self, token_group: &[HuskyToken]) -> AstResult<RawStmtVariant> {
        expect_block_head!(token_group);
        let match_expr = self.parse_expr(&token_group[1..(token_group.len() - 1)])?;
        Ok(RawStmtVariant::Match {
            match_expr,
            match_liason: MatchLiason::Pure,
        })
    }

    pub(super) fn parse_case(&mut self, token_group: &[HuskyToken]) -> AstResult<RawStmtVariant> {
        expect_block_head!(token_group);
        if token_group.len() < 3 {
            return err!("expect `case <pattern>:`", token_group.text_range());
        }
        let (pattern, other_atoms) = self
            .parse_atoms(&token_group[1..(token_group.len() - 1)], |mut parser| {
                parser.parse_pattern()
            })?;
        Ok(RawStmtVariant::MatchBranch {
            pattern_branch_variant: RawPatternBranchVariant::Case { pattern },
        })
    }
}
