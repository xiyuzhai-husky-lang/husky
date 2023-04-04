use crate::*;

pub trait LocalTermEngine<'a> {
    fn db(&self) -> &'a dyn TermDb;
    fn unresolved_terms(&self) -> &UnresolvedTerms;
}
