use crate::foundations::{
    elem, Behave, Behaviour, PlainText, TypstContentRefined, TypstValueRepr, Unlabellable,
};
use ecow::EcoString;

/// A text space.
#[elem(Behave, Unlabellable, PlainText, Repr)]
pub struct SpaceElem {}

impl TypstValueRepr for SpaceElem {
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
