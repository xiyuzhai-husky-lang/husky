mod dataset;

use dataset::MNIST_DATASET;
use husky_core::*;
use husky_linkage_impl::standard::ugly::*;
use husky_ml_task_interface::{input_id, label::IsLabel, ugly::*, InputId};
use husky_standard_value::ugly::*;
use husky_task_interface::{init_crate, ugly::*};

init_crate!();

#[husky_standard_value::value_conversion]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MnistLabel {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl IsLabel for MnistLabel {
    fn label() -> Self {
        MNIST_DATASET.label(input_id())
    }

    fn label_at_input(input_id: InputId) -> Self {
        MNIST_DATASET.label(input_id)
    }
}

impl From<u8> for MnistLabel {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[husky_standard_value::value_conversion]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BinaryImage28([u32; 30]);

impl BinaryImage28 {
    pub fn new_zeros() -> Self {
        Self::default()
    }
}

impl husky_core::visual::Visualize for BinaryImage28 {
    fn visualize(&self, sct: &mut __VisualSynchrotron) -> __Visual {
        __Visual::new_binary_image(
            4,
            28,
            28,
            self.0
                .iter()
                .map(|u| {
                    unsafe { std::mem::transmute_copy::<_, [u8; 4]>(u) }
                        .into_iter()
                        .rev()
                })
                .flatten()
                .collect(),
            sct,
        )
    }
}

impl std::ops::Index<usize> for BinaryImage28 {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for BinaryImage28 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[husky_standard_value::value_conversion]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BinaryGrid28([u32; 31]);

impl BinaryGrid28 {
    pub fn new_zeros() -> Self {
        Self::default()
    }
}

impl std::ops::Index<usize> for BinaryGrid28 {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for BinaryGrid28 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl BinaryGrid28 {}

pub fn input() -> &'static BinaryImage28 {
    MNIST_DATASET.input(input_id())
}
