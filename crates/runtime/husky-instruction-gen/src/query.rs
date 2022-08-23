use crate::*;
use entity_kind::TyKind;
use husky_check_utils::should_eq;
use husky_comptime::AskCompileTime;
use husky_file::FilePtr;
use husky_linkage_table::ResolveLinkage;
use husky_package_semantics::PackageQueryGroup;
use infer_decl::DeclQueryGroup;
use vm::{__ResolvedLinkage, __VMResult};

#[salsa::query_group(InstructionGenQueryGroupStorage)]
pub trait InstructionGenQueryGroup: AskCompileTime {
    fn entity_instruction_sheet(&self, route: EntityRoutePtr) -> Option<Arc<InstructionSheet>>;
    fn method_opt_instruction_sheet(
        &self,
        member_route: EntityRoutePtr,
    ) -> Option<Arc<InstructionSheet>>;
    fn dataset_config_instruction_sheet(&self, target_entrance: FilePtr) -> Arc<InstructionSheet>;
    fn enum_literal_to_i32(&self, route: EntityRoutePtr) -> i32;
}

fn entity_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    route: EntityRoutePtr,
) -> Option<Arc<InstructionSheet>> {
    let entity_defn = db.comptime().entity_defn(route).unwrap();
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
        EntityDefnVariant::Ty { .. } => todo!(),
        EntityDefnVariant::Builtin => {
            p!(route.ident());
            todo!()
        }
        EntityDefnVariant::EnumVariant { .. } => todo!(),
        EntityDefnVariant::TyField { .. } => todo!(),
        EntityDefnVariant::TraitAssociatedTypeImpl { ty, .. } => todo!(),
        EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
        EntityDefnVariant::Method {
            ref parameters,
            ref opt_source,
            ..
        } => {
            msg_once!("handle generics");
            match opt_source.as_ref()? {
                CallFormSource::Func { stmts } => Some(new_func_instruction_sheet(
                    db,
                    parameters
                        .iter()
                        .map(|parameter| parameter.ranged_ident.ident),
                    stmts,
                    true, // has_this
                )),
                CallFormSource::Proc { stmts } => todo!(),
                CallFormSource::Lazy { stmts } => todo!(),
                CallFormSource::Static(_) => None,
            }
        }
        EntityDefnVariant::Trait { .. } => todo!(),
        EntityDefnVariant::Function {
            ref spatial_parameters,
            ref parameters,
            output,
            ref source,
        } => todo!(),
        EntityDefnVariant::Input { .. } => todo!(),
        EntityDefnVariant::Any => todo!(),
    }
}

fn method_opt_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    member_route: EntityRoutePtr,
) -> Option<Arc<InstructionSheet>> {
    let ty = member_route.parent();
    let entity_defn = db.comptime().entity_defn(ty).unwrap();
    match entity_defn.variant {
        EntityDefnVariant::Ty {
            ty_members: ref type_members,
            ref variants,
            ty_kind: kind,
            ref trait_impls,
            ref members,
            ..
        } => {
            let method_defn = db.comptime().member_defn(member_route);
            match method_defn.variant {
                EntityDefnVariant::Method {
                    ref parameters,
                    ref opt_source,
                    ..
                } => {
                    let inputs = parameters
                        .iter()
                        .map(|input_placeholder| input_placeholder.ranged_ident.ident);
                    match opt_source.as_ref()? {
                        CallFormSource::Func { stmts } => {
                            Some(new_func_instruction_sheet(db, inputs, stmts, true))
                        }
                        CallFormSource::Proc { stmts } => todo!(),
                        CallFormSource::Lazy { stmts } => todo!(),
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
    target_entrance: FilePtr,
) -> Arc<InstructionSheet> {
    let package = db.comptime().package(target_entrance).unwrap();
    new_func_instruction_sheet(db, vec![].into_iter(), &package.config.dataset.stmts, false)
}

fn enum_literal_to_i32(db: &dyn InstructionGenQueryGroup, route: EntityRoutePtr) -> i32 {
    let ty_decl = db.comptime().ty_decl(route.parent()).unwrap();
    ty_decl
        .variants
        .position(route.ident().custom())
        .unwrap()
        .try_into()
        .unwrap()
}
