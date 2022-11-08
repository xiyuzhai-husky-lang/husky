use ref_arena::{RefArena, RefArenaIdx};

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FigureCanvasAtom {
    Primitive(PrimitiveValueData),
    Graphics2d(Graphics2dCanvasData),
}

pub type FigureCanvasAtomIdx = RefArenaIdx<FigureCanvasAtom, 100>;
pub type FigureCanvasAtomArena = RefArena<FigureCanvasAtom, 100>;

impl FigureCanvasAtom {
    pub fn new(visual_data: VisualData) -> Option<Self> {
        Some(match visual_data {
            VisualData::BinaryImage28 { padded_rows } => {
                FigureCanvasAtom::Graphics2d(Graphics2dCanvasData {
                    image_layers: vec![ImageLayerData::binary_image28(&padded_rows)],
                    shapes: Vec::new(),
                    xrange: (0.0, 28.0),
                    yrange: (0.0, 28.0),
                })
            }
            VisualData::Primitive { value } => FigureCanvasAtom::Primitive(value),
            VisualData::BinaryGrid28 { ref padded_rows } => {
                FigureCanvasAtom::Graphics2d(Graphics2dCanvasData {
                    image_layers: vec![],
                    shapes: vec![Shape2dData::laser_grid28(padded_rows)],
                    xrange: (0.0, 28.0),
                    yrange: (0.0, 28.0),
                })
            }
            VisualData::Contour { points } => FigureCanvasAtom::Graphics2d(Graphics2dCanvasData {
                image_layers: vec![],
                shapes: vec![Shape2dData::Contour { points }],
                xrange: (0.0, 28.0),
                yrange: (0.0, 28.0),
            }),
            VisualData::Group(mut visuals) => {
                if visuals.len() == 0 {
                    return Default::default();
                }
                if visuals.len() == 1 {
                    return Self::new(visuals.pop().unwrap());
                }
                match visuals[0].world() {
                    VisualWorld::Primitive => Self::new_specific_primitive_group(visuals)?,
                    VisualWorld::Graphics2d => Self::new_graphics2d_group(visuals),
                    VisualWorld::Graphics3d => todo!(),
                }
            }
            VisualData::LineSegment { start, end } => {
                FigureCanvasAtom::Graphics2d(Graphics2dCanvasData {
                    image_layers: vec![],
                    shapes: vec![Shape2dData::LineSegment { start, end }],
                    xrange: (0.0, 28.0),
                    yrange: (0.0, 28.0),
                })
            }
        })
    }

    pub fn new_graphics2d_group(visuals: Vec<VisualData>) -> Self {
        let mut image_layers = Vec::new();
        let mut shapes = Vec::new();
        for visual_data in visuals {
            match visual_data {
                VisualData::BinaryImage28 { ref padded_rows } => {
                    image_layers.push(ImageLayerData::binary_image28(padded_rows))
                }
                VisualData::BinaryGrid28 { .. }
                | VisualData::Contour { .. }
                | VisualData::Group(_)
                | VisualData::LineSegment { .. } => shapes.push(visual_data.into()),
                VisualData::Primitive { .. } => panic!(),
            }
        }
        FigureCanvasAtom::Graphics2d(Graphics2dCanvasData {
            image_layers,
            shapes,
            xrange: (0.0, 28.0),
            yrange: (0.0, 28.0),
        })
    }

    pub fn new_specific_primitive_group(_visuals: Vec<VisualData>) -> Option<Self> {
        None
    }
}

impl<'a> ContainsImageLayers<'a> for &'a FigureCanvasAtom {
    fn image_layers(&self) -> Vec<&'a ImageLayerData> {
        match self {
            FigureCanvasAtom::Primitive(_) => todo!(),
            FigureCanvasAtom::Graphics2d(data) => data.image_layers(),
        }
    }
}

impl<'a> ContainsShapes<'a> for &'a FigureCanvasAtom {
    fn shapes(&self) -> Vec<&'a Shape2dData> {
        match self {
            FigureCanvasAtom::Primitive(_) => todo!(),
            FigureCanvasAtom::Graphics2d(data) => data.shapes(),
        }
    }
}
