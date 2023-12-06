use super::*;

impl EtherealTerm {
    /// returns a toolchain except for universe, literals and categories
    pub fn toolchain(self, db: &::salsa::Db) -> Option<Toolchain> {
        match self {
            EtherealTerm::Literal(_) => None,
            EtherealTerm::Symbol(term) => term.toolchain(db),
            EtherealTerm::Variable(term) => term.toolchain(db),
            EtherealTerm::EntityPath(path) => Some(path.toolchain(db)),
            EtherealTerm::Category(_) => todo!(),
            EtherealTerm::Universe(_) => None,
            EtherealTerm::Curry(term) => ethereal_term_curry_toolchain(db, term),
            EtherealTerm::Ritchie(term) => ethereal_term_ritchie_toolchain(db, term),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(term) => ethereal_term_application_toolchain(db, term),
            EtherealTerm::Subitem(_) => todo!(),
            EtherealTerm::AsTraitSubitem(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }

    pub fn item_path_menu(self, db: &::salsa::Db) -> Option<&ItemPathMenu> {
        Some(item_path_menu(db, self.toolchain(db)?))
    }

    pub fn ethereal_term_menu(self, db: &::salsa::Db) -> Option<&EtherealTermMenu> {
        Some(db.ethereal_term_menu(self.toolchain(db)?))
    }
}
