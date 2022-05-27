use crate::*;
use check_utils::should_eq;
use entity_kind::TyKind;
use file::FilePtr;
use linkage_table::ResolveLinkage;
use pack_semantics::PackageQueryGroup;
use vm::{EvalValue, Linkage, MemberValue, OwnedValue, VMRuntimeResult, VMValue};

#[salsa::query_group(InstructionGenQueryGroupStorage)]
pub trait InstructionGenQueryGroup:
    EntityDefnQueryGroup + PackageQueryGroup + ResolveLinkage
{
    fn entity_instruction_sheet(&self, route: EntityRoutePtr) -> Option<Arc<InstructionSheet>>;
    fn method_opt_instruction_sheet(
        &self,
        member_route: EntityRoutePtr,
    ) -> Option<Arc<InstructionSheet>>;
    fn dataset_config_instruction_sheet(&self, pack_main: FilePtr) -> Arc<InstructionSheet>;
    fn enum_literal_as_u8(&self, route: EntityRoutePtr) -> u8;
}

fn entity_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    route: EntityRoutePtr,
) -> Option<Arc<InstructionSheet>> {
    let entity_defn = db.entity_defn(route).unwrap();
    match entity_defn.variant {
        EntityDefnVariant::Module { .. } => todo!(),
        EntityDefnVariant::Feature { .. } => todo!(),
        EntityDefnVariant::Pattern { .. } => todo!(),
        EntityDefnVariant::Func {
            ref input_placeholders,
            ref stmts,
            ..
        } => Some(new_func_instruction_sheet(
            db,
            input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.ranged_ident.ident),
            stmts,
            false,
        )),
        EntityDefnVariant::Proc {
            ref parameters,
            ref stmts,
            ..
        } => Some(new_proc_instruction_sheet(
            db,
            parameters
                .iter()
                .map(|parameter| parameter.ranged_ident.ident),
            stmts,
            false, // has_this
        )),
        EntityDefnVariant::Type { .. } => todo!(),
        EntityDefnVariant::Main(_) => todo!(),
        EntityDefnVariant::Builtin => {
            p!(route.ident());
            todo!()
        }
        EntityDefnVariant::EnumVariant { .. } => todo!(),
        EntityDefnVariant::TypeField { .. } => todo!(),
        EntityDefnVariant::TraitAssociatedTypeImpl { ty, .. } => todo!(),
        EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
        EntityDefnVariant::Method {
            ref parameters,
            ref method_variant,
            ..
        } => {
            msg_once!("handle generics");
            match method_variant {
                MethodDefnVariant::TypeMethod { ty, method_source } => match method_source {
                    MethodSource::Func { stmts } => Some(new_func_instruction_sheet(
                        db,
                        parameters
                            .iter()
                            .map(|parameter| parameter.ranged_ident.ident),
                        stmts,
                        true, // has_this
                    )),
                    MethodSource::Proc { stmts } => todo!(),
                    MethodSource::Pattern { stmts } => todo!(),
                    MethodSource::Static(_) => todo!(),
                },
                MethodDefnVariant::TraitMethod {
                    trai,
                    opt_default_source,
                } => todo!(),
                MethodDefnVariant::TraitMethodImpl { trai, opt_source } => todo!(),
            }
        }
        EntityDefnVariant::Trait { .. } => todo!(),
    }
}

fn method_opt_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    member_route: EntityRoutePtr,
) -> Option<Arc<InstructionSheet>> {
    let ty = member_route.parent();
    let entity_defn = db.entity_defn(ty).unwrap();
    match entity_defn.variant {
        EntityDefnVariant::Main(_) => todo!(),
        EntityDefnVariant::Module { .. } => todo!(),
        EntityDefnVariant::Feature { .. } => todo!(),
        EntityDefnVariant::Pattern {} => todo!(),
        EntityDefnVariant::Func { .. } => todo!(),
        EntityDefnVariant::Proc { .. } => todo!(),
        EntityDefnVariant::Type {
            ty_members: ref type_members,
            ref variants,
            kind,
            ref trait_impls,
            ref members,
            ..
        } => {
            let method_defn = db.member_defn(member_route);
            match method_defn.variant {
                EntityDefnVariant::Method {
                    ref method_variant,
                    parameters: ref input_placeholders,
                    ..
                } => {
                    let inputs = input_placeholders
                        .iter()
                        .map(|input_placeholder| input_placeholder.ranged_ident.ident);
                    let source = match method_variant {
                        MethodDefnVariant::TypeMethod { ty, method_source } => method_source,
                        MethodDefnVariant::TraitMethod {
                            trai,
                            opt_default_source,
                        } => todo!(),
                        MethodDefnVariant::TraitMethodImpl { trai, opt_source } => todo!(),
                    };
                    match source {
                        MethodSource::Func { stmts } => {
                            Some(new_func_instruction_sheet(db, inputs, stmts, true))
                        }
                        MethodSource::Proc { stmts } => todo!(),
                        MethodSource::Pattern { stmts } => todo!(),
                        MethodSource::Static(_) => None,
                    }
                }
                _ => panic!(),
            }
        }
        EntityDefnVariant::Builtin => todo!(),
        EntityDefnVariant::EnumVariant { .. } => todo!(),
        EntityDefnVariant::TypeField { .. } => todo!(),
        EntityDefnVariant::TraitAssociatedTypeImpl { ty, .. } => todo!(),
        EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
        EntityDefnVariant::Method { .. } => todo!(),
        EntityDefnVariant::Trait { .. } => todo!(),
    }
}

fn dataset_config_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    pack_main: FilePtr,
) -> Arc<InstructionSheet> {
    let pack = db.package(pack_main).unwrap();
    new_func_instruction_sheet(db, vec![].into_iter(), &pack.config.dataset.stmts, false)
}

fn enum_literal_as_u8(db: &dyn InstructionGenQueryGroup, route: EntityRoutePtr) -> u8 {
    let ty_decl = db.ty_decl(route.parent()).unwrap();
    ty_decl
        .variants
        .position(route.ident().custom())
        .unwrap()
        .try_into()
        .unwrap()
}
