use super::*;
use husky_entity_path::TypePath;
use husky_hir_ty::HirType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
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
            HirType::Symbol(slf) => todo!(),
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
            HirType::Symbol(symbol) => {
                use husky_print_utils::p;
                use salsa::DebugWithDb;
                p!(symbol.debug(db), linkage_instantiation.debug(db));
                match linkage_instantiation {
                    Some(linkage_instantiation) => {
                        match linkage_instantiation.resolve(symbol.into()) {
                            LinkageTermSymbolResolution::Explicit(_) => todo!(),
                            LinkageTermSymbolResolution::SelfLifetime => todo!(),
                            LinkageTermSymbolResolution::SelfPlace(_) => todo!(),
                        }
                    }
                    None => todo!(),
                }
            }
            HirType::TypeAssociatedType(_) => unreachable!(),
            HirType::TraitAssociatedType(_) => unreachable!(),
            HirType::Ritchie(_) => LinkageTypeRitchie::new(db).into(),
        }
    }
}
