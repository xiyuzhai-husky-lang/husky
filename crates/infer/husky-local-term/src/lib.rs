use husky_term::*;

pub enum LocalTerm {
    Term(Term),
    ImplicitLifetime(u8),
}
