// todo: move these to husky-value-protocol
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValuePresentation {
    Bool(bool),
    Enum,
    Struct,
    AdHoc(String),
}

pub type EnumU8ValuePresenter =
    fn(u8, &mut ValuePresenterCache, &mut ValuePresentationSynchrotron) -> ValuePresentation;

#[derive(Default)]
pub struct ValuePresenterCache {}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValuePresentationSynchrotron {}
impl ValuePresentationSynchrotron {
    pub fn diff_actions(&self) -> &[ValuePresentationSynchrotronAction] {
        // ad hoc: todo!()
        &[]
    }

    pub fn status(&self) -> ValuePresentationSynchrotronStatus {
        ValuePresentationSynchrotronStatus {}
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValuePresentationSynchrotronStatus {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValuePresentationSynchrotronAction {}
