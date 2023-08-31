#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LinkagePath {}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum LinkagePath {
//     FnCall {},
//     MethodFnCall {},
//     FieldAccess {},
// }

// impl LinkagePath {
//     pub fn deps(self, db: &dyn HirDepsDb) -> HirLinkageDeps {
//         todo!()
//     }
// }
