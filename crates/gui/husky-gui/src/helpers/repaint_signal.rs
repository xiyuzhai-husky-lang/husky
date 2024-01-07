#[derive(Clone)]
pub struct EguiRepaintSignal {
    ctx: egui::Context,
}

impl EguiRepaintSignal {
    pub fn new(ctx: egui::Context) -> Self {
        Self { ctx }
    }
}

impl notify_change::NotifyChange for EguiRepaintSignal {
    fn notify(&self) {
        self.ctx.request_repaint()
    }
}
