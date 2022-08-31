mod image;
mod shape;

use super::*;

pub use image::*;
pub use shape::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Graphics2dCanvasData {
    pub(crate) image_layers: Vec<ImageLayerData>,
    pub(crate) shapes: Vec<Shape2dData>,
    pub xrange: (f32, f32),
    pub yrange: (f32, f32),
}

impl Graphics2dCanvasData {
    pub fn from_visual_data(visual_data: VisualData) -> Graphics2dCanvasData {
        match visual_data {
            VisualData::BinaryImage28 { padded_rows } => Graphics2dCanvasData {
                image_layers: vec![ImageLayerData::binary_image28(&padded_rows)],
                shapes: Vec::new(),
                xrange: (0.0, 28.0),
                yrange: (0.0, 28.0),
            },
            VisualData::BinaryGrid28 { ref padded_rows } => Graphics2dCanvasData {
                image_layers: vec![],
                shapes: vec![Shape2dData::laser_grid28(padded_rows)],
                xrange: (0.0, 28.0),
                yrange: (0.0, 28.0),
            },
            VisualData::Contour { points } => Graphics2dCanvasData {
                image_layers: vec![],
                shapes: vec![Shape2dData::Contour { points }],
                xrange: (0.0, 28.0),
                yrange: (0.0, 28.0),
            },
            VisualData::Group(visual_data_group) => {
                let mut image_layers = Vec::new();
                let mut shapes = Vec::new();
                for visual_data in visual_data_group {
                    match visual_data {
                        VisualData::BinaryImage28 { ref padded_rows } => {
                            image_layers.push(ImageLayerData::binary_image28(padded_rows))
                        }
                        VisualData::Primitive { .. } => todo!(),
                        VisualData::BinaryGrid28 { .. }
                        | VisualData::Contour { .. }
                        | VisualData::Group(_)
                        | VisualData::LineSegment { .. } => shapes.push(visual_data.into()),
                    }
                }
                Graphics2dCanvasData {
                    image_layers,
                    shapes,
                    xrange: (0.0, 28.0),
                    yrange: (0.0, 28.0),
                }
            }
            VisualData::LineSegment { start, end } => Graphics2dCanvasData {
                image_layers: vec![],
                shapes: vec![Shape2dData::LineSegment { start, end }],
                xrange: (0.0, 28.0),
                yrange: (0.0, 28.0),
            },
            _ => {
                panic!()
            }
        }
    }

    pub fn image_layers<'a>(
        &'a self,
        pinned_canvas_values: &[&'a FigureCanvasData],
    ) -> Vec<&'a ImageLayerData> {
        let mut image_layers = self.image_layers.iter().collect::<Vec<&_>>();
        for pinned_canvas_value in pinned_canvas_values {
            image_layers.extend(match pinned_canvas_value {
                FigureCanvasData::Graphics2d { graphics2d_data } => {
                    graphics2d_data.image_layers.iter()
                }
                FigureCanvasData::Mutations { mutations } => todo!(),
                FigureCanvasData::GenericGraphics2d {
                    partitioned_samples,
                } => todo!(),
                _ => continue,
            })
        }
        image_layers
    }

    pub fn shapes<'a>(
        &'a self,
        pinned_canvas_values: &[&'a FigureCanvasData],
    ) -> Vec<&'a Shape2dData> {
        let mut shapes = self.shapes.iter().collect::<Vec<&_>>();
        for pinned_canvas_value in pinned_canvas_values {
            shapes.extend(match pinned_canvas_value {
                FigureCanvasData::Graphics2d { graphics2d_data } => graphics2d_data.shapes.iter(),
                FigureCanvasData::Mutations { mutations } => todo!(),
                FigureCanvasData::GenericGraphics2d {
                    partitioned_samples,
                } => todo!(),
                _ => continue,
            })
        }
        shapes
    }
}
