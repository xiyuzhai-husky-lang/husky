#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VdBigIntData {
    inner: num_bigint::BigInt,
}

impl VdBigIntData {
    pub fn new(inner: num_bigint::BigInt) -> Self {
        debug_assert!(
            is_outside_i128_range(&inner),
            "BigInt value must be outside i128 range"
        );
        Self { inner }
    }

    pub fn try_new(inner: num_bigint::BigInt) -> Result<Self, ()> {
        if is_outside_i128_range(&inner) {
            Ok(Self { inner })
        } else {
            Err(())
        }
    }
}

fn is_outside_i128_range(value: &num_bigint::BigInt) -> bool {
    value > &num_bigint::BigInt::from(i128::MAX) || value < &num_bigint::BigInt::from(i128::MIN)
}
