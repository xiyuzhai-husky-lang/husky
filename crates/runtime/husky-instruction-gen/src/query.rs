use crate::*;
use husky_comptime::ComptimeQueryGroup;
use husky_file::FileItd;
use husky_package_semantics::PackageQueryGroup;

#[salsa::query_group(InstructionGenQueryGroupStorage)]
pub trait InstructionGenQueryGroup: ComptimeQueryGroup {
    fn entity_instruction_sheet(&self, route: EntityRoutePtr) -> Option<Arc<InstructionSheet>>;
    fn method_opt_instruction_sheet(
        &self,
        member_route: EntityRoutePtr,
    ) -> Option<Arc<InstructionSheet>>;
    fn dataset_config_instruction_sheet(&self, target_entrance: FileItd) -> Arc<InstructionSheet>;
    fn enum_literal_to_i32(&self, route: EntityRoutePtr) -> i32;
}

fn entity_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    route: EntityRoutePtr,
) -> Option<Arc<InstructionSheet>> {
    let entity_defn = db.entity_defn(route).unwrap();
    match entity_defn.variant {
        EntityDefnVariant::Module { .. } => todo!(),
        EntityDefnVariant::Feature { .. } => todo!(),
        EntityDefnVariant::Func {
            ref parameters,
            ref stmts,
            ..
        } => Some(new_func_instruction_sheet(
            db,
            parameters
                .iter()
                .map(|input_placeholder| input_placeholder.ranged_ident().ident),
            stmts,
            false,
        )),
        EntityDefnVariant::Proc {
            ref parameters,
            ref stmts,
            ..
        } => Some(new_proc_instruction_sheet(
            db,
            parameters.iter().map(|parameter| parameter.ident()),
            stmts,
            false, // has_this
        )),
        EntityDefnVariant::Ty { .. } => todo!(),
        EntityDefnVariant::Builtin => {
            p!(route.ident());
            todo!()
        }
        EntityDefnVariant::EnumVariant { .. } => todo!(),
        EntityDefnVariant::TyField { .. } => todo!(),
        EntityDefnVariant::TraitAssociatedTypeImpl { .. } => todo!(),
        EntityDefnVariant::TraitAssociatedConstSizeImpl { .. } => todo!(),
        EntityDefnVariant::Method {
            ref parameters,
            ref opt_source,
            ..
        } => {
            msg_once!("handle generics");
            match opt_source.as_ref()? {
                CallFormSource::Func { stmts } => Some(new_func_instruction_sheet(
                    db,
                    parameters.iter().map(|parameter| parameter.ident()),
                    stmts,
                    true, // has_this
                )),
                CallFormSource::Proc { .. } => todo!(),
                CallFormSource::Lazy { .. } => todo!(),
                CallFormSource::Static(_) => None,
            }
        }
        EntityDefnVariant::Trait { .. } => todo!(),
        EntityDefnVariant::Function { .. } => todo!(),
        EntityDefnVariant::TargetInput { .. } => todo!(),
        EntityDefnVariant::Any => todo!(),
    }
}

fn method_opt_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    member_route: EntityRoutePtr,
) -> Option<Arc<InstructionSheet>> {
    let ty = member_route.parent();
    let entity_defn = db.entity_defn(ty).unwrap();
    match entity_defn.variant {
        EntityDefnVariant::Ty { .. } => {
            let method_defn = db.member_defn(member_route);
            match method_defn.variant {
                EntityDefnVariant::Method {
                    ref parameters,
                    ref opt_source,
                    ..
                } => {
                    let inputs = parameters
                        .iter()
                        .map(|input_placeholder| input_placeholder.ranged_ident().ident);
                    match opt_source.as_ref()? {
                        CallFormSource::Func { stmts } => {
                            Some(new_func_instruction_sheet(db, inputs, stmts, true))
                        }
                        CallFormSource::Proc { stmts } => {
                            Some(new_proc_instruction_sheet(db, inputs, stmts, true))
                        }
                        CallFormSource::Lazy { .. } => todo!(),
                        CallFormSource::Static(_) => None,
                    }
                }
                _ => panic!(),
            }
        }
        _ => todo!(),
    }
}

fn dataset_config_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    target_entrance: FileItd,
) -> Arc<InstructionSheet> {
    let package = db.package(target_entrance).unwrap();
    new_func_instruction_sheet(db, vec![].into_iter(), &package.config.dataset.stmts, false)
}

fn enum_literal_to_i32(db: &dyn InstructionGenQueryGroup, route: EntityRoutePtr) -> i32 {
    todo!()
    // let ty_decl = db.ty_decl(route.parent()).unwrap();
    // ty_decl
    //     .variants
    //     .position(route.ident().custom())
    //     .unwrap()
    //     .try_into()
    //     .unwrap()
}
