use crate::{view::TraceDocView, *};
#[cfg(feature = "egui")]
use egui::*;
use husky_gui::helpers::repaint_signal::EguiRepaintSignal;
use husky_visual_protocol::visual::image::ImageVisualData;
use std::{path::PathBuf, sync::Arc};

use husky_trace_protocol::{
    client::TraceClient,
    figure::IsFigure,
    protocol::{IsTraceProtocol, IsTraceProtocolFull},
    view::action::TraceViewActionBuffer,
};

use notify_change::NotifyChange;
use ui::{component::IsUiComponent, ui::egui::UiCache, visual_widget::VisualWidget};

/// storage, state
pub struct TraceDoc<TraceProtocol, RepaintSignal>
where
    TraceProtocol: IsTraceProtocol,
    RepaintSignal: NotifyChange,
{
    current_dir: PathBuf,
    trace_client: TraceClient<TraceProtocol, RepaintSignal>,
    action_buffer: TraceViewActionBuffer<TraceProtocol>,
    ui_cache: UiCache,
}

#[cfg(feature = "egui")]
impl<TraceProtocol, Settings, UiActionBuffer> IsUiComponent<egui::Ui, Settings, UiActionBuffer>
    for TraceDoc<TraceProtocol, EguiRepaintSignal>
where
    TraceProtocol: IsTraceProtocolFull,
    TraceProtocol::Figure: VisualWidget<egui::Ui>,
    Settings: HasTraceDocSettings,
{
    fn render_dyn(
        &mut self,
        ui: &mut egui::Ui,
        settings: &mut Settings,
        _action_buffer: &mut UiActionBuffer,
    ) {
        self.trace_client.update();
        self.render(ui, settings);
        let actions = self.action_buffer.take_actions();
        if actions.len() > 1 {
            use husky_print_utils::p;
            p!(actions);
            todo!()
        }
        for action in actions {
            match self.trace_client.take_view_action(action) {
                Ok(_) => (),
                Err(e) => println!("e = {e} while take action"),
            }
        }
    }
}

#[cfg(feature = "egui")]
impl<TraceProtocol> TraceDoc<TraceProtocol, EguiRepaintSignal>
where
    TraceProtocol: IsTraceProtocolFull,
    TraceProtocol::Figure: VisualWidget<egui::Ui>,
{
    fn render<Settings>(&mut self, ui: &mut Ui, settings: &mut Settings)
    where
        Settings: HasTraceDocSettings,
    {
        let trace_client = &self.trace_client;
        if let Some(e) = trace_client.connection_error() {
            ui.label(RichText::new(e.to_string()).color(Color32::RED));
        }
        if let Some(trace_cache) = trace_client.opt_cache() {
            TraceDocView::new(
                &self.current_dir,
                trace_cache,
                &mut self.action_buffer,
                settings,
                &mut self.ui_cache,
                ui,
            )
            .render_standard_layout(ui);
        } else {
            // todo: render connecting status
        }
    }
}

impl<TraceProtocol: IsTraceProtocolFull> TraceDoc<TraceProtocol, EguiRepaintSignal> {
    pub fn new(
        tokio_runtime: Arc<tokio::runtime::Runtime>,
        repaint_signal: EguiRepaintSignal,
        ctx: &egui::Context,
    ) -> Self {
        let color_image: ColorImage = (&ImageVisualData::Binary {
            bits_per_row: 2,
            width: 15,
            height: 4,
            bitmap: vec![1, 0, 11, 0, 31, 0, 51, 0],
        })
            .into();
        let options = TextureOptions {
            magnification: TextureFilter::Nearest,
            minification: TextureFilter::Linear,
        };
        Self {
            current_dir: std::env::current_dir().unwrap(),
            trace_client: TraceClient::new_mock(tokio_runtime, repaint_signal),
            action_buffer: Default::default(),
            ui_cache: Default::default(),
        }
    }
}
