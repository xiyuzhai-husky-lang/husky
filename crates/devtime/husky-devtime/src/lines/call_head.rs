use husky_coword::Ident;

use super::*;

impl<'a> TraceLineGenerator<'a> {
    pub(super) fn gen_call_head_lines(&mut self, item: &EntityDefn) {
        match item.variant {
            EntityDefnVariant::Func { ref parameters, .. } => self.gen_call_head_lines_aux(
                &self.runtime().text(item.file).unwrap(),
                "func ",
                item.ident,
                parameters,
            ),
            EntityDefnVariant::Proc { ref parameters, .. } => self.gen_call_head_lines_aux(
                &self.runtime().text(item.file).unwrap(),
                "proc ",
                item.ident,
                parameters,
            ),
            EntityDefnVariant::Module { .. } => todo!(),
            EntityDefnVariant::Feature { .. } => todo!(),
            EntityDefnVariant::Function { .. } => todo!(),
            EntityDefnVariant::Method { ref parameters, .. } => self.gen_call_head_lines_aux(
                &self.runtime().text(item.file).unwrap(),
                "func ",
                item.ident,
                parameters,
            ),
            EntityDefnVariant::EtherealTerm { .. } => todo!(),
            EntityDefnVariant::Trait { .. } => todo!(),
            EntityDefnVariant::EnumVariant { .. } => todo!(),
            EntityDefnVariant::Builtin => todo!(),
            EntityDefnVariant::TyField { .. } => todo!(),
            EntityDefnVariant::TraitAssociatedTypeImpl { .. } => todo!(),
            EntityDefnVariant::TraitAssociatedConstSizeImpl { .. } => todo!(),
            EntityDefnVariant::TargetInput { .. } => todo!(),
            EntityDefnVariant::Any => todo!(),
        };
    }
    pub(super) fn gen_call_head_lines_aux(
        &mut self,
        _text: &HuskyText,
        _routine_keyword: &'static str,
        _ident: Ident,
        _parameters: &[Parameter],
    ) {
        todo!()
        // self.render_keyword_token(routine_keyword, None, None);
        // self.render_ident_token(ident.as_str(), None, None);
        // self.render_special_token("(", None, None);
        // for i in 0..parameters.len() {
        //     let parameter = &parameters[i];
        //     match parameter.contract() {
        //         ParameterModifier::None => (),
        //         ParameterModifier::Owned => todo!(),
        //         ParameterModifier::OwnedMut => todo!(),
        //         ParameterModifier::MemberAccess => todo!(),
        //         ParameterModifier::Leash => todo!(),
        //         ParameterModifier::TempRef => todo!(),
        //         ParameterModifier::TempRefMut => todo!(),
        //     }
        //     self.render_ident_token(parameter.ident().as_str(), None, None);
        //     self.render_special_token(": ", None, None);
        //     self.gen_route_token(text.ranged(parameter.declarative_ty_range()), None, None);
        //     if i < parameters.len() - 1 {
        //         self.render_special_token(", ", None, None);
        //     }
        // }
        // self.render_special_token("):", None, None);
    }
}
