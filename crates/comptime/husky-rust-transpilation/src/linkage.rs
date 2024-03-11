use crate::*;
use either::*;
use husky_corgi_config::transpilation_setup::TranspilationSetup;
use husky_entity_path::{
    AssocItemPath, FugitivePath, PreludeTypePath, TraitForTypeItemPath, TraitItemPath,
    TypeItemPath, TypeVariantPath,
};
use husky_eth_signature::signature::HasEthTemplate;
use husky_hir_ty::{ritchie::HirEagerContract, trai::HirTrait, HirType};
use husky_javelin::template_argument::constant::JavelinConstant;
use husky_linkage::{
    instantiation::{LinInstantiation, LinTermSymbolResolution, LinkageInstantiate},
    linkage::LinkageStructField,
    template_argument::{
        constant::LinConstant,
        qual,
        ty::{LinType, LinkageRitchieParameter, LinkageRitchieType},
        LinTemplateArgument,
    },
    trai::LinkageTrait,
};
use husky_linkage::{
    linkage::{package_linkages, Linkage, LinkageData},
    template_argument::ty::LinTypePathLeading,
};

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
            r#"#![feature(trait_upcasting)]
use husky_core::*;
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
        builder
            .bracketed_multiline_comma_list(RustDelimiter::Box, package_linkages(db, package_path))
    });
    builder_base.finish()
}

impl TranspileToRustWith<()> for Linkage {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<()>) {
        let db = builder.db;
        match *self.data(db) {
            LinkageData::MajorRitchieEager {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                (path, instantiation).transpile_to_rust(builder)
            }),
            LinkageData::MajorRitchieLazy {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::GnLinkageImpl, |builder| {
                (path, instantiation).transpile_to_rust(builder)
            }),
            LinkageData::MajorVal {
                path,
                instantiation: _,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                path.transpile_to_rust(builder)
            }),
            LinkageData::MethodRitchie {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                (path, instantiation).transpile_to_rust(builder)
            }),
            LinkageData::StructTypeConstructor {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                builder.struct_ty_constructor_path(path);
                turbo_fish_instantiation(instantiation, builder);
            }),
            LinkageData::StructTypeDestructor {
                path,
                ref instantiation,
                qual,
            } => builder.macro_call(RustMacroName::DestructorFnLinkageImpl, |builder| {
                builder.struct_ty_destructor_path(path, qual);
                turbo_fish_instantiation(instantiation, builder);
            }),
            LinkageData::EnumTypeVariantConstructor {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                (path, instantiation).transpile_to_rust(builder)
            }),
            LinkageData::EnumTypeVariantDiscriminator {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                builder.enum_ty_variant_discriminator_path(path);
                turbo_fish_instantiation(instantiation, builder);
            }),
            LinkageData::EnumTypeVariantDestructor {
                path,
                ref instantiation,
                qual,
            } => builder.macro_call(RustMacroName::DestructorFnLinkageImpl, |builder| {
                builder.enum_ty_variant_destructor_path(path, qual);
                turbo_fish_instantiation(instantiation, builder);
            }),
            LinkageData::EnumU8ToJsonValue { ty_path } => builder
                .macro_call(RustMacroName::EnumU8Presenter, |builder| {
                    ty_path.transpile_to_rust(builder)
                }),
            LinkageData::AssocRitchie {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                (path, instantiation).transpile_to_rust(builder)
            }),
            LinkageData::UnveilAssocFn {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::UnveilFnLinkageImpl, |builder| {
                (path, instantiation).transpile_to_rust(builder)
            }),
            LinkageData::MemoizedField {
                path,
                instantiation: _,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                path.transpile_to_rust(builder)
            }),
            LinkageData::Index => todo!(),
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
                    builder.bracketed(RustDelimiter::Vert, |builder| {
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

impl<E> TranspileToRustWith<E> for (AssocItemPath, &LinInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, instantiation) = self;
        match path {
            AssocItemPath::TypeItem(slf) => (slf, instantiation).transpile_to_rust(builder),
            AssocItemPath::TraitItem(slf) => (slf, instantiation).transpile_to_rust(builder),
            AssocItemPath::TraitForTypeItem(slf) => (slf, instantiation).transpile_to_rust(builder),
        }
    }
}

impl<E> TranspileToRustWith<E> for (FugitivePath, &LinInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, instantiation) = self;
        path.transpile_to_rust(builder);
        turbo_fish_instantiation(instantiation, builder);
    }
}

fn turbo_fish_instantiation<E>(
    instantiation: &LinInstantiation,
    builder: &mut RustTranspilationBuilder<'_, '_, E>,
) {
    if !instantiation.is_empty() {
        builder.bracketed_comma_list(
            RustDelimiter::TurboFish,
            instantiation.iter().map(|&(_, res)| match res {
                LinTermSymbolResolution::Explicit(arg) => arg,
                LinTermSymbolResolution::SelfLifetime => todo!(),
                LinTermSymbolResolution::SelfQuary(_) => todo!(),
            }),
        )
    }
}

impl<E> TranspileToRustWith<E> for (TypeVariantPath, &LinInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, instantiation) = self;
        path.transpile_to_rust(builder);
        turbo_fish_instantiation(instantiation, builder);
    }
}

