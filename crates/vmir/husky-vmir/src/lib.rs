#![feature(impl_trait_in_assoc_type)]
mod builder;
mod coercion;
pub mod destroyer;
pub mod eval;
pub mod expr;
pub mod jar;
pub mod pattern;
pub mod region;
pub mod stmt;
pub mod storage;
#[cfg(test)]
mod tests;
pub mod version_stamp;

use self::builder::VmirBuilder;
use self::jar::VmirJar as Jar;
#[cfg(test)]
use self::tests::*;
use husky_linket_impl::linket_impl::IsLinketImpl;
use husky_linktime::IsLinktime;
use husky_value::IsValue;

pub(crate) trait ToVmir<LinketImpl: IsLinketImpl>: Copy {
    type Output;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinketImpl = LinketImpl>;
}

impl<T, LinketImpl: IsLinketImpl> ToVmir<LinketImpl> for Option<T>
where
    T: ToVmir<LinketImpl>,
{
    type Output = Option<T::Output>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinketImpl = LinketImpl>,
    {
        self.map(|slf| slf.to_vmir(builder))
    }
}

impl<'a, T, LinketImpl: IsLinketImpl> ToVmir<LinketImpl> for &'a [T]
where
    &'a T: ToVmir<LinketImpl>,
{
    type Output = Vec<<&'a T as ToVmir<LinketImpl>>::Output>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinketImpl = LinketImpl>,
    {
        self.iter().map(|elem| elem.to_vmir(builder)).collect()
    }
}
