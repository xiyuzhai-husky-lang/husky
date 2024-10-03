use super::*;
use husky_entity_path::path::{assoc_item::AssocItemPath, major_item::form::MajorFormPath};
use husky_hir_decl::{
    decl::{
        AssocItemHirDecl, HasHirDecl, MajorFormHirDecl, TraitForTypeItemHirDecl, TraitItemHirDecl,
        TypeItemHirDecl,
    },
    parameter::{
        parenate::eager::HirEagerParenateParameter, self_value::eager::HirEagerSelfValueParameter,
    },
};

impl Linket {
    pub fn vm_only(self, db: &::salsa::Db) -> bool {
        linket_vm_only(db, self)
    }
}

fn linket_vm_only(db: &::salsa::Db, linket: Linket) -> bool {
    match *linket.data(db) {
        LinketData::MajorRitchie {
            path,
            ref instantiation,
        } => major_ritchie_vm_only(path, instantiation, db),
        LinketData::MajorStaticVar { .. } => todo!(),
        LinketData::MajorVal { .. } => todo!(),
        LinketData::Memo { .. } => todo!(),
        LinketData::MethodRitchie {
            path,
            ref instantiation,
            ..
        } => method_ritchie_vm_only(path, instantiation, db),
        LinketData::AssocRitchie {
            path,
            ref instantiation,
            ..
        } => assoc_ritchie_vm_only(path, instantiation, db),
        LinketData::UnveilAssocRitchie { .. } => todo!(),
        LinketData::StructConstructor { .. } => todo!(),
        LinketData::StructDestructor { .. } => todo!(),
        LinketData::EnumVariantConstructor { .. } => todo!(),
        LinketData::EnumVariantDiscriminator { .. } => todo!(),
        LinketData::EnumVariantDestructor { .. } => todo!(),
        LinketData::StructField { .. } => todo!(),
        LinketData::EnumVariantField { .. } => todo!(),
        LinketData::Index => todo!(),
        LinketData::VecConstructor { .. } => todo!(),
        LinketData::TypeDefault { .. } => todo!(),
        LinketData::EnumUnitToJsonValue { .. } => todo!(),
    }
}

fn major_ritchie_vm_only(
    path: MajorFormPath,
    instantiation: &LinInstantiation,
    db: &salsa::Db,
) -> bool {
    let Some(MajorFormHirDecl::Ritchie(hir_decl)) = path.hir_decl(db) else {
        unreachable!()
    };
    for _ in hir_decl.template_parameters(db) {
        todo!()
    }
    let return_ty = LinType::from_hir(hir_decl.return_ty(db), instantiation, db);
    lin_ty_vm_only(return_ty, db)
}

fn method_ritchie_vm_only(
    path: AssocItemPath,
    instantiation: &LinInstantiation,
    db: &salsa::Db,
) -> bool {
    match path.hir_decl(db).unwrap() {
        AssocItemHirDecl::TypeItem(hir_decl) => match hir_decl {
            TypeItemHirDecl::MethodFn(hir_decl) => {
                if self_value_parameter_vm_only(
                    hir_decl.self_value_parameter(db),
                    instantiation,
                    db,
                ) {
                    return true;
                }
                if parenate_parameters_vm_only(hir_decl.parenate_parameters(db), instantiation, db)
                {
                    return true;
                }
                let return_ty = LinType::from_hir(hir_decl.return_ty(db), instantiation, db);
                lin_ty_vm_only(return_ty, db)
            }
            _ => unreachable!(),
        },
        AssocItemHirDecl::TraitItem(hir_decl) => match hir_decl {
            TraitItemHirDecl::MethodFn(hir_decl) => {
                if self_value_parameter_vm_only(
                    hir_decl.self_value_parameter(db),
                    instantiation,
                    db,
                ) {
                    return true;
                }
                if parenate_parameters_vm_only(hir_decl.parenate_parameters(db), instantiation, db)
                {
                    return true;
                }
                let return_ty = LinType::from_hir(hir_decl.return_ty(db), instantiation, db);
                lin_ty_vm_only(return_ty, db)
            }
            _ => unreachable!(),
        },
        AssocItemHirDecl::TraitForTypeItem(hir_decl) => match hir_decl {
            TraitForTypeItemHirDecl::MethodFn(hir_decl) => {
                if self_value_parameter_vm_only(
                    hir_decl.self_value_parameter(db),
                    instantiation,
                    db,
                ) {
                    return true;
                }
                if parenate_parameters_vm_only(hir_decl.parenate_parameters(db), instantiation, db)
                {
                    return true;
                }
                let return_ty = LinType::from_hir(hir_decl.return_ty(db), instantiation, db);
                lin_ty_vm_only(return_ty, db)
            }
            _ => unreachable!(),
        },
    }
}

