use atom::symbol_proxy::Symbol;
use defn_head::*;
use fold::LocalStack;
use map_collect::MapCollect;
use print_utils::msg_once;
use static_defn::{StaticEntityDecl, StaticFuncDecl, StaticInputDecl};
use vm::InputContract;
use word::IdentDict;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MorphismDecl {
    pub generic_placeholders: IdentDict<GenericPlaceholder>,
    pub inputs: Vec<InputDecl>,
    pub output: EntityRoutePtr,
}

pub(crate) fn morphism_decl(
    db: &dyn DeclQueryGroup,
    scope: EntityRoutePtr,
) -> InferResultArc<MorphismDecl> {
    let source = db.entity_locus(scope)?;
    return match source {
        EntitySource::Builtin(data) => Ok(Arc::new(todo!())),
        EntitySource::WithinBuiltinModule => todo!(),
        EntitySource::WithinModule {
            file,
            token_group_index,
        } => {
            let ast_text = db.ast_text(file)?;
            let item = ast_text
                .folded_results
                .fold_iter(token_group_index)
                .next()
                .unwrap();
            let ast = item.value.as_ref()?;
            match ast.kind {
                AstKind::RoutineDefnHead(ref head) => todo!(),
                // type constructor
                AstKind::TypeDefnHead {
                    ref kind,
                    ref generic_placeholders,
                    ..
                } => match kind {
                    TyKind::Enum => todo!(),
                    TyKind::Struct => todo!(),
                    TyKind::Record => {
                        let mut inputs = vec![];
                        for subitem in item.children.unwrap() {
                            let subast = subitem.value.as_ref()?;
                            match subast.kind {
                                AstKind::FieldDefn(ref field_var_defn) => inputs.push(InputDecl {
                                    contract: field_var_defn.contract.constructor_input(),
                                    ty: field_var_defn.ty,
                                }),
                                _ => (),
                            }
                        }
                        Ok(Arc::new(MorphismDecl {
                            inputs,
                            output: scope,
                            generic_placeholders: generic_placeholders.clone(),
                        }))
                    }
                    TyKind::Primitive => todo!(),
                    TyKind::Vec => todo!(),
                    TyKind::Array => todo!(),
                    TyKind::Other => todo!(),
                },
                _ => panic!(),
            }
        }
        EntitySource::Module { file: file_id } => todo!(),
        EntitySource::Input { .. } => todo!(),
    };

    fn func_call_decl_from_static(
        db: &dyn DeclQueryGroup,
        static_decl: &StaticFuncDecl,
    ) -> CallDecl {
        let (generic_placeholders, _, symbols) =
            db.parse_generics(static_decl.generic_placeholders);
        let inputs = static_decl.inputs.map(|input| InputDecl {
            ty: db.parse_entity(input.ty, None, &symbols).unwrap(),
            contract: input.contract,
        });
        let output = db.parse_entity(static_decl.output, None, &symbols).unwrap();
        CallDecl {
            generic_placeholders,
            inputs,
            output,
        }
    }
}
