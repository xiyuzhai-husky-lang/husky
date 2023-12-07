use husky_core::*;

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

pub struct BinaryImage28([u32; 30]);

pub struct BinaryGrid28([u32; 31]);

impl BinaryImage28 {
    
}

impl IntIndex for BinaryImage28 {
    
    type Output = u32;
}

impl BinaryGrid28 {
    
}

impl IntIndex for BinaryGrid28 {
    
    type Output = u32;
}