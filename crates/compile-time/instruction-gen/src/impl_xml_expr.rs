use vm::InstructionVariant;

use crate::*;

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn compile_xml_expr(&mut self, expr: Arc<XmlExpr>) {
        match expr.variant {
            XmlExprVariant::Value(ref value_expr) => {
                self.compile_eager_expr(value_expr, self.sheet.variable_stack.next_stack_idx());
                self.push_instruction(Instruction::new(
                    InstructionVariant::NewXmlFromValue {
                        ty: value_expr.ty(),
                    },
                    expr,
                ))
            }
            XmlExprVariant::Tag {
                tag_kind,
                ref props,
            } => {
                for (i, (_, argument)) in props.iter().enumerate() {
                    self.compile_eager_expr(
                        argument,
                        self.sheet.variable_stack.next_stack_idx() + i,
                    )
                }
                self.push_instruction(Instruction::new(
                    InstructionVariant::NewXmlFromTag {
                        tag_kind,
                        props: props.keys().collect(),
                        n_child_expr: 0,
                    },
                    expr.clone(),
                ))
            }
        }
    }
}
