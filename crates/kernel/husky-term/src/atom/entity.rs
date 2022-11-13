mod namespace;
mod root;

pub use namespace::*;






use crate::{TermDb};

// #[derive(Debug, PartialEq, Eq, Hash)]
// pub struct TermEntity {
//     path: EntityPathItd,
//     ty: Ty,
// }

// impl TermEntity {
//     pub fn ty(&self) -> Ty {
//         self.ty
//     }

//     pub fn path(&self) -> EntityPathItd {
//         self.path
//     }
// }

// impl Into<Term> for TermEntity {
//     fn into(self) -> Term {
//         Term::AtomTermTerm::Entity(self)
//     }
// }

#[test]
fn display_term() {
    use crate::tests::TermTestsDb;
    let db = TermTestsDb::new();
    let menu = db.term_menu();
    assert_eq!(menu.unit().to_string(), "void");
    assert_eq!(menu.i32().to_string(), "i32");
    assert_eq!(menu.i64().to_string(), "i64");
    assert_eq!(menu.f32().to_string(), "f32");
    assert_eq!(menu.f64().to_string(), "f64");
    assert_eq!(menu.b32().to_string(), "b32");
    assert_eq!(menu.b64().to_string(), "b64");
    assert_eq!(menu.bool().to_string(), "bool");
}
