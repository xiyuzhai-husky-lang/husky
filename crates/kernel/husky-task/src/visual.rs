use crate::*;

pub trait IsVisualProtocol {
    type VisualElement;
}

pub type VisualProtocol<Task> = <<Task as IsTask>::DevAscension as IsDevAscension>::VisualProtocol;
pub type VisualElement<Task> = <VisualProtocol<Task> as IsVisualProtocol>::VisualElement;
