use husky_tex::eval::Tracer;
use husky_tex::layout::TexFrame;
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::Arc;

pub fn spawn_compilation_thread(
    egui_ctx: egui::Context,
) -> (SyncSender<String>, Receiver<Result<TexFrame, String>>) {
    let sandbox = Arc::new(crate::sandbox::Sandbox::new());

    let (in_send, in_recv) = std::sync::mpsc::sync_channel(4);
    let (out_send, out_recv) = std::sync::mpsc::sync_channel(4);

    std::thread::spawn(move || {
        while let Ok(input) = in_recv.recv() {
            let mut tracer = Tracer::new();
            let compiled =
                husky_tex::compile(&Arc::clone(&sandbox).with_source(input), &mut tracer)
                    .map_err(|errors| format!("{errors:#?}"))
                    .and_then(|doc| {
                        doc.pages
                            .into_iter()
                            .next()
                            .ok_or_else(|| r#"error"#.into())
                            .map(|page| page.frame)
                    });
            _ = out_send.send(compiled);
            egui_ctx.request_repaint();
        }
    });

    (in_send, out_recv)
}
