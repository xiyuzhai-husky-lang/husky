use salsa::{storage::HasJar, DbWithJar};

use crate::{style::snake_to_dash, *};

pub trait WordDb: DbWithJar<WordJar> {
    fn it_word_owned(&self, data: String) -> Word;

    fn it_word_borrowed(&self, data: &str) -> Word;

    fn dt_word(&self, data: Word) -> &str;

    fn it_ident_owned(&self, data: String) -> Option<Identifier>;

    fn it_ident_borrowed(&self, data: &str) -> Option<Identifier>;

    fn it_auxiliary_ident_borrowed(&self, data: &str) -> Option<Label>;

    fn dt_ident(&self, data: Identifier) -> &str;

    fn word_db(&self) -> &dyn WordDb;

    fn word_jar(&self) -> &WordJar;

    fn word_menu(&self) -> &WordMenu;

    fn ident_to_dashed(&self, ident: Identifier) -> String;
}

impl<T> WordDb for T
where
    T: DbWithJar<WordJar>,
{
    fn it_word_owned(&self, data: String) -> Word {
        Word::from_owned(self, data)
    }

    fn it_word_borrowed(&self, data: &str) -> Word {
        Word::from_ref(self, data)
    }

    fn dt_word(&self, word: Word) -> &str {
        word.data(self)
    }

    fn word_db(&self) -> &dyn WordDb {
        self
    }

    fn word_jar(&self) -> &WordJar {
        &<Self as HasJar<WordJar>>::jar(self).0
    }

    fn word_menu(&self) -> &WordMenu {
        self.word_jar()
            .word_menu_cell()
            .get_or_init(|| WordMenu::new(self))
    }

    fn it_ident_owned(&self, data: String) -> Option<Identifier> {
        Identifier::from_owned(self, data)
    }

    fn it_ident_borrowed(&self, data: &str) -> Option<Identifier> {
        Identifier::from_borrowed(self, data)
    }

    fn it_auxiliary_ident_borrowed(&self, data: &str) -> Option<Label> {
        Label::from_borrowed(self, data)
    }

    fn dt_ident(&self, ident: Identifier) -> &str {
        ident.data(self)
    }

    fn ident_to_dashed(&self, ident: Identifier) -> String {
        snake_to_dash(self.dt_ident(ident))
    }
}
