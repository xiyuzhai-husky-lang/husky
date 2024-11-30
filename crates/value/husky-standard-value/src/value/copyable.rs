use super::*;

#[derive(Debug)]
pub struct CopyableConverter(fn(&u128) -> &dyn ImmortalDyn);

impl CopyableConverter {
    pub fn new<T>(t: T) -> (Self, u128)
    where
        T: Copy + Immortal + Into<u128> + Eq,
        for<'a> &'a T: From<&'a u128>,
    {
        let v: u128 = t.into();
        #[cfg(debug_assertions)]
        {
            let s: &T = (&v).into();
            assert_eq!(*s, t);
        }
        (
            Self(|v| {
                let t: &T = v.into();
                t
            }),
            v,
        )
    }
}
