use crate::*;

// mom
impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Literal(_) => todo!(),
            Term::Entity(_) => todo!(),
            Term::Curry(_) => todo!(),
            Term::Variable(_) => todo!(),
            Term::Abstraction(_) => todo!(),
            Term::Application(_) => todo!(),
            Term::Universe(_) => todo!(),
        }
    }
}

impl std::fmt::Display for TermLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.data() {
            TermLiteralData::I32(_) => todo!(),
        }
    }
}

impl std::fmt::Display for TermEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.path().fmt(f)
    }
}
