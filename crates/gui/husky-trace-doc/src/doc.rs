use crate::{view::TraceDocView, *};
#[cfg(feature = "egui")]
use egui::*;
use hotkey::TraceDocHotkeyAction;
use husky_gui::helpers::repaint_signal::EguiRepaintSignal;

use std::{path::PathBuf, sync::Arc};

use husky_trace_protocol::{
    caryatid::CaryatidUi,
    client::TraceClient,
    figure::{FigureUi, FigureUiCache},
    protocol::{IsTraceProtocol, IsTraceProtocolFull},
    view::action::TraceViewActionBuffer,
};
use notify_change::NotifyChange;
use ui::{
    component::IsUiComponent,
    hotkey::egui::{HotkeyBuffer, HotkeyMap},
};

/// storage, state
pub struct TraceDoc<TraceProtocol, RepaintSignal>
where
    TraceProtocol: IsTraceProtocol,
    RepaintSignal: NotifyChange,
{
    current_dir: PathBuf,
    trace_client: TraceClient<TraceProtocol, RepaintSignal>,
    view_action_buffer: TraceViewActionBuffer<TraceProtocol>,
    figure_ui_cache: FigureUiCache<egui::Ui>,
    // set after client is initialized
    caryatid_ui_buffer: Option<<TraceProtocol::Caryatid as IsCaryatid>::UiBuffer>,
    hotkey_map: HotkeyMap<TraceDocHotkeyAction>,
}

#[cfg(feature = "egui")]
impl<TraceProtocol, ParentSettings, ParentActionBuffer>
    IsUiComponent<egui::Ui, ParentSettings, ParentActionBuffer>
    for TraceDoc<TraceProtocol, EguiRepaintSignal>
where
    TraceProtocol: IsTraceProtocolFull,
    TraceProtocol::Figure: FigureUi<egui::Ui>,
    TraceProtocol::Caryatid: CaryatidUi<Ui>,
    ParentSettings: HasTraceDocSettings,
{
    fn render(
        &mut self,
        parent_settings: &mut ParentSettings,
        hotkey_buffer: &mut HotkeyBuffer,
        parent_action_buffer: &mut ParentActionBuffer,
        ui: &mut egui::Ui,
    ) {
        self.trace_client.update(&mut self.caryatid_ui_buffer);
        if let Some((number, hotkey_action)) = hotkey_buffer.extract(&self.hotkey_map) {
            if let Some(view_action) =
                hotkey_action.view_action(number, self.trace_client.opt_trace_synchrotron())
            {
                self.view_action_buffer.push(view_action)
            } else {
                // todo: report invalid hotkey press
            }
        }
        self.render_inner(ui, parent_settings);
        let actions = self.view_action_buffer.take_actions();
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
    TraceProtocol::Caryatid: CaryatidUi<Ui>,
{
    fn render_inner<Settings>(&mut self, ui: &mut Ui, settings: &mut Settings)
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
                &mut self.view_action_buffer,
                settings,
                &mut self.figure_ui_cache,
                self.caryatid_ui_buffer.as_mut().unwrap(),
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
            view_action_buffer: Default::default(),
            figure_ui_cache: Default::default(),
            caryatid_ui_buffer: Default::default(),
            hotkey_map: HotkeyMap::new([(
                "Alt+F",
                TraceDocHotkeyAction::FillCaryatidWithTraceVarDeps,
            )])
            .unwrap(),
        }
    }
}
