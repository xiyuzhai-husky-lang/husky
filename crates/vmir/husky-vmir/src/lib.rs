#![feature(impl_trait_in_assoc_type)]
mod builder;
mod coersion;
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
use husky_task::linktime::IsLinktime;
use husky_task_interface::IsLinkageImpl;

pub(crate) trait ToVmir<LinkageImpl: IsLinkageImpl>: Copy {
    type Output;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinkageImpl = LinkageImpl>;
}

impl<T, LinkageImpl: IsLinkageImpl> ToVmir<LinkageImpl> for Option<T>
where
    T: ToVmir<LinkageImpl>,
{
    type Output = Option<T::Output>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinkageImpl = LinkageImpl>,
    {
        self.map(|slf| slf.to_vmir(builder))
    }
}

impl<'a, T, LinkageImpl: IsLinkageImpl> ToVmir<LinkageImpl> for &'a [T]
where
    &'a T: ToVmir<LinkageImpl>,
{
    type Output = Vec<<&'a T as ToVmir<LinkageImpl>>::Output>;

    fn to_vmir<Linktime>(self, builder: &mut VmirBuilder<Linktime>) -> Self::Output
    where
        Linktime: IsLinktime<LinkageImpl = LinkageImpl>,
    {
        self.iter().map(|elem| elem.to_vmir(builder)).collect()
    }
}
