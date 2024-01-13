pub mod synchrotron;

// todo: move these to husky-value-protocol
use self::synchrotron::ValuePresentationSynchrotron;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValuePresentation {
    Unit(()),
    Bool(bool),
    Enum,
    Struct,
    AdHoc(String),
}

pub type EnumU8ValuePresenter =
    fn(u8, &mut ValuePresenterCache, &mut ValuePresentationSynchrotron) -> ValuePresentation;

#[derive(Default)]
pub struct ValuePresenterCache {}
