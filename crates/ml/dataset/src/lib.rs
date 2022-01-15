mod iter;
mod loader;
pub mod synthetic;

use std::any::Any;

pub use iter::SampleIter;
pub use loader::SampleLoader;

pub trait Dataset {
    fn dev_loader(&self) -> SampleLoader;
    fn val_iter(&self) -> SampleIter;
    fn test_iter(&self) -> SampleIter;
}

pub enum Input<'a> {
    Owned(Box<dyn Any>),
    Borrowed(&'a dyn Any),
}
