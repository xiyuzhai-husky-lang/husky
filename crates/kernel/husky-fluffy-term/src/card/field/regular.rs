use super::*;

pub(super) fn direct_regular_field_card(
    engine: &mut impl FluffyTermEngine,
    signature: TypeSignature,
    arguments: &[FluffyTerm],
    ident: Ident,
) -> Result<Option<FluffyFieldCard>, FluffyCardError> {
    match signature {
        TypeSignature::Enum(_) => todo!(),
        TypeSignature::RegularStruct(signature) => {
            regular_struct_direct_regular_field_card(engine, signature, arguments, ident)
        }
        TypeSignature::UnitStruct(_) => todo!(),
        TypeSignature::TupleStruct(_) => todo!(),
        TypeSignature::Record(_) => todo!(),
        TypeSignature::Inductive(_) => todo!(),
        TypeSignature::Structure(_) => todo!(),
        TypeSignature::Foreign(_) => todo!(),
        TypeSignature::Union(_) => todo!(),
    }
}

fn regular_struct_direct_regular_field_card(
    engine: &mut impl FluffyTermEngine,
    signature: RegularStructTypeSignature,
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
