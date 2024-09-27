use crate::*;
use either::*;
use husky_corgi_config::transpilation_setup::TranspilationSetup;
use husky_entity_kind::ritchie::RitchieItemKind;
use husky_entity_path::path::{
    assoc_item::{
        trai_for_ty_item::TraitForTypeItemPath, trai_item::TraitItemPath, ty_item::TypeItemPath,
        AssocItemPath,
    },
    major_item::{form::MajorFormPath, ty::PreludeTypePath},
    ty_variant::TypeVariantPath,
};
use husky_eth_signature::signature::HasEthTemplate;
use husky_hir_decl::decl::{HasHirDecl, TypeHirDecl, TypeVariantHirDecl};
use husky_hir_ty::{ritchie::HirContract, trai::HirTrait, HirType};
use husky_javelin::template_argument::constant::JavelinConstant;
use husky_linket::{
    context::LinComptimeVarOverride,
    instantiation::{LinInstantiate, LinInstantiation, LinTermVariableResolution},
    linket::{ty::LinLeashClass, LinField},
    template_argument::{
        constant::LinConstant,
        qual::{self, LinQual},
        ty::{LinRitchieParameter, LinRitchieType, LinType},
        LinTemplateArgument,
    },
    trai::LinketTrait,
};
use husky_linket::{
    linket::{package_linkets, Linket, LinketData},
    template_argument::ty::LinTypePathLeading,
};
use husky_vfs::path::linktime_target_path::LinktimeTargetPath;

use self::helpers::TupleFieldVariable;

#[salsa::tracked(return_ref)]
pub(crate) fn package_linkets_transpilation(
    db: &::salsa::Db,
    target_path: LinktimeTargetPath,
    setup: TranspilationSetup,
) -> String {
    let mut builder_base = RustTranspilationBuilderBase::new(
        db,
        target_path.toolchain(db),
        setup,
        Some(format!(
            r#"#![feature(trait_upcasting)]
use husky_core::*;
use {}::{{*, ugly::*}};
"#,
            setup.rust_data(db).unwrap().task_dependency_ident.data(db),
        )),
        None,
    );
    let mut builder = RustTranspilationBuilder::new(&mut builder_base);
    builder.on_fresh_semicolon_paragraph(|builder| {
        use husky_manifest::HasManifest;

        builder.rustfmt_skip();
        builder.macro_name(RustMacroName::LinketImpls);
        builder.delimited_multiline_comma_list(
            RustDelimiter::Box,
            target_path
                .full_dependencies(db)
                .unwrap()
                .iter()
                .map(|&package_path| package_linkets(db, package_path))
                .flatten(),
        );
    });
    builder_base.finish()
}

