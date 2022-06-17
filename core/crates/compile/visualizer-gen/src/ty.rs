use semantics_eager::FuncStmtVariant;
use semantics_lazy::LazyStmtVariant;

use crate::*;

#[derive(Debug, Clone, Copy)]
pub enum VisualTy {
    Void,
    B32,
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
            StaticVisualTy::B32 => VisualTy::B32,
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
                VisualTy::B32 => todo!(),
                VisualTy::I32 => todo!(),
                VisualTy::F32 => todo!(),
                VisualTy::Point2d | VisualTy::Shape2d => VisualTy::Shape2d,
                VisualTy::Region2d => todo!(),
                VisualTy::Image2d => todo!(),
                VisualTy::Graphics2d => todo!(),
            },
        }
    }

    pub(crate) fn from_stmts(stmts: &[Arc<LazyStmt>]) -> VisualTy {
        match stmts.last().unwrap().variant {
            LazyStmtVariant::Return { ref result } => todo!(),
            LazyStmtVariant::ReturnXml { ref xml_expr } => match xml_expr.variant {
                semantics_lazy::XmlExprVariant::Value(_) => todo!(),
                semantics_lazy::XmlExprVariant::Tag { tag_kind, .. } => match tag_kind {
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
