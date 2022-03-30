use super::*;
use print_utils::*;

pub(super) fn load() -> (Arc<Vec<Arc<BinaryImage28>>>, Arc<Vec<u8>>) {
    let mut images: Vec<Arc<BinaryImage28>> = Vec::new();
    let mut labels: Vec<u8> = Vec::new();
    let file_content: Vec<u8> =
        std::fs::read("/home/xiyuzhai/Documents/data/mnist_bool_images").unwrap();
    assert_eq!(file_content.len(), 60000 * (1 + 28 * 4));
    for input_idx in 0..60000 {
        let base = input_idx * (1 + 28 * 4);
        labels.push(file_content[base]);
        images.push(Arc::new(BinaryImage28::read(
            &file_content[(base + 1)..(base + 1 + 28 * 4)],
        )));
    }
    assert_eq!(labels[0], 5);
    assert_eq!(labels.len(), 60000);
    assert_eq!(images.len(), 60000);
    (Arc::new(images), Arc::new(labels))
}

use std::sync::Arc;
