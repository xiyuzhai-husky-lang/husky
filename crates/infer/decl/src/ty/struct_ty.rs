use crate::*;
use vec_map::VecMap;
use word::IdentDict;

pub(crate) fn struct_decl(
    db: &dyn DeclQueryGroup,
    this_ty: EntityRoutePtr,
    generic_placeholders: IdentDict<GenericPlaceholder>,
    mut children: AstIter,
) -> InferResultArc<TyDecl> {
    let mut type_members = VecMap::default();
    let mut trait_impls = vec![TraitImplDecl::clone_trait_impl(db, this_ty)];
    // add fields
    while let Some(subitem) = children.next() {
        let subast = subitem.value.as_ref()?;
        match subast.kind {
            AstKind::FieldDefn(ref field_var_defn) => {
                type_members.insert_new(TypeMemberDecl::Field(Arc::new(FieldDecl {
                    ident: field_var_defn.ident,
                    contract: field_var_defn.contract,
                    ty: field_var_defn.ty,
                })))
            }
            _ => break,
        }
    }
    // add other members
    while let Some(subitem) = children.next() {
        let subast = subitem.value.as_ref()?;
        match subast.kind {
            AstKind::FieldDefn(ref field_var_defn) => panic!("expect fields to be defined first"),
            AstKind::MembRoutineDefnHead(ref field_var_defn) => {
                type_members.insert_new(TypeMemberDecl::Method(Arc::new(field_var_defn.into())))
            }
            _ => panic!(),
        }
    }
    Ok(Arc::new(TyDecl::new(
        db,
        this_ty,
        generic_placeholders,
        type_members,
        Default::default(),
        TyKind::Struct,
        trait_impls,
    )))
}
