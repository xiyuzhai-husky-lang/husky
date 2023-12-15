pub trait IsTaskValue:
    Sized
    + std::ops::Add<Self, Output = Self>
    + std::ops::AddAssign<Self>
    + std::ops::BitOr<Self, Output = Self>
    + std::ops::BitOrAssign<Self>
    + std::ops::BitXor<Self, Output = Self>
    + std::ops::BitXorAssign<Self>
    + std::ops::Div<Self, Output = Self>
    + std::ops::Mul<Self, Output = Self>
    + std::ops::MulAssign<Self>
    + std::ops::Neg<Output = Self>
    + std::ops::Not<Output = Self>
    + std::ops::Shl<Self, Output = Self>
    + std::ops::ShlAssign<Self>
    + std::ops::Shr<Self, Output = Self>
    + std::ops::ShrAssign<Self>
    + std::ops::Sub<Self, Output = Self>
    + std::ops::SubAssign<Self>
{
}
