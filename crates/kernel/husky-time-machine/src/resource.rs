mod idx;
mod lifetime;
mod variable;

pub use idx::*;
pub use lifetime::*;
pub use variable::*;

use local_stack::LocalStack;

#[derive(Default)]
pub struct ResourceStack(LocalStack<Resource>);

pub enum Resource {
    Variable(VariableResource),
    Lifetime(LifetimeResource),
}
