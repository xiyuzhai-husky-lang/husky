use vm::InstructionVariant;

use crate::*;

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn compile_xml_expr(&mut self, expr: Arc<XmlExpr>) {
        match expr.variant {
            XmlExprVariant::Value(ref value_expr) => {
                self.compile_eager_expr(value_expr);
                self.push_instruction(Instruction::new(
                    InstructionVariant::NewXmlFromValue { ty: value_expr.ty },
                    expr,
                ))
            }
            XmlExprVariant::Tag {
                tag_kind,
                ref props,
            } => {
                for (_, argument) in props.iter() {
                    self.compile_eager_expr(argument)
                }
                self.push_instruction(Instruction::new(
                    InstructionVariant::NewXmlFromTag {
                        name: tag_kind.as_str(),
                        props: props.keys().collect(),
                        n_child_expr: 0,
                    },
                    expr.clone(),
                ))
            }
        }
    }
}
