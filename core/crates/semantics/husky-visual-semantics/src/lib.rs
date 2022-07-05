// use avec::Avec;
// use husky_eager_semantics::FuncStmt;
// use husky_lazy_semantics::LazyStmt;
// use std::sync::Arc;

// // #[derive(Debug, Clone, PartialEq, Eq)]
// // pub enum VisualizerSource {
// //     Static(&'static StaticVisualizer),
// //     Custom { stmts: Avec<LazyStmt> },
// // }
// use crate::*;
// use husky_compile_time::AskCompileTime;
// use husky_entity_semantics::{EntityDefnQueryGroup, EntityDefnVariant};
// use husky_entity_syntax::EntityLocus;
// use husky_instruction_gen::{new_visual_instruction_sheet, InstructionGenQueryGroup};
// use print_utils::p;
// use static_defn::EntityStaticDefnVariant;
// use upcast::Upcast;

// #[salsa::query_group(VisualQueryGroupStorage)]
// pub trait VisualQueryGroup:
//     AskCompileTime + Upcast<dyn InterpreterQueryGroup> + Upcast<dyn InstructionGenQueryGroup>
// {
//     fn visualizer(&self, ty: EntityRoutePtr) -> Arc<Visualizer>;
//     fn visual_ty(&self, ty: EntityRoutePtr) -> VisualTy;
// }
