use crate::DevRuntime;

pub trait AskRuntime {
    fn runtime(&self) -> &DevRuntime;
}
