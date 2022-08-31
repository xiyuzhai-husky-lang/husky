mod dimension;
mod layer;
mod original;

pub use dimension::*;
pub use layer::*;
pub use original::*;

use super::*;
use web_sys::ImageData;

pub trait ContainsImageLayers {
    fn image_layers(&self) -> &[ImageLayerData];

    fn total_image_layers<'a, P>(&'a self, pins: &[&'a P]) -> Vec<&'a ImageLayerData>
    where
        P: ContainsImageLayers,
    {
        let mut total_image_layers: Vec<&'a ImageLayerData> = vec![];
        total_image_layers.extend(self.image_layers().iter());
        for pinned_canvas_value in pins {
            total_image_layers.extend(pinned_canvas_value.image_layers().iter())
        }
        total_image_layers
    }
}
