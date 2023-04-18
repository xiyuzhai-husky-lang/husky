use super::*;

pub(super) fn direct_regular_field_card(
    engine: &mut impl FluffyTermEngine,
    signature: TypeDeclarativeSignature,
    arguments: &[FluffyTerm],
    ident: Ident,
) -> Result<Option<FluffyFieldCard>, FluffyCardError> {
    match signature {
        TypeDeclarativeSignature::Enum(_) => todo!(),
        TypeDeclarativeSignature::RegularStruct(signature) => {
            regular_struct_direct_regular_field_card(engine, signature, arguments, ident)
        }
        TypeDeclarativeSignature::UnitStruct(_) => todo!(),
        TypeDeclarativeSignature::TupleStruct(_) => todo!(),
        TypeDeclarativeSignature::Record(_) => todo!(),
        TypeDeclarativeSignature::Inductive(_) => todo!(),
        TypeDeclarativeSignature::Structure(_) => todo!(),
        TypeDeclarativeSignature::Extern(_) => todo!(),
        TypeDeclarativeSignature::Union(_) => todo!(),
    }
}

fn regular_struct_direct_regular_field_card(
    engine: &mut impl FluffyTermEngine,
    signature: RegularStructTypeDeclarativeSignature,
    arguments: &[FluffyTerm],
    ident: Ident,
) -> FluffyCardResult<Option<FluffyFieldCard>> {
    let db = engine.db();
    let implicit_parameters = signature.implicit_parameters(db);
    let fields = signature.fields(db);
    if implicit_parameters.len() != arguments.len() {
        todo!()
    }
    let Some(field) = fields.iter().find(|field|field.ident()==ident) else {
        return Ok(None)
    };
    todo!()
}
