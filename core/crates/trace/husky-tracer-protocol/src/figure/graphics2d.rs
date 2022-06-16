mod image;
mod shape;

use super::*;

pub use image::*;
pub use shape::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Graphics2dCanvasData {
    pub image_layers: Vec<ImageLayerData>,
    pub shapes: Vec<Shape2dData>,
    pub xrange: (f32, f32),
    pub yrange: (f32, f32),
}
