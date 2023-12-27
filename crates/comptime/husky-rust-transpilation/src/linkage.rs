use crate::*;
use either::*;
use husky_corgi_config::transpilation_setup::TranspilationSetup;
use husky_entity_kind::FugitiveKind;
use husky_entity_path::{
    AssociatedItemPath, FugitivePath, MajorItemPath, PatternPath, PreludeIntTypePath,
    PreludeNumTypePath, PreludeTypePath, PrincipalEntityPath, TraitForTypeItemPath, TraitItemPath,
    TraitPath, TypeItemPath, TypePath, TypeSketch, TypeVariantPath,
};
use husky_ethereal_signature::signature::HasEtherealSignatureTemplate;
use husky_hir_decl::HasHirDecl;
use husky_hir_ty::{ritchie::HirEagerContract, trai::HirTrait, HirType};
use husky_javelin::{javelin::JavelinData, path::JavelinPath};
use husky_linkage::{
    instantiation::{LinkageInstantiate, LinkageInstantiation, LinkageTermSymbolResolution},
    linkage::LinkageStructField,
    template_argument::{
        place,
        ty::{LinkageRitchieParameter, LinkageRitchieType, LinkageType},
        LinkageTemplateArgument,
    },
    trai::LinkageTrait,
};
use husky_linkage::{
    linkage::{package_linkages, Linkage, LinkageData},
    template_argument::ty::LinkageTypePathLeading,
};
use husky_vfs::{CrateKind, ModulePathData, PackagePathSource};
use salsa::DebugWithDb;

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn package_linkages_transpilation(
    db: &::salsa::Db,
    package_path: PackagePath,
    setup: TranspilationSetup,
) -> String {
    let mut builder_base = RustTranspilationBuilderBase::new(
        db,
        package_path.toolchain(db),
        setup,
        Some(format!(
            r#"use husky_core::*;
use {}::{{*, ugly::*}};
use {}::*;
"#,
            setup.rust_data(db).unwrap().task_dependency_ident.data(db),
            package_path.ident(db).data(db)
        )),
        None,
    );
    let mut builder = RustTranspilationBuilder::new(&mut builder_base);
    builder.on_fresh_semicolon_paragraph(|builder| {
        builder.rustfmt_skip();
        builder.macro_name(RustMacroName::LinkageImpls);
        builder.bracketed_multiline_comma_list(RustBracket::Box, package_linkages(db, package_path))
    });
    builder_base.finish()
}

impl TranspileToRustWith<()> for Linkage {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<()>) {
        let db = builder.db;
        match *self.data(db) {
            LinkageData::FunctionFnItem {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                (path, instantiation).transpile_to_rust(builder)
            }),
            LinkageData::FunctionGnItem {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::GnLinkageImpl, |builder| {
                (path, instantiation).transpile_to_rust(builder)
            }),
            LinkageData::ValItem {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                path.transpile_to_rust(builder)
            }),
            LinkageData::MethodFn {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                (path, instantiation).transpile_to_rust(builder)
            }),
            LinkageData::TypeConstructor {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                builder.ty_constructor_linkage(path)
            }),
            LinkageData::AssociatedFunctionFn {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                (path, instantiation).transpile_to_rust(builder)
            }),
            LinkageData::UnveilAssociatedFunctionFn {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::UnveilFnLinkageImpl, |builder| {
                (path, instantiation).transpile_to_rust(builder)
            }),
            LinkageData::MemoizedField {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                path.transpile_to_rust(builder)
            }),
            LinkageData::Index => todo!(),
            LinkageData::TypeVariantConstructor {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                (path, instantiation).transpile_to_rust(builder)
            }),
            LinkageData::StructField { self_ty, field } => {
                builder.macro_call(RustMacroName::StructFieldLinkageImpl, |builder| {
                    self_ty.transpile_to_rust(builder);
                    builder.punctuation(RustPunctuation::CommaSpaced);
                    match field {
                        LinkageStructField::Tuple => todo!(),
                        LinkageStructField::Props { ident } => ident.transpile_to_rust(builder),
                    }
                })
            }
            LinkageData::TypeDefault { ty } => builder
                .macro_call(RustMacroName::TypeDefault, |builder| {
                    ty.transpile_to_rust(builder)
                }),
            LinkageData::VecConstructor { element_ty } => {
                builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                    builder.bracketed(RustBracket::Vertical, |builder| {
                        builder.v();
                        builder.punctuation(RustPunctuation::Colon);
                        builder.vec_ty(element_ty)
                    });
                    builder.v()
                })
            }
        }
    }
}

