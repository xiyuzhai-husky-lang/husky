use crate::foundations::{
    elem, Behave, Behaviour, PlainText, Repr, TypstContentRefined, Unlabellable,
};
use ecow::EcoString;

/// A text space.
#[elem(Behave, Unlabellable, PlainText, Repr)]
pub struct SpaceElem {}

impl Repr for SpaceElem {
    fn repr(&self) -> EcoString {
        "[ ]".into()
    }
}

impl Behave for TypstContentRefined<SpaceElem> {
    fn behaviour(&self) -> Behaviour {
        Behaviour::Weak(2)
    }
}

impl Unlabellable for TypstContentRefined<SpaceElem> {}

impl PlainText for TypstContentRefined<SpaceElem> {
    fn plain_text(&self, text: &mut EcoString) {
        text.push(' ');
    }
}
