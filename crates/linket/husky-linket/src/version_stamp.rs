use crate::{
    template_argument::{
        ty::{LinRitchieType, LinType, LinTypePathLeading},
        LinTemplateArgument, LinTemplateArguments,
    },
    *,
};
use ::version_stamp::HasVersionStamp;
use context::LinComptimeVarOverride;
use husky_entity_path::path::major_item::ty::TypePath;
use husky_hir_defn::{
    defn::{HasHirDefn, HirDefn},
    version_stamp::HirDefnVersionStamp,
};

#[salsa::interned(constructor = new)]
pub struct LinketVersionStamp {
    data: LinketVersionStampData,
    dependencies: Vec<LinketVersionStampDependency>,
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinketVersionStampData {
    HirDefn(Linket),
    Type(LinType),
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinketVersionStampDependency {
    Linket(LinketVersionStamp),
    HirDefn(HirDefnVersionStamp),
}

impl HasVersionStamp for Linket {
    type VersionStamp = LinketVersionStamp;

    fn version_stamp(self, db: &::salsa::Db) -> LinketVersionStamp {
        linket_version_stamp(db, self)
    }
}

#[salsa::tracked]
fn linket_version_stamp(db: &::salsa::Db, linket: Linket) -> LinketVersionStamp {
    let mut builder = LinketVersionStampBuilder::new(linket, db);
    match *linket.data(db) {
        LinketData::MajorRitchie {
            path,
            ref instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinketData::MajorStaticVar {
            path,
            ref instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinketData::MajorVal {
            path,
            ref instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinketData::Memo {
            path,
            ref instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinketData::MethodRitchie {
            path,
            ref instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinketData::AssocRitchie {
            path,
            ref instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinketData::UnveilAssocRitchie {
            path,
            ref instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinketData::StructConstructor {
            path,
            ref instantiation,
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation)
        }
        LinketData::StructDestructor { self_ty } => builder.add_ty_path_leading(self_ty),
        LinketData::EnumVariantConstructor {
            path,
            ref instantiation,
            ..
        }
        | LinketData::EnumVariantDiscriminator {
            path,
            ref instantiation,
            ..
        }
        | LinketData::EnumVariantDestructor {
            path,
            ref instantiation,
            ..
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation);
        }
        LinketData::EnumUnitToJsonValue { ty_path } => builder.add_ty_path(ty_path, db),
        LinketData::StructField { self_ty, .. } => builder.add_ty_path_leading(self_ty),
        LinketData::EnumVariantField {
            path,
            ref instantiation,
            ..
        } => {
            let hir_defn: HirDefn = path.hir_defn(db).unwrap().into();
            builder.add(hir_defn);
            builder.add_instantiation(instantiation);
        }
        LinketData::TypeDefault { ty } => builder.add_ty(ty),
        LinketData::VecConstructor { element_ty } => builder.add(element_ty),
        LinketData::Index => todo!(),
    }
    builder.finish()
}

impl HasVersionStamp for LinType {
    type VersionStamp = LinketVersionStamp;

    fn version_stamp(self, db: &salsa::Db) -> Self::VersionStamp {
        match self {
            LinType::PathLeading(slf) => linket_ty_path_leading_version_stamp(db, slf),
            LinType::Ritchie(slf) => linket_ty_ritchie_version_stamp(db, slf),
        }
    }
}

#[salsa::tracked]
fn linket_ty_path_leading_version_stamp(
    db: &::salsa::Db,
    linket_ty: LinTypePathLeading,
) -> LinketVersionStamp {
    let mut builder = LinketVersionStampBuilder::new(LinType::PathLeading(linket_ty), db);
    let hir_defn: HirDefn = linket_ty.ty_path(db).hir_defn(db).unwrap().into();
    builder.add(hir_defn);
    builder.add_template_arguments(linket_ty.template_arguments(db));
    builder.finish()
}

#[salsa::tracked]
fn linket_ty_ritchie_version_stamp(
    db: &::salsa::Db,
    linket_ty: LinRitchieType,
) -> LinketVersionStamp {
    let builder = LinketVersionStampBuilder::new(LinType::Ritchie(linket_ty), db);
    builder.finish()
}

pub struct LinketVersionStampBuilder<'a> {
    data: LinketVersionStampData,
    substamps: Vec<LinketVersionStampDependency>,
    db: &'a ::salsa::Db,
}

impl<'a> LinketVersionStampBuilder<'a> {
    pub(crate) fn new(data: impl Into<LinketVersionStampData>, db: &'a ::salsa::Db) -> Self {
        Self {
            data: data.into(),
            substamps: vec![],
            db,
        }
    }

    pub(crate) fn add(
        &mut self,
        item: impl HasVersionStamp<VersionStamp: Into<LinketVersionStampDependency>>,
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
        for &(_, ovrd) in instantiation.context().comptime_var_overrides() {
            self.add_comptime_var_override(ovrd);
        }
        for &(_, res) in instantiation.variable_resolutions() {
            self.add_symbol_resolution(res);
        }
    }

    fn add_comptime_var_override(&mut self, ovrd: LinComptimeVarOverride) {
        match ovrd {
            LinComptimeVarOverride::Type(ty) => self.add_ty(ty),
        }
    }

    fn add_symbol_resolution(&mut self, res: LinTermVariableResolution) {
        match res {
            LinTermVariableResolution::Explicit(arg) => self.add_template_argument(arg),
            LinTermVariableResolution::SelfLifetime => (),
            LinTermVariableResolution::SelfQual(_) => (),
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
            LinTemplateArgument::Type(linket_ty) => self.add(linket_ty),
            LinTemplateArgument::Constant(_) => (),
            LinTemplateArgument::Lifetime => (),
            LinTemplateArgument::Qual(_) => (),
        }
    }

    pub(crate) fn finish(self) -> LinketVersionStamp {
        LinketVersionStamp::new(self.db, self.data, self.substamps)
    }
}