fn assoc_ritchie_vm_only(
    path: AssocItemPath,
    instantiation: &LinInstantiation,
    db: &salsa::Db,
) -> bool {
    match path.hir_decl(db).unwrap() {
        AssocItemHirDecl::TypeItem(hir_decl) => match hir_decl {
            TypeItemHirDecl::AssocRitchie(hir_decl) => {
                if parenate_parameters_vm_only(hir_decl.parenate_parameters(db), instantiation, db)
                {
                    return true;
                }
                let return_ty = LinType::from_hir(hir_decl.return_ty(db), instantiation, db);
                lin_ty_vm_only(return_ty, db)
            }
            _ => unreachable!(),
        },
        AssocItemHirDecl::TraitItem(hir_decl) => match hir_decl {
            TraitItemHirDecl::AssocRitchie(hir_decl) => {
                if parenate_parameters_vm_only(hir_decl.parenate_parameters(db), instantiation, db)
                {
                    return true;
                }
                let return_ty = LinType::from_hir(hir_decl.return_ty(db), instantiation, db);
                lin_ty_vm_only(return_ty, db)
            }
            _ => unreachable!(),
        },
        AssocItemHirDecl::TraitForTypeItem(hir_decl) => match hir_decl {
            TraitForTypeItemHirDecl::AssocRitchie(hir_decl) => {
                if parenate_parameters_vm_only(hir_decl.parenate_parameters(db), instantiation, db)
                {
                    return true;
                }
                let return_ty = LinType::from_hir(hir_decl.return_ty(db), instantiation, db);
                lin_ty_vm_only(return_ty, db)
            }
            _ => unreachable!(),
        },
    }
}

fn instantiation_vm_only(instantiation: &LinInstantiation, db: &salsa::Db) -> bool {
    for &(_, res) in instantiation.variable_resolutions() {
        if match res {
            LinTermVariableResolution::Explicit(arg) => template_argument_vm_only(arg, db),
            LinTermVariableResolution::SelfLifetime => false,
            LinTermVariableResolution::SelfQual(qual) => qual_vm_only(qual),
        } {
            return true;
        }
    }
    false
}

fn parenate_parameters_vm_only(
    parameters: &[HirEagerParenateParameter],
    instantiation: &LinInstantiation,
    db: &::salsa::Db,
) -> bool {
    for parameter in parameters {
        if parenate_parameter_vm_only(parameter, instantiation, db) {
            return true;
        }
    }
    false
}

fn parenate_parameter_vm_only(
    parameter: &HirEagerParenateParameter,
    instantiation: &LinInstantiation,
    db: &::salsa::Db,
) -> bool {
    match *parameter {
        HirEagerParenateParameter::Simple {
            pattern_idx,
            contract,
            ty,
        } => lin_ty_vm_only(LinType::from_hir(ty, instantiation, db), db), // could relax this
        HirEagerParenateParameter::Keyed => todo!(),
        HirEagerParenateParameter::Variadic => todo!(),
    }
}

fn self_value_parameter_vm_only(
    parameter: HirEagerSelfValueParameter,
    instantiation: &LinInstantiation,
    db: &::salsa::Db,
) -> bool {
    lin_ty_vm_only(LinType::from_hir(parameter.self_ty, instantiation, db), db)
}

// TODO: cache this
fn lin_ty_vm_only(ty: LinType, db: &::salsa::Db) -> bool {
    match ty {
        LinType::PathLeading(ty) => {
            for &arg in ty.template_arguments(db) {
                if template_argument_vm_only(arg, db) {
                    return true;
                }
            }
            false
        }
        LinType::Ritchie(ty) => false,
    }
}

fn template_argument_vm_only(arg: LinTemplateArgument, db: &salsa::Db) -> bool {
    match arg {
        LinTemplateArgument::Vacant => (),
        LinTemplateArgument::Type(ty) => {
            if lin_ty_vm_only(ty, db) {
                return true;
            }
        }
        LinTemplateArgument::Constant(_) => (),
        LinTemplateArgument::Lifetime => (),
        LinTemplateArgument::Qual(qual) => {
            if qual_vm_only(qual) {
                return true;
            }
        }
    }
    false
}

fn qual_vm_only(qual: LinQual) -> bool {
    match qual {
        LinQual::Ref => true,
        LinQual::Mut => true,
        LinQual::Transient => false,
    }
}
