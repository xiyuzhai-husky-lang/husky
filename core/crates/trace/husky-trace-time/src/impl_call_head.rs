use super::*;
use crate::*;
use defn_head::Parameter;
use semantics_entity::*;
use word::{CustomIdentifier, Identifier};

impl HuskyTraceTime {
    pub fn new_call_head(&mut self, entity: Arc<EntityDefn>) -> TraceId {
        let tokens = match entity.variant {
            EntityDefnVariant::Func { ref parameters, .. } => routine_call_head_tokens(
                &self.runtime.compile_time().text(entity.file).unwrap(),
                "func ",
                entity.ident,
                parameters,
            ),
            EntityDefnVariant::Proc {
                parameters: ref parameters,
                ..
            } => routine_call_head_tokens(
                &self.runtime.compile_time().text(entity.file).unwrap(),
                "proc ",
                entity.ident,
                parameters,
            ),
            _ => todo!(),
        };
        return self.new_trace(None, 0, TraceVariant::CallHead { entity, tokens });

        fn routine_call_head_tokens<'eval>(
            text: &Text,
            routine_keyword: &'static str,
            ident: Identifier,
            parameters: &[Parameter],
        ) -> Vec<TraceTokenData> {
            let mut tokens = vec![
                keyword!(routine_keyword),
                ident!(ident.as_str()),
                special!("("),
            ];
            for i in 0..parameters.len() {
                let input_placeholder = &parameters[i];
                tokens.push(ident!(input_placeholder.ranged_ident.ident.as_str()));
                tokens.push(special!(": "));
                tokens.push(route!(text.ranged(input_placeholder.ranged_ty.range)));
                if i < parameters.len() - 1 {
                    tokens.push(special!(", "));
                }
            }
            tokens.push(special!("):"));
            tokens
        }
    }
}
