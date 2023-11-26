use crate::{template_argument::ty::LinkageType, *};
use husky_hir_defn::version_stamp::HirDefnVersionStamp;

#[salsa::interned(db = LinkageDb, jar = LinkageJar, constructor = new)]
pub struct LinkageVersionStamp {
    data: LinkageVersionStampData,
    dependencies: Vec<LinkageVersionStampDependency>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
pub enum LinkageVersionStampData {
    HirDefn(Linkage),
    Type(LinkageType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
pub enum LinkageVersionStampDependency {
    Linkage(LinkageVersionStamp),
    HirDefn(HirDefnVersionStamp),
}

pub struct LinkageVersionStampBuilder<'a> {
    data: LinkageVersionStampData,
    substamps: Vec<LinkageVersionStampDependency>,
    db: &'a ::salsa::Db,
}

impl<'a> LinkageVersionStampBuilder<'a> {
    pub(crate) fn new(data: impl Into<LinkageVersionStampData>, db: &'a ::salsa::Db) -> Self {
        Self {
            data: data.into(),
            substamps: vec![],
            db,
        }
    }

    pub(crate) fn add(
        &mut self,
        item: impl HasVersionStamp<::salsa::Db + 'a, VersionStamp: Into<LinkageVersionStampDependency>>,
    ) {
        self.substamps.push(item.version_stamp(self.db).into())
    }

    pub(crate) fn finish(self) -> LinkageVersionStamp {
        LinkageVersionStamp::new(self.db, self.data, self.substamps)
    }
}
