use husky_entity_path::EntityPath;

pub enum Accessibility {
    PubCrate(EntityPath),  // this is default
    Public,                // everyone can access it
    Protected(EntityPath), // only one module and its submodules
    Private(EntityPath),   // only self
}
