use crate::*;
use husky_ml_task_interface::InputId;

lazy_static::lazy_static! {
    pub(crate) static ref MNIST_DATASET: MnistDataset = MnistDataset::new(35016232u64);
}

pub struct MnistDataset {
    images: Vec<BinaryImage28>,
    labels: Vec<MnistLabel>,
    permutation: Vec<u32>,
}

impl MnistDataset {
    fn new(seed: u64) -> Self {
        let (images, labels) = load_mnist_images_and_labels();
        let permutation = husky_xrng_utils::generate_random_permutation(60000, seed);
        Self {
            images,
            labels,
            permutation,
        }
    }

    pub(crate) fn input(&'static self, sample_id: InputId) -> &'static BinaryImage28 {
        &self.images[self.index(sample_id)]
    }

    pub(crate) fn label(&'static self, sample_id: InputId) -> MnistLabel {
        self.labels[self.index(sample_id)]
    }

    fn index(&'static self, sample_id: InputId) -> usize {
        self.permutation[sample_id.index()] as usize
    }
}

fn load_mnist_images_and_labels() -> (Vec<BinaryImage28>, Vec<MnistLabel>) {
    let mut images: Vec<BinaryImage28> = vec![];
    let mut labels: Vec<MnistLabel> = vec![];
    let mut dir: &std::path::Path = &std::env::current_dir().unwrap();
    while dir.file_name().unwrap().to_str().unwrap() != "husky" {
        dir = dir.parent().unwrap();
    }
    let file_content: Vec<u8> = match std::fs::read(dir.join("data/mnist-binary-images")) {
        Ok(file_content) => file_content,
        Err(e) => {
            println!(
                "current dir = {:?}, dir = {dir:?}",
                (std::env::current_dir())
            );
            todo!()
        }
    };
    assert_eq!(file_content.len(), 60000 * (1 + 28 * 4));
    for sample_idx in 0..60000 {
        let base = sample_idx * (1 + 28 * 4);
        labels.push(file_content[base].into());
        images.push(BinaryImage28::read(
            &file_content[(base + 1)..(base + 1 + 28 * 4)],
        ));
    }
    assert_eq!(labels[0], 5.into());
    assert_eq!(labels.len(), 60000);
    assert_eq!(images.len(), 60000);
    (images, labels)
}

impl BinaryImage28 {
    fn read(content: &[u8]) -> Self {
        assert_eq!(content.len(), 28 * 4);
        let mut padded_rows = [0; 30];
        for i in 0..28 {
            let mut row = 0u32;
            for k in 0..4 {
                row |= (content[i * 4 + k] as u32) << (3 - k) * 8;
            }
            padded_rows[i + 1] = row;
        }
        Self(padded_rows)
    }
}
