mod input;
mod output;

pub use input::*;
pub use output::*;

use atom::{
    symbol::{Symbol, SymbolContextKind},
    SymbolContext,
};
use defn_head::*;
use fold::LocalStack;
use implement::Implementor;
use map_collect::MapCollect;
use print_utils::{msg_once, p};
use static_defn::{EntityStaticDefnVariant, StaticInputPlaceholder};
use vm::{InputContract, OutputContract};
use word::IdentDict;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RoutineDecl {
    pub generic_placeholders: IdentDict<GenericPlaceholder>,
    pub inputs: Vec<InputDecl>,
    pub output: OutputDecl,
}

impl RoutineDecl {
    pub fn instantiate(&self, instantiator: &Instantiator) -> Arc<Self> {
        Arc::new(Self {
            generic_placeholders: self
                .generic_placeholders
                .iter()
                .filter_map(|placeholder| instantiator.instantiate_generic_placeholder(placeholder))
                .collect(),
            inputs: self.inputs.map(|input| input.instantiate(instantiator)),
            output: self.output.instantiate(instantiator),
        })
    }
}

impl From<&RoutineDefnHead> for RoutineDecl {
    fn from(head: &RoutineDefnHead) -> Self {
        RoutineDecl {
            generic_placeholders: head.generic_placeholders.clone(),
            inputs: head
                .input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.into())
                .collect(),
            output: OutputDecl {
                ty: head.output_ty.route,
                contract: head.output_contract,
            },
        }
    }
}

pub(crate) fn call_decl(
    db: &dyn DeclQueryGroup,
    route: EntityRoutePtr,
) -> InferResultArc<RoutineDecl> {
    let source = db.entity_source(route)?;
    return match source {
        EntitySource::StaticModuleItem(static_defn) => Ok(match static_defn.variant {
            EntityStaticDefnVariant::Routine { .. } => {
                routine_decl_from_static(db, vec![], static_defn)
            }
            EntityStaticDefnVariant::Type { .. } => {
                db.type_decl(route)?.opt_type_call.clone().expect("todo")
            }
            _ => panic!(),
        }),
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
                AstKind::RoutineDefnHead(ref head) => Ok(Arc::new(head.into())),
                // type constructor
                AstKind::TypeDefnHead { .. } => {
                    let ty_decl = db.type_decl(route)?;
                    Ok(ty_decl.opt_type_call.clone().expect("todo"))

                    // ok_or(InferError {
                    //     variant: InferErrorVariant::Original {
                    //         message: format!("no type call for {:?}", route),
                    //         range: todo!(),
                    //     },
                    //     dev_src: todo!(),
                    // })
                }
                _ => panic!(),
            }
        }
        EntitySource::Module { file: file_id } => todo!(),
        EntitySource::Input { .. } => todo!(),
        EntitySource::StaticTypeMember => todo!(),
    };
}

pub(crate) fn routine_decl_from_static(
    db: &dyn DeclQueryGroup,
    mut symbols: Vec<Symbol>,
    static_defn: &EntityStaticDefn,
) -> Arc<RoutineDecl> {
    match static_defn.variant {
        EntityStaticDefnVariant::Routine {
            ref generic_placeholders,
            ref inputs,
            output_ty,
            output_contract,
            linkage,
            routine_kind,
        } => {
            let generic_placeholders =
                db.parse_generic_placeholders_from_static(generic_placeholders);
            symbols.extend(db.symbols_from_generic_placeholders(&generic_placeholders));
            let symbol_context = SymbolContext {
                opt_package_main: None,
                db: db.upcast(),
                opt_this_ty: None,
                symbols: &symbols,
                kind: SymbolContextKind::Normal,
            };
            let inputs = inputs.map(|input| InputDecl {
                ty: symbol_context.entity_route_from_str(input.ty).unwrap(),
                contract: input.contract,
                ident: db.custom_ident(input.name),
            });
            let output_ty = symbol_context.entity_route_from_str(output_ty).unwrap();
            Arc::new(RoutineDecl {
                generic_placeholders,
                inputs,
                output: OutputDecl {
                    contract: output_contract,
                    ty: output_ty,
                },
            })
        }
        _ => panic!(),
    }
}