impl TranspileToRustWith<()> for Linket {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<()>) {
        let db = builder.db;
        match *self.data(db) {
            LinketData::MajorFunctionRitchie {
                path,
                ref instantiation,
            } => match path.kind(db).ritchie() {
                RitchieItemKind::Fn => builder.macro_call(RustMacroName::FnLinketImpl, |builder| {
                    (path, instantiation).transpile_to_rust(builder)
                }),
                RitchieItemKind::Gn => builder.macro_call(RustMacroName::GnLinketImpl, |builder| {
                    (path, instantiation).transpile_to_rust(builder)
                }),
                RitchieItemKind::Vn => todo!(),
                RitchieItemKind::Pn => todo!(),
                RitchieItemKind::Qn => todo!(),
                RitchieItemKind::Bn => todo!(),
                RitchieItemKind::Sn => todo!(),
                RitchieItemKind::Tn => todo!(),
            },
            LinketData::MajorStaticVar { path, .. } => {
                builder.macro_call(RustMacroName::StaticVarLinketImpl, |builder| {
                    path.transpile_to_rust(builder);
                    builder.punctuation(RustPunctuation::CommaSpaced);
                    builder.item_path_id_interface_cache(path);
                })
            }
            LinketData::MajorVal {
                path,
                instantiation: _,
            } => builder.macro_call(RustMacroName::ValLinketImpl, |builder| {
                path.transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::CommaSpaced);
                builder.item_path_id_interface_cache(path);
            }),
            LinketData::MethodRitchie {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::FnLinketImpl, |builder| {
                (path, instantiation).transpile_to_rust(builder);
            }),
            LinketData::EnumVariantConstructor {
                self_ty,
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::EnumVariantConstructorLinketImpl, |builder| {
                self_ty.transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::CommaSpaced);
                path.transpile_to_rust(builder);
                match path.hir_decl(db).unwrap() {
                    TypeVariantHirDecl::Props(hir_decl) => {
                        builder.punctuation(RustPunctuation::CommaSpaced);
                        builder.delimited_comma_list(
                            RustDelimiter::Par,
                            hir_decl.fields(db).iter().map(|field| field.ident()),
                        );
                    }
                    TypeVariantHirDecl::Unit(_) => (),
                    TypeVariantHirDecl::Tuple(hir_decl) => {
                        builder.punctuation(RustPunctuation::CommaSpaced);
                        builder.delimited_comma_list(
                            RustDelimiter::Par,
                            hir_decl
                                .fields(db)
                                .iter()
                                .enumerate()
                                .map(|(i, _)| TupleFieldVariable(i)),
                        );
                    }
                }
            }),
            LinketData::EnumVariantDestructor {
                self_ty,
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::EnumVariantDestructorLinketImpl, |builder| {
                self_ty.transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::CommaSpaced);
                path.transpile_to_rust(builder);
                match path.hir_decl(db).unwrap() {
                    TypeVariantHirDecl::Props(hir_decl) => {
                        builder.punctuation(RustPunctuation::CommaSpaced);
                        builder.delimited_comma_list(
                            RustDelimiter::Curl,
                            hir_decl.fields(db).iter().map(|field| field.ident()),
                        );
                    }
                    TypeVariantHirDecl::Tuple(hir_decl) => {
                        builder.punctuation(RustPunctuation::CommaSpaced);
                        builder.delimited_comma_list(
                            RustDelimiter::Par,
                            hir_decl
                                .fields(db)
                                .iter()
                                .enumerate()
                                .map(|(i, _)| TupleFieldVariable(i)),
                        );
                    }
                    TypeVariantHirDecl::Unit(_) => unreachable!(),
                }
            }),
            LinketData::EnumVariantDiscriminator {
                self_ty,
                path,
                ref instantiation,
            } => builder.macro_call(
                RustMacroName::EnumVariantDiscriminatorLinketImpl,
                |builder| {
                    self_ty.transpile_to_rust(builder);
                    builder.punctuation(RustPunctuation::CommaSpaced);
                    path.transpile_to_rust(builder);
                    match path.hir_decl(db).unwrap() {
                        TypeVariantHirDecl::Props(_) => {
                            builder.punctuation(RustPunctuation::CommaSpaced);
                            builder.curl_unit();
                        }
                        TypeVariantHirDecl::Tuple(_) => {
                            builder.punctuation(RustPunctuation::CommaSpaced);
                            builder.par_unit();
                        }
                        TypeVariantHirDecl::Unit(_) => (),
                    }
                },
            ),
            LinketData::EnumVariantField {
                path,
                ref instantiation,
                field_ty_leash_class,
                field,
            } => builder.macro_call(RustMacroName::EnumVariantFieldLinketImpl, |builder| {
                path.parent_ty_path(db).transpile_to_rust(builder);
                if !instantiation.is_empty() {
                    // todo: should we allow comptime var here?
                    builder.delimited_comma_list(
                        RustDelimiter::Angle,
                        instantiation
                            .variable_resolutions()
                            .iter()
                            .map(|(_, res)| match res {
                                LinTermVariableResolution::Explicit(arg) => arg,
                                LinTermVariableResolution::SelfLifetime
                                | LinTermVariableResolution::SelfQual(_) => unreachable!(),
                            }),
                    );
                }
                builder.punctuation(RustPunctuation::CommaSpaced);
                path.transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::CommaSpaced);
                match field {
                    LinField::Tuple { index } => builder.delimited(RustDelimiter::Par, |builder| {
                        field_ty_leash_class.transpile_to_rust(builder);
                        TupleFieldVariable(index.into()).transpile_to_rust(builder);
                    }),
                    LinField::Props { ident } => {
                        builder.delimited(RustDelimiter::Curl, |builder| {
                            field_ty_leash_class.transpile_to_rust(builder);
                            ident.transpile_to_rust(builder);
                        })
                    }
                }
            }),
            LinketData::EnumUnitToJsonValue { ty_path } => builder
                .macro_call(RustMacroName::EnumUnitPresenter, |builder| {
                    ty_path.transpile_to_rust(builder)
                }),
            LinketData::StructConstructor {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::FnLinketImpl, |builder| {
                builder.struct_ty_constructor_path(path);
                turbo_fish_instantiation(instantiation, builder);
            }),
            LinketData::StructDestructor { self_ty } => {
                builder.macro_call(RustMacroName::StructDestructorLinketImpl, |builder| {
                    self_ty.transpile_to_rust(builder);
                    builder.punctuation(RustPunctuation::CommaSpaced);
                    self_ty.ty_path(db).transpile_to_rust(builder);
                    match self_ty.ty_path(db).hir_decl(db).unwrap() {
                        TypeHirDecl::PropsStruct(hir_decl) => {
                            for field in hir_decl.fields(db) {
                                builder.punctuation(RustPunctuation::CommaSpaced);
                                field.ident().transpile_to_rust(builder)
                            }
                        }
                        TypeHirDecl::TupleStruct(hir_decl) => todo!(),
                        TypeHirDecl::Enum(_)
                        | TypeHirDecl::UnitStruct(_)
                        | TypeHirDecl::Extern(_)
                        | TypeHirDecl::Union(_) => unreachable!(),
                    }
                })
            }
            LinketData::StructField {
                self_ty,
                field_ty_leash_class,
                field,
            } => builder.macro_call(RustMacroName::StructFieldLinketImpl, |builder| {
                self_ty.transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::CommaSpaced);
                field_ty_leash_class.transpile_to_rust(builder);
                match field {
                    LinField::Tuple { index } => todo!(),
                    LinField::Props { ident } => ident.transpile_to_rust(builder),
                }
            }),
            LinketData::AssocRitchie {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::FnLinketImpl, |builder| {
                (path, instantiation).transpile_to_rust(builder)
            }),
            LinketData::UnveilAssocRitchie {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::UnveilLinketImpl, |builder| {
                (path, instantiation).transpile_to_rust(builder)
            }),
            LinketData::Memo {
                path,
                instantiation: _,
            } => builder.macro_call(RustMacroName::MemoLinketImpl, |builder| {
                path.transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::CommaSpaced);
                builder.item_path_id_interface_cache(path);
            }),
            LinketData::Index => todo!(),
            LinketData::TypeDefault { ty } => builder
                .macro_call(RustMacroName::TypeDefault, |builder| {
                    ty.transpile_to_rust(builder)
                }),
            LinketData::VecConstructor { element_ty } => {
                builder.macro_call(RustMacroName::FnLinketImpl, |builder| {
                    builder.delimited(RustDelimiter::Vert, |builder| {
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

impl<E> TranspileToRustWith<E> for (MajorFormPath, &LinInstantiation) {
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
        builder.delimited_heterogeneous_list_with(RustDelimiter::TurboFish, |builder| {
            builder.heterogeneous_comma_list_items(
                instantiation
                    .context()
                    .comptime_var_overrides()
                    .iter()
                    .map(|&(_, ovrd)| ovrd),
            );
            builder.heterogeneous_comma_list_items(
                instantiation
                    .variable_resolutions()
                    .iter()
                    .map(|&(_, res)| match res {
                        LinTermVariableResolution::Explicit(arg) => arg,
                        LinTermVariableResolution::SelfLifetime => todo!(),
                        LinTermVariableResolution::SelfQual(_) => todo!(),
                    }),
            );
        })
    }
}

impl<E> TranspileToRustWith<E> for LinComptimeVarOverride {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        match self {
            LinComptimeVarOverride::Type(ty) => ty.transpile_to_rust(builder),
        }
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
        .lin_instantiate(lin_instantiation, db);
        let ident = path.ident(db);
        builder.delimited(RustDelimiter::Angle, |builder| {
            match self_ty {
                LinType::PathLeading(self_ty) => match self_ty.ty_path(db).refine(db) {
                    Left(PreludeTypePath::VEC) => match ident.data(db) {
                        "first" | "last" => {
                            // `first` or `last` are methods from slice,
                            // so we write down Rust's slice type `[T]` instead
                            builder.delimited_comma_list(
                                RustDelimiter::Box,
                                self_ty.template_arguments(db),
                            );
                            return;
                        }
                        _ => (),
                    },
                    Left(PreludeTypePath::CYCLIC_SLICE) => {
                        builder.cyclic_slice_leashed_ty();
                        builder.delimited_comma_list(
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
                    LinTermVariableResolution::Explicit(LinTemplateArgument::Qual(_)) => {
                        todo!()
                    }
                    LinTermVariableResolution::SelfQual(place) => match place {
                        LinQual::Ref => ident.transpile_to_rust(builder),
                        LinQual::Mut => builder.method_ritchie_ident_mut(ident),
                        LinQual::Transient => todo!(),
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

impl<E> TranspileToRustWith<E> for LinketTrait {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db;
        self.trai_path(db).transpile_to_rust(builder);
        let template_arguments = self.template_arguments(db);
        if !template_arguments.is_empty() {
            builder.delimited_comma_list(RustDelimiter::Angle, template_arguments)
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
                    builder.delimited_comma_list(RustDelimiter::Angle, template_arguments)
                }
            }
        }
    }
}

impl<E> TranspileToRustWith<E> for LinTemplateArgument {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        match self {
            LinTemplateArgument::Vacant => todo!(),
            LinTemplateArgument::Type(linket_ty) => linket_ty.transpile_to_rust(builder),
            LinTemplateArgument::Constant(constant) => constant.transpile_to_rust(builder),
            LinTemplateArgument::Lifetime => todo!(),
            LinTemplateArgument::Qual(_) => todo!(),
        }
    }
}

impl<E> TranspileToRustWith<E> for LinRitchieType {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db();
        builder.keyword(RustKeyword::Fn);
        builder.delimited_comma_list(RustDelimiter::Par, self.parameters(db).iter());
        builder.punctuation(RustPunctuation::LightArrow);
        self.return_ty(db).transpile_to_rust(builder)
    }
}

impl<E> TranspileToRustWith<E> for LinRitchieParameter {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        match self.contract() {
            // ad hoc
            HirContract::Pure => (),
            // builder.punctuation(RustPunctuation::Ambersand),
            HirContract::Move => (),
            HirContract::Borrow => builder.punctuation(RustPunctuation::Ambersand),
            HirContract::BorrowMut => todo!(),
            HirContract::Compterm => todo!(),
            HirContract::Leash => todo!(),
            HirContract::At => todo!(),
        }
        self.parameter_ty().transpile_to_rust(builder)
    }
}

impl<E> TranspileToRustWith<E> for LinLeashClass {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        builder.word(self.code())
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
        builder.delimited(RustDelimiter::Angle, |builder| {
            let trait_for_type_impl_block_eth_template =
                path.impl_block(db).eth_template(db).unwrap();
            let self_ty = HirType::from_eth(trait_for_type_impl_block_eth_template.self_ty(db), db)
                .unwrap()
                .lin_instantiate(lin_instantiation, db);
            self_ty.transpile_to_rust(builder);
            builder.keyword(RustKeyword::As);
            let trai = HirTrait::from_eth(trait_for_type_impl_block_eth_template.trai(db), db);
            trai.lin_instantiate(lin_instantiation, db)
                .transpile_to_rust(builder)
        });
        builder.punctuation(RustPunctuation::ColonColon);
        path.ident(db).transpile_to_rust(builder)
    }
}
