pub trait AstBaseVisitor {
    fn visit_stmt(&mut self);
}

pub trait TopDownVisitor {
    type Message;
}

pub trait BottomUpVisitor {}

pub enum StmtMessage<Message> {
    Return { return_expr_msg: Message },
}
