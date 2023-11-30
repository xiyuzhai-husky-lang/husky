use crate::{template_argument::ty::JavelinType, *};
use ::version_stamp::HasVersionStamp;
use husky_hir_defn::version_stamp::HirDefnVersionStamp;

#[salsa::interned(db = JavelinDb, jar = JavelinJar, constructor = new)]
pub struct JavelinVersionStamp {
    data: JavelinVersionStampData,
    dependencies: Vec<JavelinVersionStampDependency>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
pub enum JavelinVersionStampData {
    HirDefn(Javelin),
    Type(JavelinType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
pub enum JavelinVersionStampDependency {
    Javelin(JavelinVersionStamp),
    HirDefn(HirDefnVersionStamp),
}

pub struct JavelinVersionStampBuilder<'a> {
    data: JavelinVersionStampData,
    substamps: Vec<JavelinVersionStampDependency>,
    db: &'a ::salsa::Db,
}

impl<'a> JavelinVersionStampBuilder<'a> {
    pub(crate) fn new(data: impl Into<JavelinVersionStampData>, db: &'a ::salsa::Db) -> Self {
        Self {
            data: data.into(),
            substamps: vec![],
            db,
        }
    }

    pub(crate) fn add(
        &mut self,
        item: impl HasVersionStamp<VersionStamp: Into<JavelinVersionStampDependency>>,
    ) {
        self.substamps.push(item.version_stamp(self.db).into())
    }

    pub(crate) fn finish(self) -> JavelinVersionStamp {
        JavelinVersionStamp::new(self.db, self.data, self.substamps)
    }
}
