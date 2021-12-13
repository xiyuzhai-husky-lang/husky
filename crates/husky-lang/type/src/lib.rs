mod intern;
mod mem_func;
mod mem_lazy;
mod mem_var;

pub use intern::TypeId;
pub use mem_func::MemFunc;
pub use mem_lazy::MemLazy;
pub use mem_var::MemVar;

use scope::ScopeId;
pub enum Type {
    Named(ScopeId),
    Template {
        template: ScopeId,
        arguments: Vec<TemplateArgument>,
    },
}

pub enum TemplateArgument {
    Type(TypeId),
    Usize(usize),
}
