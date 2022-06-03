use super::*;

pub(crate) fn vec_decl(db: &dyn DeclQueryGroup) -> Arc<TyDecl> {
    TyDecl::from_static(db, &VEC_TYPE_DEFN)
}
