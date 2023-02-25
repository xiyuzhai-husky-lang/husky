use super::default::*;
use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VarianceRepr {
    base: Variance,
    dependency_exprs: Vec<VarianceExpr>,
    dependencies: Vec<VarianceId>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum VarianceExpr {
    Atom(VarianceId),
    Compose(VarianceId, Vec<VarianceExpr>),
}

impl VarianceRepr {
    pub(crate) fn base(&self) -> Variance {
        self.base
    }

    pub(crate) fn dependencies(&self) -> &[VarianceId] {
        self.dependencies.as_ref()
    }
}

pub(crate) fn entity_variance_reprs(
    db: &dyn TypeDb,
    path: EntityPath,
) -> VarianceResultRef<&[VarianceRepr]> {
    let term_menu = db.term_menu(path.toolchain(db)).as_ref().unwrap();
    match path {
        EntityPath::Module(_) => todo!(),
        EntityPath::ModuleItem(path) => match path {
            ModuleItemPath::Type(path) => ty_entity_variance_reprs(db, path)
                .as_ref()
                .map(|t| t.as_ref()),
            ModuleItemPath::Trait(path) => trai_entity_variance_reprs(db, path)
                .as_ref()
                .map(|t| t.as_ref()),
            ModuleItemPath::Form(path) => form_entity_variance_reprs(db, path)
                .as_ref()
                .map(|t| t.as_ref()),
        },
        EntityPath::AssociatedItem(_) => todo!(),
        EntityPath::Variant(_) => todo!(),
    }
}

#[salsa::tracked(jar = TypeJar, return_ref)]
pub(crate) fn ty_entity_variance_reprs(
    db: &dyn TypeDb,
    path: TypePath,
) -> VarianceResult<Vec<VarianceRepr>> {
    let term_menu = db.term_menu(path.toolchain(db)).as_ref().unwrap();
    let decl = match db.ty_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedVarianceError::DeclError.into()),
    };
    let signature = match db.ty_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    let implicit_parameters = signature.implicit_parameters(db);
    let mut reprs = implicit_parameters
        .iter()
        .map(|implicit_parameter| VarianceRepr {
            base: implicit_parameter
                .annotated_variance()
                .unwrap_or(TYPE_VARIANCE_DEFAULT),
            dependency_exprs: vec![],
            dependencies: vec![],
        })
        .collect::<Vec<_>>();
    if reprs.len() > 0 {
        // ad hoc: todo
        // match signature {
        //     TypeSignature::Enum(_) => todo!(),
        //     TypeSignature::RegularStruct(_) => todo!(),
        //     TypeSignature::UnitStruct(_) => todo!(),
        //     TypeSignature::TupleStruct(_) => todo!(),
        //     TypeSignature::Record(_) => todo!(),
        //     TypeSignature::Inductive(_) => todo!(),
        //     TypeSignature::Structure(_) => todo!(),
        //     TypeSignature::Foreign(_) => (),
        //     TypeSignature::Union(_) => todo!(),
        // }
    }
    for (repr, implicit_parameter) in std::iter::zip(reprs.iter(), implicit_parameters.iter()) {
        if let Some(annotated_variance) = implicit_parameter.annotated_variance() {
            // verify the calculated is the same as the annotated
            // todo!()
        }
    }
    Ok(reprs)
}

#[salsa::tracked(jar = TypeJar, return_ref)]
pub(crate) fn trai_entity_variance_reprs(
    db: &dyn TypeDb,
    path: TraitPath,
) -> VarianceResult<Vec<VarianceRepr>> {
    let term_menu = db.term_menu(path.toolchain(db)).as_ref().unwrap();
    let decl = match db.trai_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedVarianceError::DeclError.into()),
    };
    let signature = match db.trai_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    let implicit_parameters = signature.implicit_parameters(db);
    let mut reprs = implicit_parameters
        .iter()
        .map(|parameter| VarianceRepr {
            base: parameter
                .annotated_variance()
                .unwrap_or(TRAIT_VARIANCE_DEFAULT),
            dependency_exprs: vec![],
            dependencies: vec![],
        })
        .collect::<Vec<_>>();
    if reprs.len() > 0 {
        // todo!()
    }
    Ok(reprs)
}

#[salsa::tracked(jar = TypeJar, return_ref)]
pub(crate) fn form_entity_variance_reprs(
    db: &dyn TypeDb,
    path: FormPath,
) -> VarianceResult<Vec<VarianceRepr>> {
    let decl = match db.form_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedVarianceError::DeclError.into()),
    };
    let signature = match db.form_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    let implicit_parameters = signature.implicit_parameters(db);
    let mut reprs = implicit_parameters
        .iter()
        .map(|parameter| VarianceRepr {
            base: parameter
                .annotated_variance()
                .unwrap_or(FORM_VARIANCE_DEFAULT),
            dependency_exprs: vec![],
            dependencies: vec![],
        })
        .collect::<Vec<_>>();
    if reprs.len() > 0 {
        // todo!()
    }
    Ok(reprs)
}
