use crate::{
    template_argument::{
        ty::{LinkageRitchieType, LinkageType, LinkageTypePathLeading},
        LinkageTemplateArgument, LinkageTemplateArguments,
    },
    *,
};
use ::version_stamp::HasVersionStamp;
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
    Type(LinkageType),
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
    match linkage.data(db) {
        LinkageData::FunctionFnItem {
            path,
            instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinkageData::FunctionGnItem {
            path,
            instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinkageData::ValItem {
            path,
            instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinkageData::MemoizedField {
            path,
            instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinkageData::MethodFn {
            path,
            instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinkageData::AssociatedFunctionFn {
            path,
            instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinkageData::UnveilAssociatedFunctionFn {
            path,
            instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinkageData::TypeConstructor {
            path,
            instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinkageData::TypeVariantConstructor {
            path,
            instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        &LinkageData::VecConstructor { element_ty } => builder.add(element_ty),
        &LinkageData::PropsStructField { self_ty, ident } => builder.add(self_ty),
        LinkageData::Index => todo!(),
    }
    builder.finish()
}

impl HasVersionStamp for LinkageType {
    type VersionStamp = LinkageVersionStamp;

    fn version_stamp(self, db: &salsa::Db) -> Self::VersionStamp {
        match self {
            LinkageType::PathLeading(slf) => linkage_ty_path_leading_version_stamp(db, slf),
            LinkageType::Ritchie(slf) => linkage_ty_ritchie_version_stamp(db, slf),
        }
    }
}

#[salsa::tracked(jar = LinkageJar)]
fn linkage_ty_path_leading_version_stamp(
    db: &::salsa::Db,
    linkage_ty: LinkageTypePathLeading,
) -> LinkageVersionStamp {
    let mut builder = LinkageVersionStampBuilder::new(LinkageType::PathLeading(linkage_ty), db);
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
    let mut builder = LinkageVersionStampBuilder::new(LinkageType::Ritchie(linkage_ty), db);
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

    fn add_instantiation(&mut self, instantiation: &LinkageInstantiation) {
        for &(_, res) in &**instantiation {
            self.add_symbol_resolution(res);
        }
    }

    fn add_symbol_resolution(&mut self, res: LinkageTermSymbolResolution) {
        match res {
            LinkageTermSymbolResolution::Explicit(arg) => self.add_template_argument(arg),
            LinkageTermSymbolResolution::SelfLifetime => (),
            LinkageTermSymbolResolution::SelfPlace(_) => (),
        }
    }

    fn add_template_arguments(&mut self, args: &LinkageTemplateArguments) {
        for &arg in args {
            self.add_template_argument(arg);
        }
    }

    // todo: consider trait implementation
    fn add_template_argument(&mut self, arg: LinkageTemplateArgument) {
        match arg {
            LinkageTemplateArgument::Vacant => (),
            LinkageTemplateArgument::Type(linkage_ty) => self.add(linkage_ty),
            LinkageTemplateArgument::Constant(_) => (),
            LinkageTemplateArgument::Lifetime => (),
            LinkageTemplateArgument::Place(_) => (),
        }
    }

    pub(crate) fn finish(self) -> LinkageVersionStamp {
        LinkageVersionStamp::new(self.db, self.data, self.substamps)
    }
}
