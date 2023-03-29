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
    db: &dyn RawTypeDb,
    path: EntityPath,
) -> VarianceResultRef<&[VarianceRepr]> {
    let _raw_term_menu = db.raw_term_menu(path.toolchain(db)).unwrap();
    match path {
        EntityPath::Module(_) => todo!(),
        EntityPath::ModuleItem(path) => match path {
            ModuleItemPath::Type(path) => ty_entity_variance_reprs(db, path),
            ModuleItemPath::Trait(path) => trai_entity_variance_reprs(db, path),
            ModuleItemPath::Form(path) => form_entity_variance_reprs(db, path),
        },
        EntityPath::AssociatedItem(path) => match path {
            AssociatedItemPath::TypeItem(path) => ty_item_entity_variance_reprs(db, path),
            AssociatedItemPath::TraitItem(_) => todo!(),
            AssociatedItemPath::TraitForTypeItem(_) => todo!(),
        },
        EntityPath::Variant(_) => todo!(),
    }
    .as_ref()
    .map(|t| t.as_ref())
}

#[salsa::tracked(jar = RawTypeJar, return_ref)]
pub(crate) fn ty_entity_variance_reprs(
    db: &dyn RawTypeDb,
    path: TypePath,
) -> VarianceResult<Vec<VarianceRepr>> {
    let _raw_term_menu = db.raw_term_menu(path.toolchain(db)).unwrap();
    let decl = match path.decl(db) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedVarianceError::DeclError.into()),
    };
    let signature = match db.ty_signature_from_decl(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    let implicit_parameters = signature.implicit_parameters(db);
    let reprs = implicit_parameters
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
        //      TypeSignature::Enum(_) => todo!(),
        //      TypeSignature::RegularStruct(_) => todo!(),
        //      TypeSignature::UnitStruct(_) => todo!(),
        //      TypeSignature::TupleStruct(_) => todo!(),
        //      TypeSignature::Record(_) => todo!(),
        //      TypeSignature::Inductive(_) => todo!(),
        //      TypeSignature::Structure(_) => todo!(),
        //      TypeSignature::Foreign(_) => (),
        //      TypeSignature::Union(_) => todo!(),
        // }
    }
    for (_repr, implicit_parameter) in std::iter::zip(reprs.iter(), implicit_parameters.iter()) {
        if let Some(_annotated_variance) = implicit_parameter.annotated_variance() {
            // verify the calculated is the same as the annotated
            // todo!()
        }
    }
    Ok(reprs)
}

#[salsa::tracked(jar = RawTypeJar, return_ref)]
pub(crate) fn trai_entity_variance_reprs(
    db: &dyn RawTypeDb,
    path: TraitPath,
) -> VarianceResult<Vec<VarianceRepr>> {
    let _raw_term_menu = db.raw_term_menu(path.toolchain(db)).unwrap();
    let decl = match db.trai_decl(path) {
        Ok(decl) => decl,
        Err(_) => return Err(DerivedVarianceError::DeclError.into()),
    };
    let signature = match db.trai_signature(decl) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    let implicit_parameters = signature.implicit_parameters(db);
    let reprs = implicit_parameters
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

#[salsa::tracked(jar = RawTypeJar, return_ref)]
pub(crate) fn form_entity_variance_reprs(
    db: &dyn RawTypeDb,
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
    let reprs = implicit_parameters
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

#[salsa::tracked(jar = RawTypeJar, return_ref)]
pub(crate) fn ty_item_entity_variance_reprs(
    db: &dyn RawTypeDb,
    path: TypeItemPath,
) -> VarianceResult<Vec<VarianceRepr>> {
    let signature = match db.ty_item_signature(path) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    let implicit_parameters = signature.implicit_parameters(db);
    let reprs = implicit_parameters
        .iter()
        .map(|parameter| VarianceRepr {
            base: parameter
                .annotated_variance()
                .unwrap_or(TY_ITEM_VARIANCE_DEFAULT),
            dependency_exprs: vec![],
            dependencies: vec![],
        })
        .collect::<Vec<_>>();
    if reprs.len() > 0 {
        // todo!()
    }
    Ok(reprs)
}
