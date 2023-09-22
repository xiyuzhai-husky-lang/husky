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

pub(crate) fn item_variance_reprs(
    db: &dyn DeclarativeTypeDb,
    path: ItemPath,
) -> VarianceResultRef<&[VarianceRepr]> {
    let _declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    match path {
        ItemPath::Submodule(_) => todo!(),
        ItemPath::MajorItem(path) => match path {
            MajorItemPath::Type(path) => ty_template_parameter_variance_reprs(db, path),
            MajorItemPath::Trait(path) => trai_item_variance_reprs(db, path),
            MajorItemPath::Fugitive(path) => form_item_variance_reprs(db, path),
        },
        ItemPath::AssociatedItem(path) => match path {
            AssociatedItemPath::TypeItem(path) => ty_item_item_variance_reprs(db, path),
            AssociatedItemPath::TraitItem(_) => todo!(),
            AssociatedItemPath::TraitForTypeItem(_) => todo!(),
        },
        ItemPath::TypeVariant(_) => todo!(),
        ItemPath::ImplBlock(_) => todo!(),
        ItemPath::Attr(_) => todo!(),
    }
    .as_ref()
    .map(|t| t.as_ref())
}

#[salsa::tracked(jar = DeclarativeTypeJar, return_ref)]
pub(crate) fn ty_template_parameter_variance_reprs(
    db: &dyn DeclarativeTypeDb,
    path: TypePath,
) -> VarianceResult<Vec<VarianceRepr>> {
    let _declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    let signature = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    let template_parameters = signature.template_parameters(db);
    let reprs = template_parameters
        .iter()
        .map(|template_parameter| VarianceRepr {
            base: template_parameter
                .annotated_variance()
                .unwrap_or(TYPE_VARIANCE_DEFAULT),
            dependency_exprs: vec![],
            dependencies: vec![],
        })
        .collect::<Vec<_>>();
    if reprs.len() > 0 {
        // ad hoc: todo
        // match signature {
        //      TypeDeclarativeSignatureTemplate::Enum(_) => todo!(),
        //      TypeDeclarativeSignatureTemplate::PropsStruct(_) => todo!(),
        //      TypeDeclarativeSignatureTemplate::UnitStruct(_) => todo!(),
        //      TypeDeclarativeSignatureTemplate::TupleStruct(_) => todo!(),
        //      TypeDeclarativeSignatureTemplate::Record(_) => todo!(),
        //      TypeDeclarativeSignatureTemplate::Inductive(_) => todo!(),
        //      TypeDeclarativeSignatureTemplate::Structure(_) => todo!(),
        //      TypeDeclarativeSignatureTemplate::Foreign(_) => (),
        //      TypeDeclarativeSignatureTemplate::Union(_) => todo!(),
        // }
    }
    for (_repr, template_parameter) in std::iter::zip(reprs.iter(), template_parameters.iter()) {
        if let Some(_annotated_variance) = template_parameter.annotated_variance() {
            // verify the calculated is the same as the annotated
            // todo!()
        }
    }
    Ok(reprs)
}

#[salsa::tracked(jar = DeclarativeTypeJar, return_ref)]
pub(crate) fn trai_item_variance_reprs(
    db: &dyn DeclarativeTypeDb,
    path: TraitPath,
) -> VarianceResult<Vec<VarianceRepr>> {
    let _declarative_term_menu = db.declarative_term_menu(path.toolchain(db)).unwrap();
    let signature = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    let template_parameters = signature.template_parameters(db);
    let reprs = template_parameters
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

#[salsa::tracked(jar = DeclarativeTypeJar, return_ref)]
pub(crate) fn form_item_variance_reprs(
    db: &dyn DeclarativeTypeDb,
    path: FugitivePath,
) -> VarianceResult<Vec<VarianceRepr>> {
    let signature = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    let template_parameters = signature.template_parameters(db);
    let reprs = template_parameters
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

#[salsa::tracked(jar = DeclarativeTypeJar, return_ref)]
pub(crate) fn ty_item_item_variance_reprs(
    db: &dyn DeclarativeTypeDb,
    path: TypeItemPath,
) -> VarianceResult<Vec<VarianceRepr>> {
    let signature = match path.declarative_signature_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedVarianceError::SignatureError.into()),
    };
    let template_parameters = signature.template_parameters(db);
    let reprs = template_parameters
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