impl<E> TranspileToRustWith<E> for (AssociatedItemPath, &LinkageInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, instantiation) = self;
        match path {
            AssociatedItemPath::TypeItem(slf) => (slf, instantiation).transpile_to_rust(builder),
            AssociatedItemPath::TraitItem(slf) => (slf, instantiation).transpile_to_rust(builder),
            AssociatedItemPath::TraitForTypeItem(slf) => {
                (slf, instantiation).transpile_to_rust(builder)
            }
        }
    }
}

impl<E> TranspileToRustWith<E> for (FugitivePath, &LinkageInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, instantiation) = self;
        path.transpile_to_rust(builder);
        if !instantiation.is_empty() {
            builder.bracketed_comma_list(
                RustBracket::TurboFish,
                instantiation.iter().map(|&(_, res)| match res {
                    LinkageTermSymbolResolution::Explicit(arg) => arg,
                    LinkageTermSymbolResolution::SelfLifetime => unreachable!(),
                    LinkageTermSymbolResolution::SelfPlace(_) => unreachable!(),
                }),
            )
        }
    }
}

impl<E> TranspileToRustWith<E> for (TypeVariantPath, &LinkageInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, instantiation) = self;
        if instantiation.is_empty() {
            path.transpile_to_rust(builder)
        } else {
            todo!()
        }
    }
}

impl<E> TranspileToRustWith<E> for (TypeItemPath, &LinkageInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, linkage_instantiation) = self;
        let db = builder.db;
        let self_ty = HirType::from_ethereal(
            path.impl_block(db)
                .ethereal_signature_template(db)
                .unwrap()
                .self_ty(db),
            db,
        )
        .unwrap()
        .linkage_instantiate(linkage_instantiation, db);
        let ident = path.ident(db).unwrap();
        builder.bracketed(RustBracket::Angle, |builder| {
            match self_ty {
                LinkageType::PathLeading(self_ty) => match self_ty.ty_path(db).refine(db) {
                    Left(PreludeTypePath::VEC) => match ident.data(db) {
                        "first" | "last" => {
                            builder.bracketed_comma_list(
                                RustBracket::Box,
                                self_ty.template_arguments(db),
                            );
                            return;
                        }
                        _ => (),
                    },
                    Left(PreludeTypePath::CYCLIC_SLICE) => {
                        builder.cyclic_slice_leashed_ty();
                        builder.bracketed_comma_list(
                            RustBracket::Angle,
                            self_ty.template_arguments(db),
                        );
                        return;
                    }
                    _ => (),
                },
                _ => (),
            }
            self_ty.transpile_to_rust(builder)
        });
        builder.punctuation(RustPunctuation::ColonColon);
        let places = linkage_instantiation.places();
        match places.len() {
            0 => ident.transpile_to_rust(builder),
            1 => {
                let (symbol, place) = places[0];
                match place {
                    LinkageTermSymbolResolution::Explicit(LinkageTemplateArgument::Place(_)) => {
                        todo!()
                    }
                    LinkageTermSymbolResolution::SelfPlace(place) => match place {
                        place::LinkagePlace::Ref => ident.transpile_to_rust(builder),
                        place::LinkagePlace::RefMut => builder.method_fn_ident_mut(ident),
                        place::LinkagePlace::Transient => todo!(),
                    },
                    _ => unreachable!(),
                }
            }
            _ => todo!(),
        }
    }
}

