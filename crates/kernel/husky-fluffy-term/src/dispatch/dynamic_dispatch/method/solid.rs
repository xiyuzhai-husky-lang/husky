mod trai_method;
mod ty_method;

pub(crate) use self::trai_method::*;
pub(crate) use self::ty_method::*;

use super::*;

impl SolidTerm {
    pub(super) fn method_dispatch_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: ExprIdx,
        ident: Ident,
        available_traits: &[TraitPath],
        mut indirections: SmallVec<[FluffyDynamicDispatchIndirection; 2]>,
    ) -> FluffyTermMaybeResult<FluffyMethodDispatch> {
        match self.data(engine) {
            SolidTermData::TypeOntology {
                path,
                refined_path,
                arguments,
            } => todo!(),
            SolidTermData::TypeOntologyAtPlace {
                place,
                path,
                refined_path,
                arguments,
                base_ty_term,
            } => match base_ty_term {
                Some(base_ty_term) => {
                    indirections.push(FluffyDynamicDispatchIndirection::Place(*place));
                    todo!()
                    // JustOk(
                    //     ethereal_ty_method_dispatch(engine, expr_idx, *base_ty_term, ident)?
                    //         .merge(indirections),
                    // )
                }
                None => todo!(),
            },
            SolidTermData::Curry { .. } | SolidTermData::Ritchie { .. } => Nothing,
            SolidTermData::SymbolAtPlace { .. } => todo!(),
        }
    }
}
