use crate::*;
use vec_map::VecDict;
use word::IdentDict;

pub(crate) fn struct_decl(
    db: &dyn DeclQueryGroup,
    entity_route_kind: EntityRouteKind,
    generic_placeholders: IdentDict<GenericPlaceholder>,
    children: AstIter,
) -> InferResultArc<TyDecl> {
    let mut field_vars = VecDict::default();
    let mut methods = VecDict::default();
    let mut traits = vec![db.entity_route_menu().clone_trait];

    for subitem in children {
        let subast = subitem.value.as_ref()?;
        match subast.kind {
            AstKind::FieldDefn(ref field_var_defn) => field_vars.insert_new(FieldDecl {
                ident: field_var_defn.ident,
                contract: field_var_defn.contract,
                ty: field_var_defn.ty,
            }),
            AstKind::MembRoutineDefnHead(ref field_var_defn) => {
                methods.insert_new(field_var_defn.into())
            }
            _ => panic!(),
        }
    }

    Ok(Arc::new(TyDecl {
        this_type: todo!(),
        generic_placeholders,
        traits,
        fields: todo!(),
        methods,
        variants: todo!(),
        kind: todo!(),
    }))

    //     ::new(
    //     db,
    //     entity_route_kind,
    //     generic_placeholders,
    //     traits,
    //     TyKind::Struct {
    //         fields: Arc::new(field_vars),
    //         methods: field_routines,
    //     },
    // )))
}
