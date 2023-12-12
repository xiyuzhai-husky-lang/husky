use super::*;
use husky_entity_path::TypePath;
use husky_hir_ty::HirType;
use husky_javelin::template_argument::ty::{JavelinType, JavelinTypePathLeading};

#[salsa::debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LinkageType {
    PathLeading(LinkageTypePathLeading),
    Ritchie(LinkageTypeRitchie),
}

impl LinkageInstantiate for HirType {
    type Output = LinkageType;

    fn linkage_instantiate(
        self,
        linkage_instantiation: &LinkageInstantiation,
        db: &salsa::Db,
    ) -> Self::Output {
        match self {
            HirType::PathLeading(slf) => LinkageType::PathLeading(LinkageTypePathLeading::new(
                db,
                slf.ty_path(db),
                slf.template_arguments(db)
                    .iter()
                    .map(|&arg| {
                        LinkageTemplateArgument::from_hir(arg, Some(linkage_instantiation), db)
                    })
                    .collect(),
            )),
            HirType::Symbol(slf) => match linkage_instantiation.resolve(slf.into()) {
                LinkageTermSymbolResolution::Explicit(arg) => match arg {
                    LinkageTemplateArgument::Vacant => todo!(),
                    LinkageTemplateArgument::Type(linkage_ty) => linkage_ty,
                    LinkageTemplateArgument::Constant(_) => todo!(),
                    LinkageTemplateArgument::Lifetime => todo!(),
                    LinkageTemplateArgument::Place(_) => todo!(),
                },
                LinkageTermSymbolResolution::SelfLifetime => todo!(),
                LinkageTermSymbolResolution::SelfPlace(_) => todo!(),
            },
            HirType::TypeAssociatedType(_) => todo!(),
            HirType::TraitAssociatedType(_) => todo!(),
            HirType::Ritchie(_) => todo!(),
        }
    }
}

#[salsa::interned(db = LinkageDb, jar = LinkageJar, constructor = new)]
pub struct LinkageTypePathLeading {
    pub ty_path: TypePath,
    /// phantom arguments are ignored
    #[return_ref]
    pub template_arguments: LinkageTemplateArguments,
}

#[salsa::interned(db = LinkageDb, jar = LinkageJar, constructor = new)]
pub struct LinkageTypeRitchie {}

impl LinkageType {
    pub(crate) fn from_hir(
        hir_ty: HirType,
        linkage_instantiation: Option<&LinkageInstantiation>,
        db: &::salsa::Db,
    ) -> Self {
        match hir_ty {
            HirType::PathLeading(hir_ty) => LinkageTypePathLeading::new(
                db,
                hir_ty.ty_path(db),
                LinkageTemplateArgument::from_hir_template_arguments(
                    hir_ty.template_arguments(db),
                    linkage_instantiation,
                    db,
                ),
            )
            .into(),
            HirType::Symbol(symbol) => match linkage_instantiation {
                Some(linkage_instantiation) => match linkage_instantiation.resolve(symbol.into()) {
                    LinkageTermSymbolResolution::Explicit(arg) => match arg {
                        LinkageTemplateArgument::Vacant => todo!(),
                        LinkageTemplateArgument::Type(linkage_ty) => linkage_ty,
                        LinkageTemplateArgument::Constant(_) => todo!(),
                        LinkageTemplateArgument::Lifetime => todo!(),
                        LinkageTemplateArgument::Place(_) => todo!(),
                    },
                    LinkageTermSymbolResolution::SelfLifetime => todo!(),
                    LinkageTermSymbolResolution::SelfPlace(_) => todo!(),
                },
                None => todo!(),
            },
            HirType::TypeAssociatedType(_) => unreachable!(),
            HirType::TraitAssociatedType(_) => unreachable!(),
            HirType::Ritchie(_) => LinkageTypeRitchie::new(db).into(),
        }
    }

    pub(crate) fn from_javelin(
        javelin_ty: JavelinType,
        linkage_instantiation: &LinkageInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        match javelin_ty {
            JavelinType::PathLeading(javelin_ty) => {
                LinkageTypePathLeading::from_javelin(javelin_ty, linkage_instantiation, db).into()
            }
            JavelinType::Ritchie(javelin_ty) => LinkageTypeRitchie::new(db).into(),
        }
    }
}

impl LinkageTypePathLeading {
    fn from_javelin(
        javelin_ty: JavelinTypePathLeading,
        linkage_instantiation: &LinkageInstantiation,
        db: &::salsa::Db,
    ) -> Self {
        LinkageTypePathLeading::new(
            db,
            javelin_ty.ty_path(db),
            javelin_ty
                .template_arguments(db)
                .iter()
                .map(|&arg| LinkageTemplateArgument::from_javelin(arg, linkage_instantiation, db))
                .collect(),
        )
    }
}
