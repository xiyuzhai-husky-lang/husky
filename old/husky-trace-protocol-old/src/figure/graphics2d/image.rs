mod dimension;
mod layer;
mod original;

pub use dimension::*;
pub use layer::*;
pub use original::*;

use super::*;
use web_sys::ImageData;

pub trait ContainsImageLayers<'a> {
    fn image_layers(&self) -> Vec<&'a ImageLayerData>;
}

impl<'a, T> ContainsImageLayers<'a> for std::rc::Rc<T>
where
    T: ContainsImageLayers<'a>,
{
    fn image_layers(&self) -> Vec<&'a ImageLayerData> {
        <T as ContainsImageLayers<'a>>::image_layers(self)
    }
}

impl<'a, T> ContainsImageLayers<'a> for std::sync::Arc<T>
where
    T: ContainsImageLayers<'a>,
{
    fn image_layers(&self) -> Vec<&'a ImageLayerData> {
        <T as ContainsImageLayers>::image_layers(self)
    }
}

impl<'a, T, S> ContainsImageLayers<'a> for (T, S)
where
    T: ContainsImageLayers<'a>,
    S: ContainsImageLayers<'a>,
{
    fn image_layers(&self) -> Vec<&'a ImageLayerData> {
        let mut image_layers = self.0.image_layers();
        image_layers.extend(self.1.image_layers().into_iter());
        image_layers
    }
}

impl<'a, T> ContainsImageLayers<'a> for [T]
where
    T: ContainsImageLayers<'a>,
{
    fn image_layers(&self) -> Vec<&'a ImageLayerData> {
        let mut image_layers = vec![];

        for item in self.iter() {
            image_layers.extend(item.image_layers().into_iter());
        }
        image_layers
    }
}

impl<'a, T> ContainsImageLayers<'a> for Vec<T>
where
    T: ContainsImageLayers<'a>,
{
    fn image_layers(&self) -> Vec<&'a ImageLayerData> {
        let mut image_layers = vec![];

        for item in self.iter() {
            image_layers.extend(item.image_layers().into_iter());
        }
        image_layers
    }
}
