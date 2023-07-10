use salsa::{storage::HasJar, DbWithJar};

use crate::{style::snake_to_dash, *};

pub trait CowordDb: DbWithJar<CowordJar> {
    fn it_coword_owned(&self, data: String) -> Coword;

    fn it_coword_borrowed(&self, data: &str) -> Coword;

    fn dt_coword(&self, data: Coword) -> &str;

    fn it_ident_owned(&self, data: String) -> Option<Ident>;

    fn it_ident_borrowed(&self, data: &str) -> Option<Ident>;

    fn it_auxiliary_ident_borrowed(&self, data: &str) -> Option<Label>;

    fn dt_ident(&self, data: Ident) -> &str;

    fn word_db(&self) -> &dyn CowordDb;

    fn word_jar(&self) -> &CowordJar;

    fn word_menu(&self) -> &CowordMenu;

    fn ident_to_dashed(&self, ident: Ident) -> String;
}

impl<T> CowordDb for T
where
    T: DbWithJar<CowordJar>,
{
    fn it_coword_owned(&self, data: String) -> Coword {
        Coword::from_owned(self, data)
    }

    fn it_coword_borrowed(&self, data: &str) -> Coword {
        Coword::from_ref(self, data)
    }

    fn dt_coword(&self, word: Coword) -> &str {
        word.data(self)
    }

    fn word_db(&self) -> &dyn CowordDb {
        self
    }

    fn word_jar(&self) -> &CowordJar {
        &<Self as HasJar<CowordJar>>::jar(self).0
    }

    fn word_menu(&self) -> &CowordMenu {
        ident_menu(self)
    }

    fn it_ident_owned(&self, data: String) -> Option<Ident> {
        Ident::from_owned(self, data)
    }

    fn it_ident_borrowed(&self, data: &str) -> Option<Ident> {
        Ident::from_borrowed(self, data)
    }

    fn it_auxiliary_ident_borrowed(&self, data: &str) -> Option<Label> {
        Label::from_borrowed(self, data)
    }

    fn dt_ident(&self, ident: Ident) -> &str {
        ident.data(self)
    }

    fn ident_to_dashed(&self, ident: Ident) -> String {
        snake_to_dash(self.dt_ident(ident))
    }
}
