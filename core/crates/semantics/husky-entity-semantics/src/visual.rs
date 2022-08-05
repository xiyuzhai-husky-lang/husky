use husky_lazy_semantics::XmlExprVariant;
use husky_static_visualizer::{StaticVisualizer, StaticVisualizerFp};
use husky_trace_protocol::VisualData;
use husky_xml_syntax::XmlTagKind;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VisualTy {
    Any,
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
    AnyGroup,
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

impl Visualizer {
    pub fn void() -> Arc<Self> {
        Arc::new(Self {
            visual_ty: VisualTy::Void,
            variant: VisualizerVariant::Void,
        })
    }
    pub fn generic() -> Arc<Self> {
        Arc::new(Self {
            visual_ty: VisualTy::Any,
            variant: VisualizerVariant::Any,
        })
    }

    pub fn from_static(
        db: &dyn EntityDefnQueryGroup,
        ty: EntityRoutePtr,
        static_visualizer: StaticVisualizer,
    ) -> Arc<Self> {
        Arc::new(Self {
            visual_ty: VisualTy::from_static(db, ty, static_visualizer.visual_ty),
            variant: VisualizerVariant::Static {
                fp: static_visualizer.fp,
            },
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VisualizerVariant {
    Group { element_ty: EntityRoutePtr },
    Custom { stmts: Avec<LazyStmt> },
    Static { fp: StaticVisualizerFp },
    Void,
    Any,
}

pub(crate) fn visualizer(db: &dyn EntityDefnQueryGroup, ty: EntityRoutePtr) -> Arc<Visualizer> {
    let ty_defn = db.entity_defn(ty).unwrap();
    if ty.spatial_arguments.len() == 0 {
        match ty_defn.variant {
            EntityDefnVariant::Ty { ref visualizer, .. } => visualizer.clone(),
            EntityDefnVariant::Any => Visualizer::generic(),
            _ => todo!(),
        }
    } else {
        match ty_defn.variant {
            EntityDefnVariant::Ty { ref visualizer, .. } => match visualizer.visual_ty {
                VisualTy::AnyGroup => {
                    let element_ty = ty.spatial_arguments[0].take_entity_route();
                    let visual_ty = match db.visualizer(element_ty).visual_ty {
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
                        VisualTy::Any => VisualTy::AnyGroup,
                        VisualTy::AnyGroup => todo!(),
                    };
                    Arc::new(Visualizer {
                        visual_ty,
                        variant: VisualizerVariant::Group { element_ty },
                    })
                }
                _ => panic!(),
            },
            EntityDefnVariant::Any => Visualizer::generic(),
            _ => todo!(),
        }
    }

    //     Visualizer {
    //         visual_ty: match opt_static_visual_ty {
    //             Some(static_visual_ty) => VisualTy::from_static(db, ty, static_visual_ty),
    //             None => {
    //                 if let Some(ref stmts) = opt_visual_stmts {
    //                     VisualTy::from_stmts(db, stmts)
    //                 } else {
    //                     panic!("No visual source for ty `{ty:?}`")
    //                 }
    //             }
    //         },
    //         variant: match ty_kind {
    //             TyKind::Enum => todo!(),
    //             TyKind::Record => todo!(),
    //             TyKind::Struct => VisualizerVariant::Custom {
    //                 opt_stmts: opt_visual_stmts.clone(),
    //             },
    //             TyKind::Primitive => todo!(),
    //             TyKind::Vec | TyKind::Slice | TyKind::CyclicSlice | TyKind::Array => {
    //                 VisualizerVariant::Group {
    //                     element_ty: ty.spatial_arguments[0].take_entity_route(),
    //                 }
    //             }
    //             TyKind::Tuple => todo!(),
    //             TyKind::Mor => todo!(),
    //             TyKind::Fp => todo!(),
    //             TyKind::AssociatedAny => todo!(),
    //             TyKind::ThisAny => todo!(),
    //             TyKind::SpatialPlaceholderAny => todo!(),
    //             TyKind::BoxAny => todo!(),
    //             TyKind::HigherKind => todo!(),
    //             TyKind::Ref => todo!(),
    //             TyKind::Option => todo!(),
    //         },
    //     },
    //     _ => panic!(),
    // })
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
                VisualTy::Any => VisualTy::AnyGroup,
                VisualTy::AnyGroup => todo!(),
            },
            StaticVisualTy::Dataset => todo!(),
        }
    }
}
