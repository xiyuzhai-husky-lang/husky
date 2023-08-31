use crate::DevRuntime;

// husky_ast::AstJar
#[salsa::db()]
pub struct DevRuntimeDb {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DevRuntimeDb {}

impl salsa::ParallelDatabase for DevRuntimeDb {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        todo!()
    }
}
