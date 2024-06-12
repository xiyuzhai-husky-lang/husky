use super::*;
use husky_entity_path::path::assoc_item::AssocItemPath;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct AssocStaticVarFlySignature {
    /// it's
    /// - TypeItemPath, if signature comes from type item
    /// - TraitForItemPath, if signature comes from  trait for type item with custom implementation
    /// - TraitItemPath, if signature comes from  trait for type item following default implementation
    path: AssocItemPath,
    return_ty: FlyTerm,
    ty: FlyTerm,
    instantiation: FlyInstantiation,
}

// # constructor
impl AssocStaticVarFlySignature {
    pub(crate) fn from_ty_assoc_static() -> Self {
        todo!()
    }
}

/// # getters
impl AssocStaticVarFlySignature {
    /// it's
    /// - TypeItemPath, if signature comes from type item
    /// - TraitForItemPath, if signature comes from  trait for type item with custom implementation
    /// - TraitItemPath, if signature comes from  trait for type item following default implementation
    pub fn path(&self) -> AssocItemPath {
        self.path
    }

    pub fn ty(&self) -> FlyTerm {
        self.ty
    }
}
//  {
//                 trai,
//                 trai_item_path,
//                 ..
//             } => {
//                 let ty_result = match trai_item_path.eth_template(db)? {
//                     TraitItemEthTemplate::AssocRitchie(_) => todo!(),
//                     TraitItemEthTemplate::AssocVal(_) => todo!(),
//                     // maybe ty-1 is not entirely correct?
//                     TraitItemEthTemplate::AssocType(_) => Ok(engine.term_menu().ty-1().into()),
//                     TraitItemEthTemplate::AssocStaticMut(_) => todo!(),
//                     TraitItemEthTemplate::AssocStaticVar(_) => todo!(),
//                     TraitItemEthTemplate::MethodRitchie(_) => todo!(),
//                     TraitItemEthTemplate::MethodCurry(_) => todo!(),
//                 };
//                 ty_result
//             }
