use super::*;
use husky_expr::PatternExprIdx;

impl std::ops::Index<PatternExprIdx> for SignatureRegion {
    type Output = Liason;

    fn index(&self, index: PatternExprIdx) -> &Self::Output {
        &self.liasons[index]
    }
}
