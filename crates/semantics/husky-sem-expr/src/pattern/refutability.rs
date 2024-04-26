use husky_syn_expr::SynPatternIdx;

struct PatternProduct {
    patterns: Vec<SynPatternIdx>,
}

struct PatternChoices {
    choices: Vec<PatternProduct>,
}

pub enum RefutationTarget {}

fn refute(choices: PatternChoices, target: RefutationTarget) -> PatternChoices {
    todo!()
}
