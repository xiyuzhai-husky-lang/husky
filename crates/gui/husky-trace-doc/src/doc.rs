use crate::{view::TraceDocView, *};
#[cfg(feature = "egui")]
use egui::*;
use husky_devsoul_interface::pedestal::IsPedestal;
use husky_gui::helpers::repaint_signal::EguiRepaintSignal;

use std::{path::PathBuf, sync::Arc};

use husky_trace_protocol::{
    client::TraceClient,
    figure::{FigureUi, FigureUiCache},
    pedestal::PedestalUi,
    protocol::{IsTraceProtocol, IsTraceProtocolFull},
    view::action::TraceViewActionBuffer,
};
use notify_change::NotifyChange;
use ui::component::IsUiComponent;

/// storage, state
pub struct TraceDoc<TraceProtocol, RepaintSignal>
where
    TraceProtocol: IsTraceProtocol,
    RepaintSignal: NotifyChange,
{
    current_dir: PathBuf,
    trace_client: TraceClient<TraceProtocol, RepaintSignal>,
    action_buffer: TraceViewActionBuffer<TraceProtocol>,
    figure_ui_cache: FigureUiCache<egui::Ui>,
    // set after client is initialized
    pedestal_ui_buffer: Option<<TraceProtocol::Pedestal as IsPedestal>::UiBuffer>,
}

#[cfg(feature = "egui")]
impl<TraceProtocol, Settings, UiActionBuffer> IsUiComponent<egui::Ui, Settings, UiActionBuffer>
    for TraceDoc<TraceProtocol, EguiRepaintSignal>
where
    TraceProtocol: IsTraceProtocolFull,
    TraceProtocol::Figure: FigureUi<egui::Ui>,
    TraceProtocol::Pedestal: PedestalUi<Ui>,
    Settings: HasTraceDocSettings,
{
    fn render_dyn(
        &mut self,
        ui: &mut egui::Ui,
        settings: &mut Settings,
        _action_buffer: &mut UiActionBuffer,
    ) {
        self.trace_client.update(&mut self.pedestal_ui_buffer);
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
    TraceProtocol::Figure: FigureUi<egui::Ui>,
    TraceProtocol::Pedestal: PedestalUi<Ui>,
{
    fn render<Settings>(&mut self, ui: &mut Ui, settings: &mut Settings)
    where
        Settings: HasTraceDocSettings,
    {
        let trace_client = &self.trace_client;
        if let Some(e) = trace_client.connection_error() {
            ui.label(RichText::new(e.to_string()).color(Color32::RED));
        }
        if let Some(trace_synchrotron) = trace_client.opt_trace_synchrotron() {
            TraceDocView::new(
                &self.current_dir,
                trace_synchrotron,
                &mut self.action_buffer,
                settings,
                &mut self.figure_ui_cache,
                self.pedestal_ui_buffer.as_mut().unwrap(),
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
        _ctx: &egui::Context,
    ) -> Self {
        Self {
            current_dir: std::env::current_dir().unwrap(),
            trace_client: TraceClient::new_mock(tokio_runtime, repaint_signal),
            action_buffer: Default::default(),
            figure_ui_cache: Default::default(),
            pedestal_ui_buffer: Default::default(),
        }
    }
}
