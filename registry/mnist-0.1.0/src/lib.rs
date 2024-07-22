pub mod dataset;
pub mod input_id;
pub mod task;

use self::input_id::*;
use dataset::MNIST_DATASET;
use husky_core::*;
use husky_linket_impl::standard::ugly::*;
use husky_standard_devsoul_interface::{label::IsLabel, static_var::StandardStaticVarId, ugly::*};

use husky_devsoul_interface::ugly::*;

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

    pub fn pixel(&self, i: usize, j: usize) -> bool {
        let row = self.0[i + 1];
        (row & (1 << (29 - j))) != 0
    }
}

impl husky_core::visual::Visualize for BinaryImage28 {
    fn visualize(&self, sct: &mut __VisualSynchrotron) -> __Visual {
        __Visual::new_binary_image(
            4,
            28,
            28,
            self.0[2..]
                .iter()
                .map(|u| {
                    unsafe { std::mem::transmute::<_, [u8; 4]>(u << 2) }
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

thread_local! {
    static __INPUT: std::cell::Cell<Option<Leash<BinaryImage28>>> = Default::default();
}

#[allow(non_snake_case)]
pub fn INPUT() -> Leash<BinaryImage28> {
    __INPUT.get().unwrap()
}

pub struct INPUT {}

impl INPUT {
    pub fn set_up_for_testing(index: usize) {
        // todo: check range!
        set_input_id(MnistInputId::from_index(index))
    }

    pub fn get_id() -> StandardStaticVarId {
        input_id().index().into()
    }

    pub fn set_id(id: StandardStaticVarId) {
        todo!()
    }
}

// ad hoc
#[allow(non_snake_case)]
pub fn TASK() {}

pub struct TASK {}

impl TASK {
    pub fn set_up_for_testing(index: usize) {
        todo!()
    }

    pub fn get_id() -> StandardStaticVarId {
        todo!()
    }

    pub fn set_id(id: StandardStaticVarId) {
        todo!()
    }
}
