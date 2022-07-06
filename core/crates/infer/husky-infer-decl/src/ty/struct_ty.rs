use crate::*;
use vec_like::VecMap;
use word::IdentDict;

pub(crate) fn struct_decl(
    db: &dyn DeclQueryGroup,
    this_ty: EntityRoutePtr,
    generic_parameters: IdentDict<GenericPlaceholder>,
    mut children: AstIter,
) -> InferResultArc<TyDecl> {
    let mut type_members = VecMap::default();
    let mut trait_impls = vec![TraitImplDecl::clone_trait_impl(db, this_ty)];
    // add fields
    while let Some(subitem) = children.next() {
        let subast = subitem.value.as_ref()?;
        match subast.kind {
            AstKind::FieldDefn(ref field_defn) => {
                type_members.insert_new(TypeMemberDecl::Field(Arc::new(FieldDecl {
                    ident: field_defn.ident,
                    contract: field_defn.contract,
                    ty: field_defn.ty,
                })))
            }
            _ => break,
        }
    }
    // add other members
    while let Some(subitem) = children.next() {
        let subast = subitem.value.as_ref()?;
        match subast.kind {
            AstKind::FieldDefn(ref field_defn) => panic!("expect fields to be defined first"),
            AstKind::MembRoutineDefnHead(ref field_defn) => {
                type_members.insert_new(TypeMemberDecl::Method(Arc::new(field_defn.into())))
            }
            _ => panic!(),
        }
    }
    Ok(Arc::new(TyDecl::new(
        db,
        this_ty,
        generic_parameters,
        type_members,
        Default::default(),
        TyKind::Struct,
        trait_impls,
    )))
}
