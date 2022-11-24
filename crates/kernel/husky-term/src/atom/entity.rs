mod namespace;
mod root;

pub use namespace::*;

// #[derive(Debug, PartialEq, Eq, Hash)]
// pub struct TermEntity {
//     path: EntityPath,
//     ty: Term,
// }

// impl TermEntity {
//     pub fn ty(&self) -> Term {
//         self.ty
//     }

//     pub fn path(&self) -> EntityPath {
//         self.path
//     }
// }

// impl Into<TermData> for TermEntity {
//     fn into(self) -> TermData {
//         TermData::AtomTermTerm::Entity(self)
//     }
// }
