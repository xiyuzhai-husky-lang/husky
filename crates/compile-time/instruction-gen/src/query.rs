use check_utils::should_eq;
use entity_kind::TyKind;
use file::FilePtr;
use linkage_table::ResolveLinkage;
use pack_semantics::PackageQueryGroup;
use vm::{BoxedValue, EvalValue, Linkage, MemberValue, StackValue, VMResult};

use crate::*;

#[salsa::query_group(InstructionGenQueryGroupStorage)]
pub trait InstructionGenQueryGroup:
    EntityDefnQueryGroup + PackageQueryGroup + ResolveLinkage
{
    fn entity_instruction_sheet(&self, route: EntityRoutePtr) -> Arc<InstructionSheet>;
    fn method_instruction_sheet(&self, member_route: EntityRoutePtr) -> Arc<InstructionSheet>;
    fn dataset_config_instruction_sheet(&self, pack_main: FilePtr) -> Arc<InstructionSheet>;
}

fn entity_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    route: EntityRoutePtr,
) -> Arc<InstructionSheet> {
    let entity_defn = db.entity_defn(route).unwrap();
    match entity_defn.variant {
        EntityDefnVariant::Module { .. } => todo!(),
        EntityDefnVariant::Feature { .. } => todo!(),
        EntityDefnVariant::Pattern { .. } => todo!(),
        EntityDefnVariant::Func {
            ref input_placeholders,
            ref stmts,
            ..
        } => InstructionSheetBuilder::new_func(
            db,
            input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.ident.ident),
            stmts,
            false,
        ),
        EntityDefnVariant::Proc {
            ref input_placeholders,
            ref stmts,
            ..
        } => InstructionSheetBuilder::new_impr(
            db,
            input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.ident.ident),
            stmts,
            false,
        ),
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
        EntityDefnVariant::Method { .. } => todo!(),
        EntityDefnVariant::Trait { .. } => todo!(),
    }
}

fn method_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    member_route: EntityRoutePtr,
) -> Arc<InstructionSheet> {
    let ty = member_route.parent();
    let entity_defn = db.entity_defn(ty).unwrap();
    match entity_defn.variant {
        EntityDefnVariant::Main(_) => todo!(),
        EntityDefnVariant::Module {} => todo!(),
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
                    ref input_placeholders,
                    ..
                } => {
                    let inputs = input_placeholders
                        .iter()
                        .map(|input_placeholder| input_placeholder.ident.ident);
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
                            InstructionSheetBuilder::new_func(db, inputs, stmts, true)
                        }
                        MethodSource::Proc { stmts } => todo!(),
                        MethodSource::Pattern { stmts } => todo!(),
                        MethodSource::Static(_) => todo!(),
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
    InstructionSheetBuilder::new_func(db, vec![].into_iter(), &pack.config.dataset.stmts, false)
}

// fn virtual_vec_method_linkages(db: &dyn InstructionGenQueryGroup) -> Arc<IdentPairDict<Linkage>> {
//     let mut field_routine_linkages = IdentDict::default();
//     field_routine_linkages.insert_new((
//         db.intern_word("clone").opt_custom().unwrap(),
//         Linkage {
//             call: virtual_vec_clone,
//             nargs: 1,
//         },
//     ));
//     field_routine_linkages.insert_new((
//         db.intern_word("len").opt_custom().unwrap(),
//         Linkage {
//             call: virtual_vec_len,
//             nargs: 1,
//         },
//     ));
//     field_routine_linkages.insert_new((
//         db.intern_word("push").opt_custom().unwrap(),
//         Linkage {
//             call: virtual_vec_push,
//             nargs: 2,
//         },
//     ));
//     field_routine_linkages.insert_new((
//         db.intern_word("pop").opt_custom().unwrap(),
//         Linkage {
//             call: virtual_vec_pop,
//             nargs: 1,
//         },
//     ));
//     field_routine_linkages.insert_new((
//         db.intern_word("first").opt_custom().unwrap(),
//         Linkage {
//             call: virtual_vec_first,
//             nargs: 1,
//         },
//     ));
//     field_routine_linkages.insert_new((
//         db.intern_word("last").opt_custom().unwrap(),
//         Linkage {
//             call: virtual_vec_last,
//             nargs: 1,
//         },
//     ));
//     Arc::new(field_routine_linkages)
// }
