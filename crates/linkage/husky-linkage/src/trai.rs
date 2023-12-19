use crate::instantiation::{LinkageInstantiate, LinkageInstantiation};
use husky_hir_ty::trai::HirTrait;

pub struct LinkageTrait {}

impl LinkageInstantiate for HirTrait {
    type Output = LinkageTrait;

    fn linkage_instantiate(
        self,
        linkage_instantiation: &LinkageInstantiation,
        db: &salsa::Db,
    ) -> Self::Output {
        todo!()
    }
}
