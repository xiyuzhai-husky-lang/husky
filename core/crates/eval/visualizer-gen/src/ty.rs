use semantics_eager::FuncStmtVariant;
use semantics_lazy::{LazyStmtVariant, XmlExprVariant};

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualTy {
    Void,
    Bool,
    B32,
    B64,
    I32,
    F32,
    Point2d,
    Shape2d,
    Region2d,
    Image2d,
    Graphics2d,
}

impl VisualTy {
    pub(crate) fn from_static(
        db: &dyn VisualizerQueryGroup,
        ty: EntityRoutePtr,
        static_visual_ty: StaticVisualTy,
    ) -> Self {
        match static_visual_ty {
            StaticVisualTy::Void => VisualTy::Void,
            StaticVisualTy::Bool => VisualTy::Bool,
            StaticVisualTy::B32 => VisualTy::B32,
            StaticVisualTy::B64 => VisualTy::B64,
            StaticVisualTy::I32 => VisualTy::I32,
            StaticVisualTy::F32 => VisualTy::F32,
            StaticVisualTy::Point2d => VisualTy::Point2d,
            StaticVisualTy::Shape2d => VisualTy::Shape2d,
            StaticVisualTy::Region2d => VisualTy::Region2d,
            StaticVisualTy::Image2d => VisualTy::Image2d,
            StaticVisualTy::Graphics2d => VisualTy::Graphics2d,
            StaticVisualTy::Group => match db
                .visualizer(ty.spatial_arguments[0].take_entity_route())
                .ty
            {
                VisualTy::Void => todo!(),
                VisualTy::Bool => todo!(),
                VisualTy::B32 => todo!(),
                VisualTy::B64 => todo!(),
                VisualTy::I32 => todo!(),
                VisualTy::F32 => todo!(),
                VisualTy::Point2d | VisualTy::Shape2d => VisualTy::Shape2d,
                VisualTy::Region2d => VisualTy::Region2d,
                VisualTy::Image2d => VisualTy::Image2d,
                VisualTy::Graphics2d => todo!(),
            },
            StaticVisualTy::Dataset => todo!(),
        }
    }

    pub(crate) fn from_stmts(db: &dyn VisualizerQueryGroup, stmts: &[Arc<LazyStmt>]) -> VisualTy {
        match stmts.last().unwrap().variant {
            LazyStmtVariant::Return { ref result } => todo!(),
            LazyStmtVariant::ReturnXml { ref xml_expr } => match xml_expr.variant {
                XmlExprVariant::Value(ref expr) => db.visual_ty(expr.ty()),
                XmlExprVariant::Tag { tag_kind, .. } => match tag_kind {
                    XmlTagKind::Point2d => VisualTy::Point2d,
                    XmlTagKind::Contour | XmlTagKind::Arrow2d | XmlTagKind::LineSegment => {
                        VisualTy::Shape2d
                    }
                },
            },
            LazyStmtVariant::ConditionFlow { ref branches } => todo!(),
            LazyStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
            _ => panic!(),
        }
    }
}
