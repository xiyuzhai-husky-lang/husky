use husky_lazy_semantics::XmlExprVariant;
use husky_xml_syntax::XmlTagKind;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VisualTy {
    Void,
    Bool,
    B32,
    B64,
    Integer,
    Float,
    Point2d,
    Shape2d,
    Region2d,
    Image2d,
    Graphics2d,
    Plot2d,
    Dataset,
}

impl Default for VisualTy {
    fn default() -> Self {
        VisualTy::Void
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Visualizer {
    pub visual_ty: VisualTy,
    pub variant: VisualizerVariant,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VisualizerVariant {
    Vec,
    CyclicSlice,
    Custom { opt_stmts: Option<Avec<LazyStmt>> },
}

pub(crate) fn visualizer(db: &dyn EntityDefnQueryGroup, ty: EntityRoutePtr) -> Arc<Visualizer> {
    let ty_defn = db.entity_defn(ty).unwrap();
    Arc::new(match ty_defn.variant {
        EntityDefnVariant::Ty {
            opt_static_visual_ty,
            ref opt_visual_stmts,
            ty_kind,
            ..
        } => Visualizer {
            visual_ty: match opt_static_visual_ty {
                Some(static_visual_ty) => VisualTy::from_static(db, ty, static_visual_ty),
                None => {
                    if let Some(ref stmts) = opt_visual_stmts {
                        VisualTy::from_stmts(db, stmts)
                    } else {
                        panic!("No visual source for ty `{ty:?}`")
                    }
                }
            },
            variant: match ty_kind {
                TyKind::Enum => todo!(),
                TyKind::Record => todo!(),
                TyKind::Struct => VisualizerVariant::Custom {
                    opt_stmts: opt_visual_stmts.clone(),
                },
                TyKind::Primitive => todo!(),
                TyKind::Vec => VisualizerVariant::Vec,
                TyKind::Slice => todo!(),
                TyKind::CyclicSlice => VisualizerVariant::CyclicSlice,
                TyKind::Array => todo!(),
                TyKind::Tuple => todo!(),
                TyKind::Mor => todo!(),
                TyKind::Fp => todo!(),
                TyKind::AssociatedAny => todo!(),
                TyKind::ThisAny => todo!(),
                TyKind::SpatialPlaceholderAny => todo!(),
                TyKind::BoxAny => todo!(),
                TyKind::HigherKind => todo!(),
                TyKind::Ref => todo!(),
                TyKind::Option => todo!(),
            },
        },
        _ => panic!(),
    })
}

pub(crate) fn visual_ty(db: &dyn EntityDefnQueryGroup, ty: EntityRoutePtr) -> VisualTy {
    db.visualizer(ty).visual_ty
}

impl VisualTy {
    pub(crate) fn from_stmts(db: &dyn EntityDefnQueryGroup, stmts: &[Arc<LazyStmt>]) -> VisualTy {
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
            LazyStmtVariant::ConditionFlow { .. } => todo!(),
            LazyStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
            _ => panic!(),
        }
    }

    pub(crate) fn from_static(
        db: &dyn EntityDefnQueryGroup,
        ty: EntityRoutePtr,
        static_visual_ty: StaticVisualTy,
    ) -> Self {
        match static_visual_ty {
            StaticVisualTy::Void => VisualTy::Void,
            StaticVisualTy::Bool => VisualTy::Bool,
            StaticVisualTy::B32 => VisualTy::B32,
            StaticVisualTy::B64 => VisualTy::B64,
            StaticVisualTy::Integer => VisualTy::Integer,
            StaticVisualTy::Float => VisualTy::Float,
            StaticVisualTy::Point2d => VisualTy::Point2d,
            StaticVisualTy::Shape2d => VisualTy::Shape2d,
            StaticVisualTy::Region2d => VisualTy::Region2d,
            StaticVisualTy::Image2d => VisualTy::Image2d,
            StaticVisualTy::Graphics2d => VisualTy::Graphics2d,
            StaticVisualTy::Group => match db
                .visualizer(ty.spatial_arguments[0].take_entity_route())
                .visual_ty
            {
                VisualTy::Void => todo!(),
                VisualTy::Bool => todo!(),
                VisualTy::B32 => todo!(),
                VisualTy::B64 => todo!(),
                VisualTy::Integer => VisualTy::Plot2d,
                VisualTy::Float => todo!(),
                VisualTy::Point2d | VisualTy::Shape2d => VisualTy::Shape2d,
                VisualTy::Region2d => VisualTy::Region2d,
                VisualTy::Image2d => VisualTy::Image2d,
                VisualTy::Graphics2d => todo!(),
                VisualTy::Dataset => todo!(),
                VisualTy::Plot2d => todo!(),
            },
            StaticVisualTy::Dataset => todo!(),
        }
    }
}