impl<E> TranspileToRustWith<E> for (TypeItemPath, &LinInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, lin_instantiation) = self;
        let db = builder.db;
        let self_ty = HirType::from_eth(
            path.impl_block(db).eth_template(db).unwrap().self_ty(db),
            db,
        )
        .unwrap()
        .linkage_instantiate(lin_instantiation, db);
        let ident = path.ident(db).unwrap();
        builder.bracketed(RustDelimiter::Angle, |builder| {
            match self_ty {
                LinType::PathLeading(self_ty) => match self_ty.ty_path(db).refine(db) {
                    Left(PreludeTypePath::VEC) => match ident.data(db) {
                        "first" | "last" => {
                            builder.bracketed_comma_list(
                                RustDelimiter::Box,
                                self_ty.template_arguments(db),
                            );
                            return;
                        }
                        _ => (),
                    },
                    Left(PreludeTypePath::CYCLIC_SLICE) => {
                        builder.cyclic_slice_leashed_ty();
                        builder.bracketed_comma_list(
                            RustDelimiter::Angle,
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
        let places = lin_instantiation.places();
        match places.len() {
            0 => ident.transpile_to_rust(builder),
            1 => {
                let (_symbol, place) = places[0];
                match place {
                    LinTermSymbolResolution::Explicit(LinTemplateArgument::Qual(_)) => {
                        todo!()
                    }
                    LinTermSymbolResolution::SelfQuary(place) => match place {
                        qual::LinQual::Ref => ident.transpile_to_rust(builder),
                        qual::LinQual::RefMut => builder.method_fn_ident_mut(ident),
                        qual::LinQual::Transient => todo!(),
                    },
                    _ => unreachable!(),
                }
            }
            _ => todo!(),
        }
    }
}

impl<E> TranspileToRustWith<E> for LinType {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let _db = builder.db;
        match self {
            LinType::PathLeading(slf) => slf.transpile_to_rust(builder),
            LinType::Ritchie(slf) => slf.transpile_to_rust(builder),
        }
    }
}

impl<E> TranspileToRustWith<E> for LinkageTrait {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db;
        self.trai_path(db).transpile_to_rust(builder);
        let template_arguments = self.template_arguments(db);
        if !template_arguments.is_empty() {
            builder.bracketed_comma_list(RustDelimiter::Angle, template_arguments)
        }
    }
}

impl<E> TranspileToRustWith<E> for LinTypePathLeading {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db;
        let template_arguments = self.template_arguments(db);
        match self.ty_path(db).refine(db) {
            Left(PreludeTypePath::REF) => {
                debug_assert_eq!(template_arguments.len(), 2);
                builder.punctuation(RustPunctuation::Ambersand);
                template_arguments[0].transpile_to_rust(builder);
                template_arguments[1].transpile_to_rust(builder)
            }
            Left(PreludeTypePath::REF_MUT) => todo!(),
            Left(PreludeTypePath::SLICE) => todo!(),
            _ => {
                self.ty_path(db).transpile_to_rust(builder);
                let template_arguments = self.template_arguments(db);
                if !template_arguments.is_empty() {
                    builder.bracketed_comma_list(RustDelimiter::Angle, template_arguments)
                }
            }
        }
    }
}

impl<E> TranspileToRustWith<E> for LinTemplateArgument {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        match self {
            LinTemplateArgument::Vacant => todo!(),
            LinTemplateArgument::Type(linkage_ty) => linkage_ty.transpile_to_rust(builder),
            LinTemplateArgument::Constant(constant) => constant.transpile_to_rust(builder),
            LinTemplateArgument::Lifetime => todo!(),
            LinTemplateArgument::Qual(_) => todo!(),
        }
    }
}

impl<E> TranspileToRustWith<E> for LinkageRitchieType {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db();
        builder.keyword(RustKeyword::Fn);
        builder.bracketed_comma_list(RustDelimiter::Par, self.parameters(db).iter());
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

impl<E> TranspileToRustWith<E> for (TraitItemPath, &LinInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, _instantiation) = self;
        let db = builder.db;
        path.trai_path(db).transpile_to_rust(builder);
        builder.punctuation(RustPunctuation::ColonColon);
        path.ident(db).transpile_to_rust(builder)
    }
}

impl<E> TranspileToRustWith<E> for (TraitForTypeItemPath, &LinInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, lin_instantiation) = self;
        let db = builder.db;
        builder.bracketed(RustDelimiter::Angle, |builder| {
            let trait_for_type_impl_block_eth_template =
                path.impl_block(db).eth_template(db).unwrap();
            let self_ty = HirType::from_eth(trait_for_type_impl_block_eth_template.self_ty(db), db)
                .unwrap()
                .linkage_instantiate(lin_instantiation, db);
            self_ty.transpile_to_rust(builder);
            builder.keyword(RustKeyword::As);
            let trai = HirTrait::from_eth(trait_for_type_impl_block_eth_template.trai(db), db);
            trai.linkage_instantiate(lin_instantiation, db)
                .transpile_to_rust(builder)
        });
        builder.punctuation(RustPunctuation::ColonColon);
        path.ident(db).transpile_to_rust(builder)
    }
}
