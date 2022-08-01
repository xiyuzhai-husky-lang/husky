mod binary_grid28;
mod binary_image28;
mod load;

pub use binary_grid28::*;
pub use binary_image28::*;

use crate::*;
use load::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MnistDataset {
    images: Arc<Vec<Arc<BinaryImage28>>>,
    labels: Arc<Vec<Label>>,
    permutation: Arc<Vec<u32>>,
}

impl MnistDataset {
    pub fn new(seed: u64) -> Self {
        let (images, labels) = load_mnist();
        Self {
            images,
            labels,
            permutation: Arc::new(permutation_from_seed(60000, seed)),
        }
    }
}

impl Serialize for MnistDataset {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("MnistDataset")
    }
}

impl __StaticInfo for MnistDataset {
    type __StaticSelf = Self;

    fn __static_typename() -> Cow<'static, str> {
        todo!()
    }
}

impl<'eval> __Registrable<'eval> for MnistDataset {
    unsafe fn __to_register(self) -> __Register<'eval> {
        todo!()
    }
}

impl<'eval> DatasetDyn<'eval> for MnistDataset {
    fn dev_loader(&self) -> DataLoader<'eval> {
        Box::new(MnistDevLoader::new(
            self.images.clone(),
            self.labels.clone(),
            self.permutation.clone(),
        ))
    }

    fn val_loader(&self) -> DataLoader<'eval> {
        Box::new(MnistValLoader::new(self.permutation.clone()))
    }

    fn test_loader(&self) -> DataLoader<'eval> {
        Box::new(MnistTestLoader::new(self.permutation.clone()))
    }

    fn profile_iter(&self) -> DataIter<'eval> {
        todo!()
    }
}
