use crate::*;
use husky_ethereal_term::{EtherealTerm, EtherealTermApplication, EtherealTermDb};
use salsa::DbWithJar;

pub trait LayoutDb: DbWithJar<LayoutJar> + EtherealTermDb {
    fn reg_memory_kind(&self, ty: EtherealTerm) -> RegMemoryKind;
}

impl<T> LayoutDb for T
where
    T: DbWithJar<LayoutJar> + EtherealTermDb,
{
    fn reg_memory_kind(&self, _ty: EtherealTerm) -> RegMemoryKind {
        todo!()
    }
}

pub(crate) fn reg_memory_kind(_db: &dyn LayoutDb, _ty: EtherealTerm) -> RegMemoryKind {
    todo!()
    // let ty = ty.intrinsic();
    // if ty.is_primitive() {
    //     RegMemoryKind::Direct
    // } else {
    //     if db.is_copyable(ty).unwrap() {
    //         RegMemoryKind::BoxCopyable
    //     } else {
    //         RegMemoryKind::BoxNonCopyable
    //     }
    // }
}

#[salsa::tracked(jar = LayoutJar)]
pub(crate) fn application_reg_memory_kind(
    _db: &dyn LayoutDb,
    _ty: EtherealTermApplication,
) -> RegMemoryKind {
    todo!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegMemoryKind {
    Direct,
    BoxCopyable,
    BoxNonCopyable,
}

impl std::fmt::Display for RegMemoryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegMemoryKind::Direct => "Direct",
            RegMemoryKind::BoxCopyable => "BoxCopyable",
            RegMemoryKind::BoxNonCopyable => "BoxNonCopyable",
        }
        .fmt(f)
    }
}
