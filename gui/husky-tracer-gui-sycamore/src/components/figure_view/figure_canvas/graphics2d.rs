mod image;
mod shape;

use super::*;
use image::*;
use shape::*;

#[derive(Prop)]
pub struct Graphics2dCanvasProps {
    image_layers: Vec<ImageLayerData>,
    shapes: Vec<Shape2dData>,
    xrange: (f32, f32),
    yrange: (f32, f32),
}