impl<E> TranspileToRustWith<E> for LinkageType {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db;
        match self {
            LinkageType::PathLeading(slf) => slf.transpile_to_rust(builder),
            LinkageType::Ritchie(slf) => slf.transpile_to_rust(builder),
        }
    }
}

impl<E> TranspileToRustWith<E> for LinkageTrait {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db;
        self.trai_path(db).transpile_to_rust(builder);
        let template_arguments = self.template_arguments(db);
        if !template_arguments.is_empty() {
            builder.bracketed_comma_list(RustBracket::Angle, template_arguments)
        }
    }
}

impl<E> TranspileToRustWith<E> for LinkageTypePathLeading {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db;
        self.ty_path(db).transpile_to_rust(builder);
        let template_arguments = self.template_arguments(db);
        if !template_arguments.is_empty() {
            builder.bracketed_comma_list(RustBracket::Angle, template_arguments)
        }
    }
}

impl<E> TranspileToRustWith<E> for LinkageTemplateArgument {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        match self {
            LinkageTemplateArgument::Vacant => todo!(),
            LinkageTemplateArgument::Type(linkage_ty) => linkage_ty.transpile_to_rust(builder),
            LinkageTemplateArgument::Constant(constant) => todo!(),
            LinkageTemplateArgument::Lifetime => todo!(),
            LinkageTemplateArgument::Place(_) => todo!(),
        }
    }
}

impl<E> TranspileToRustWith<E> for LinkageRitchieType {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db();
        builder.keyword(RustKeyword::Fn);
        builder.bracketed_comma_list(RustBracket::Par, self.parameters(db).iter());
        builder.punctuation(RustPunctuation::LightArrow);
        self.return_ty(db).transpile_to_rust(builder)
    }
}

impl<E> TranspileToRustWith<E> for LinkageRitchieParameter {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        match self.contract() {
            // ad hoc
            HirEagerContract::Pure => (),
            // builder.punctuation(RustPunctuation::Ambersand),
            HirEagerContract::Move => (),
            HirEagerContract::Borrow => builder.punctuation(RustPunctuation::Ambersand),
            HirEagerContract::BorrowMut => todo!(),
            HirEagerContract::Const => todo!(),
            HirEagerContract::Leash => todo!(),
            HirEagerContract::At => todo!(),
        }
        self.parameter_ty().transpile_to_rust(builder)
    }
}

impl<E> TranspileToRustWith<E> for (TraitItemPath, &LinkageInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, instantiation) = self;
        let db = builder.db;
        path.trai_path(db).transpile_to_rust(builder);
        builder.punctuation(RustPunctuation::ColonColon);
        path.ident(db).transpile_to_rust(builder)
    }
}

impl<E> TranspileToRustWith<E> for (TraitForTypeItemPath, &LinkageInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, linkage_instantiation) = self;
        let db = builder.db;
        builder.bracketed(RustBracket::Angle, |builder| {
            let trait_for_type_impl_block_ethereal_signature_template =
                path.impl_block(db).ethereal_signature_template(db).unwrap();
            let self_ty = HirType::from_ethereal(
                trait_for_type_impl_block_ethereal_signature_template.self_ty(db),
                db,
            )
            .unwrap()
            .linkage_instantiate(linkage_instantiation, db);
            self_ty.transpile_to_rust(builder);
            builder.keyword(RustKeyword::As);
            let trai = HirTrait::from_ethereal(
                trait_for_type_impl_block_ethereal_signature_template.trai(db),
                db,
            );
            trai.linkage_instantiate(linkage_instantiation, db)
                .transpile_to_rust(builder)
        });
        builder.punctuation(RustPunctuation::ColonColon);
        path.ident(db).transpile_to_rust(builder)
    }
}
