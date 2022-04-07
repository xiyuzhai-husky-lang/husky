use entity_route::InputPlaceholder;
use word::{CustomIdentifier, Identifier};

use super::*;
use crate::*;

impl<'eval> TraceFactory<'eval> {
    pub fn new_call_head(&self, entity: Arc<EntityDefn>, text: &Text) -> Arc<Trace<'eval>> {
        let tokens = match entity.kind() {
            EntityDefnKind::Module { .. } => todo!(),
            EntityDefnKind::Feature { .. } => todo!(),
            EntityDefnKind::Pattern { .. } => todo!(),
            EntityDefnKind::Func {
                input_placeholders, ..
            } => routine_call_head_tokens("func ", entity.ident, input_placeholders, text),
            EntityDefnKind::Proc {
                input_placeholders, ..
            } => routine_call_head_tokens("proc ", entity.ident, input_placeholders, text),
            EntityDefnKind::Ty(_) => todo!(),
            EntityDefnKind::Main(_) => todo!(),
            EntityDefnKind::Builtin => todo!(),
            EntityDefnKind::EnumVariant(_) => todo!(),
        };
        return self.new_trace(None, 0, TraceKind::CallHead { entity, tokens }, text);

        fn routine_call_head_tokens<'eval>(
            routine_keyword: &'static str,
            ident: Identifier,
            input_placeholders: &[InputPlaceholder],
            text: &Text,
        ) -> Vec<TokenProps<'eval>> {
            let mut tokens = vec![
                keyword!(routine_keyword),
                ident!(ident.as_str()),
                special!("("),
            ];
            for i in 0..input_placeholders.len() {
                let input_placeholder = &input_placeholders[i];
                tokens.push(ident!(input_placeholder.ident.as_str()));
                tokens.push(special!(": "));
                tokens.push(scope!(text.ranged(input_placeholder.ranged_ty.range)));
                if i < input_placeholders.len() - 1 {
                    tokens.push(special!(", "));
                }
            }
            tokens.push(special!("):"));
            tokens
        }
    }
}
