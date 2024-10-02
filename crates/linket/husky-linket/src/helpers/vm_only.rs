use super::*;

impl Linket {
    pub fn vm_only(self, db: &::salsa::Db) -> bool {
        linket_vm_only(db, self)
    }
}

fn linket_vm_only(db: &::salsa::Db, linket: Linket) -> bool {
    match linket.data(db) {
        LinketData::MajorFunctionRitchie {
            path,
            instantiation,
        } => instantiation_vm_only(instantiation, db),
        LinketData::MajorStaticVar {
            path,
            instantiation,
        } => todo!(),
        LinketData::MajorVal {
            path,
            instantiation,
        } => todo!(),
        LinketData::Memo {
            path,
            instantiation,
        } => todo!(),
        LinketData::MethodRitchie {
            path,
            instantiation,
        } => todo!(),
        LinketData::AssocRitchie {
            path,
            instantiation,
        } => instantiation_vm_only(instantiation, db),
        LinketData::UnveilAssocRitchie {
            path,
            instantiation,
        } => todo!(),
        LinketData::StructConstructor {
            path,
            instantiation,
        } => todo!(),
        LinketData::StructDestructor { self_ty } => todo!(),
        LinketData::EnumVariantConstructor {
            self_ty,
            path,
            instantiation,
        } => todo!(),
        LinketData::EnumVariantDiscriminator {
            self_ty,
            path,
            instantiation,
        } => todo!(),
        LinketData::EnumVariantDestructor {
            self_ty,
            path,
            instantiation,
        } => todo!(),
        LinketData::StructField {
            self_ty,
            field_ty_leash_class,
            field,
        } => todo!(),
        LinketData::EnumVariantField {
            path,
            instantiation,
            field_ty_leash_class,
            field,
        } => todo!(),
        LinketData::Index => todo!(),
        LinketData::VecConstructor { element_ty } => todo!(),
        LinketData::TypeDefault { ty } => todo!(),
        LinketData::EnumUnitToJsonValue { ty_path } => todo!(),
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

fn lin_ty_vm_only(db: &::salsa::Db, ty: LinType) -> bool {
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
            if lin_ty_vm_only(db, ty) {
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
        LinQual::Ref => todo!(),
        LinQual::Mut => return true,
        LinQual::Transient => todo!(),
    }
    false
}
