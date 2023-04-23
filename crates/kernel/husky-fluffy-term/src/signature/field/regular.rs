use super::*;

pub(super) fn direct_regular_field_card(
    engine: &mut impl FluffyTermEngine,
    signature: TypeDeclarativeSignatureTemplate,
    arguments: &[FluffyTerm],
    ident: Ident,
) -> Result<Option<FluffyFieldCard>, FluffySignatureError> {
    match signature {
        TypeDeclarativeSignatureTemplate::Enum(_) => todo!(),
        TypeDeclarativeSignatureTemplate::RegularStruct(signature) => {
            regular_struct_direct_regular_field_card(engine, signature, arguments, ident)
        }
        TypeDeclarativeSignatureTemplate::UnitStruct(_) => todo!(),
        TypeDeclarativeSignatureTemplate::TupleStruct(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Record(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Inductive(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Structure(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Extern(_) => todo!(),
        TypeDeclarativeSignatureTemplate::Union(_) => todo!(),
    }
}

// impl AsFluffySignatureTemplate for RegularStructTypeDeclarativeSignatureTemplate {
//     type FluffySignature = F;

//     fn fluffy_signature(self, engine: &mut impl FluffyTermEngine) -> Self::FluffySignature {
//         todo!()
//     }
// }

fn regular_struct_direct_regular_field_card(
    engine: &mut impl FluffyTermEngine,
    signature: RegularStructTypeDeclarativeSignatureTemplate,
    arguments: &[FluffyTerm],
    ident: Ident,
) -> FluffyCardResult<Option<FluffyFieldCard>> {
    let db = engine.db();
    let implicit_parameters = signature.implicit_parameters(db);
    let fields = signature.fields(db);
    if implicit_parameters.len() != arguments.len() {
        todo!()
    }
    let Some(field) = fields.iter().find(|field| field.ident() == ident) else {
        return Ok(None)
    };
    Ok(Some(todo!()))
}
