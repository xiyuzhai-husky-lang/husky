use crate::{
    template_argument::{
        ty::{LinType, LinTypePathLeading, LinkageRitchieType},
        LinTemplateArgument, LinTemplateArguments,
    },
    *,
};
use ::version_stamp::HasVersionStamp;
use husky_entity_path::TypePath;
use husky_hir_defn::HasHirDefn;
use husky_hir_defn::{version_stamp::HirDefnVersionStamp, HirDefn};

#[salsa::interned(db = LinkageDb, jar = LinkageJar, constructor = new)]
pub struct LinkageVersionStamp {
    data: LinkageVersionStampData,
    dependencies: Vec<LinkageVersionStampDependency>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
pub enum LinkageVersionStampData {
    HirDefn(Linkage),
    Type(LinType),
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinkageVersionStampDependency {
    Linkage(LinkageVersionStamp),
    HirDefn(HirDefnVersionStamp),
}

impl HasVersionStamp for Linkage {
    type VersionStamp = LinkageVersionStamp;

    fn version_stamp(self, db: &::salsa::Db) -> LinkageVersionStamp {
        linkage_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = LinkageJar)]
fn linkage_version_stamp(db: &::salsa::Db, linkage: Linkage) -> LinkageVersionStamp {
    let mut builder = LinkageVersionStampBuilder::new(linkage, db);
    match *linkage.data(db) {
        LinkageData::MajorFunctionRitchie {
            path,
            ref instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinkageData::MajorVal {
            path,
            ref instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinkageData::MemoizedField {
            path,
            ref instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinkageData::MethodRitchie {
            path,
            ref instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinkageData::AssocRitchie {
            path,
            ref instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinkageData::UnveilAssocRitchie {
            path,
            ref instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinkageData::StructConstructor {
            path,
            ref instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinkageData::StructDestructor { self_ty } => builder.add_ty_path_leading(self_ty),
        LinkageData::EnumVariantConstructor {
            path,
            ref instantiation,
            ..
        }
        | LinkageData::EnumVariantDiscriminator {
            path,
            ref instantiation,
            ..
        }
        | LinkageData::EnumVariantDestructor {
            path,
            ref instantiation,
            ..
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation);
        }
        LinkageData::EnumUnitToJsonValue { ty_path } => builder.add_ty_path(ty_path, db),
        LinkageData::StructField { self_ty, .. } => builder.add_ty_path_leading(self_ty),
        LinkageData::EnumVariantField {
            path,
            ref instantiation,
            ..
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation);
        }
        LinkageData::TypeDefault { ty } => builder.add_ty(ty),
        LinkageData::VecConstructor { element_ty } => builder.add(element_ty),
        LinkageData::Index => todo!(),
    }
    builder.finish()
}

impl HasVersionStamp for LinType {
    type VersionStamp = LinkageVersionStamp;

    fn version_stamp(self, db: &salsa::Db) -> Self::VersionStamp {
        match self {
            LinType::PathLeading(slf) => linkage_ty_path_leading_version_stamp(db, slf),
            LinType::Ritchie(slf) => linkage_ty_ritchie_version_stamp(db, slf),
        }
    }
}

#[salsa::tracked(jar = LinkageJar)]
fn linkage_ty_path_leading_version_stamp(
    db: &::salsa::Db,
    linkage_ty: LinTypePathLeading,
) -> LinkageVersionStamp {
    let mut builder = LinkageVersionStampBuilder::new(LinType::PathLeading(linkage_ty), db);
    let hir_defn: HirDefn = linkage_ty.ty_path(db).hir_defn(db).unwrap().into();
    builder.add(hir_defn);
    builder.add_template_arguments(linkage_ty.template_arguments(db));
    builder.finish()
}

#[salsa::tracked(jar = LinkageJar)]
fn linkage_ty_ritchie_version_stamp(
    db: &::salsa::Db,
    linkage_ty: LinkageRitchieType,
) -> LinkageVersionStamp {
    let builder = LinkageVersionStampBuilder::new(LinType::Ritchie(linkage_ty), db);
    builder.finish()
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
        item: impl HasVersionStamp<VersionStamp: Into<LinkageVersionStampDependency>>,
    ) {
        self.substamps.push(item.version_stamp(self.db).into())
    }

    fn add_ty(&mut self, ty: LinType) {
        match ty {
            LinType::PathLeading(ty) => self.add_ty_path_leading(ty),
            LinType::Ritchie(_) => (),
        }
    }

    fn add_ty_path_leading(&mut self, ty: LinTypePathLeading) {
        let db = self.db;
        self.add_ty_path(ty.ty_path(db), db);
        self.add_template_arguments(ty.template_arguments(db))
    }

    fn add_ty_path(&mut self, ty_path: TypePath, db: &salsa::Db) {
        let hir_defn: HirDefn = ty_path.hir_defn(db).unwrap().into();
        self.add(hir_defn);
    }

    fn add_instantiation(&mut self, instantiation: &LinInstantiation) {
        for &(_, res) in &**instantiation {
            self.add_symbol_resolution(res);
        }
    }

    fn add_symbol_resolution(&mut self, res: LinTermSymbolResolution) {
        match res {
            LinTermSymbolResolution::Explicit(arg) => self.add_template_argument(arg),
            LinTermSymbolResolution::SelfLifetime => (),
            LinTermSymbolResolution::SelfQual(_) => (),
        }
    }

    fn add_template_arguments(&mut self, args: &LinTemplateArguments) {
        for &arg in args {
            self.add_template_argument(arg);
        }
    }

    // todo: consider trait implementation
    fn add_template_argument(&mut self, arg: LinTemplateArgument) {
        match arg {
            LinTemplateArgument::Vacant => (),
            LinTemplateArgument::Type(linkage_ty) => self.add(linkage_ty),
            LinTemplateArgument::Constant(_) => (),
            LinTemplateArgument::Lifetime => (),
            LinTemplateArgument::Qual(_) => (),
        }
    }

    pub(crate) fn finish(self) -> LinkageVersionStamp {
        LinkageVersionStamp::new(self.db, self.data, self.substamps)
    }
}
