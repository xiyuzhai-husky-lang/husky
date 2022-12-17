use husky_entity_path::EntityPath;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accessibility {
    PubCrate,              // this is default
    Public,                // everyone can access it
    Protected(EntityPath), // only one module and its submodules
    Private(EntityPath),   // only self
}
