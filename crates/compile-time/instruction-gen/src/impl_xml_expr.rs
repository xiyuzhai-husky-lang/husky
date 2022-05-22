use vm::InstructionKind;

use crate::*;

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn compile_xml_expr(&mut self, expr: Arc<XmlExpr>) {
        for (_, argument) in expr.props.iter() {
            self.compile_eager_expr(argument)
        }
        self.push_instruction(Instruction::new(
            InstructionKind::NewXml {
                name: expr.kind.as_str(),
                props: expr.props.keys().collect(),
                n_child_expr: 0,
            },
            expr.clone(),
        ))
    }
}
