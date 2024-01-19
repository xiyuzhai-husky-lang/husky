use husky_trace_protocol::synchrotron::bundle::TraceIdBundle;

use super::*;

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    Settings: HasTraceViewDocSettings,
{
    pub(super) fn render_forest(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            self.render_bundles(ui);
            ui.allocate_space(ui.available_size())
        });
    }

    fn render_bundles(&mut self, ui: &mut egui::Ui) {
        for trace_bundle in self.trace_synchrotron.trace_id_bundles() {
            TopBottomPanel::top(ui.auto_id_with(trace_bundle.crate_root_module_file_abs_path()))
                .frame(
                    Frame::none()
                        .fill(Color32::from_gray(102))
                        .inner_margin(0.0),
                )
                .show_inside(ui, |ui| self.render_bundle(trace_bundle, ui));
        }
    }

    fn render_bundle(&mut self, trace_bundle: &TraceIdBundle, ui: &mut egui::Ui) {
        TopBottomPanel::top(ui.next_auto_id())
            .frame(Frame::none().inner_margin(Margin {
                left: 5.0,
                right: 0.0,
                top: 0.0,
                bottom: 0.0,
            }))
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!(
                            "{}",
                            pathdiff::diff_paths(
                                trace_bundle.crate_root_module_file_abs_path(),
                                self.current_dir
                            )
                            .unwrap()
                            .display()
                        ))
                        .color(Color32::GREEN),
                    )
                })
            });
        Frame::none().fill(Color32::from_gray(42)).show(ui, |ui| {
            self.render_traces(trace_bundle.root_trace_ids(), ui)
        });
    }

    #[cfg(feature = "egui")]
    fn render_traces(&mut self, trace_ids: &[TraceId], ui: &mut egui::Ui) {
        ui.allocate_at_least(Vec2::new(ui.available_width(), 0.), Sense::hover());
        ui.vertical(|ui| {
            for &trace_id in trace_ids {
                self.render_trace_view_tree(trace_id, ui)
            }
        });
    }

    fn render_trace_view_tree(&mut self, trace_id: TraceId, ui: &mut egui::Ui)
    where
        TraceProtocol: IsTraceProtocol,
    {
        let pedestal = self.trace_synchrotron.pedestal();
        let entry = &self.trace_synchrotron[trace_id];
        self.render_trace_view(pedestal, trace_id, entry, ui);
        if entry.expanded()
            && let Some(subtrace_ids) = entry.subtrace_ids()
        {
            self.render_subtraces(ui, entry.view_data().trace_kind, subtrace_ids);
        }
        for &associated_trace_id in entry.associated_trace_ids() {
            self.render_associated_trace(associated_trace_id, ui)
        }
    }

    fn render_trace_view(
        &mut self,
        pedestal: <TraceProtocol as IsTraceProtocol>::Pedestal,
        trace_id: TraceId,
        entry: &TraceSynchrotronEntry<TraceProtocol>,
        ui: &mut egui::Ui,
    ) where
        TraceProtocol: IsTraceProtocol,
    {
        let trace_view_inner_margin = Margin {
            left: 1.0,
            right: 1.,
            top: 1.,
            bottom: 1.,
        };
        // todo: consider multiline value representation
        let desired_space = Vec2::new(
            ui.available_width(),
            // todo: consider multiline
            trace_view_inner_margin.top
                + ui.style().spacing.interact_size.y
                    * (entry.view_data().lines_data().len() as f32)
                + trace_view_inner_margin.bottom,
        );
        let desired_rect = egui::Rect {
            min: ui.cursor().min,
            max: ui.cursor().min + desired_space,
        };
        let trace_view_response = &ui.interact(desired_rect, ui.next_auto_id(), Sense::click());
        let followed = self.trace_synchrotron.followed_trace_id() == Some(trace_id);
        // `hovered_within` tells if the pointer is within the trace view
        // we don't use hover because we don't want widgets to intercept
        let hovered_within = ui.rect_contains_pointer(trace_view_response.rect);
        if !followed {
            let follow = trace_view_response.clicked();
            if follow {
                self.add_action(TraceViewAction::FollowTrace { trace_id })
            }
        }
        let mut frame = egui::Frame::none().inner_margin(trace_view_inner_margin);
        use husky_print_utils::p;
        if hovered_within || followed {
            frame.stroke.color = if !followed {
                frame.stroke.width = 0.3;
                Color32::LIGHT_YELLOW
            } else {
                frame.stroke.width = 1.0;
                Color32::GOLD
            }
        }
        let response = frame
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.;
                    // render prefix
                    let prefix_width = self.glyph_width * 2.8;
                    if entry.view_data().have_subtraces() {
                        let button_text = match entry.expanded() {
                            true => "-",
                            false => "+",
                        };
                        let button_text = RichText::new(button_text).family(FontFamily::Monospace);
                        let button = Button::new(button_text);
                        let button_response = button.ui(ui);
                        if button_response.clicked() {
                            self.add_action(TraceViewAction::ToggleExpansion { trace_id })
                        };
                        ui.allocate_space(Vec2::new(
                            prefix_width - button_response.rect.width(),
                            button_response.rect.height(),
                        ));
                    } else {
                        ui.allocate_space(Vec2::new(prefix_width, 0.));
                    }
                    ui.vertical(|ui| {
                        let lines_data = entry.view_data().lines_data();
                        for line_data in &lines_data[..(lines_data.len() - 1)] {
                            ui.horizontal(|ui| self.render_line(line_data, trace_id, entry, ui));
                        }
                        ui.horizontal(|ui| {
                            self.render_line(lines_data.last().unwrap(), trace_id, entry, ui);
                            match entry.stalk(pedestal) {
                                TraceStalk::None => (),
                                TraceStalk::Val(value_control_flow) => match value_control_flow {
                                    ValControlFlow::Continue(value) => self.render_value(value, ui),
                                    ValControlFlow::LoopContinue => todo!(),
                                    ValControlFlow::LoopExit(_) => todo!(),
                                    ValControlFlow::Return(value) => {
                                        self.render_space_chars(1, ui);
                                        ui.label("return");
                                        self.render_value(value, ui)
                                    }
                                    ValControlFlow::Undefined => todo!(),
                                    ValControlFlow::Err(_) => todo!(),
                                },
                                TraceStalk::Vm(_) => todo!(),
                            }
                            // this is important to keep the interaction region large enough
                            let desired_size = ui.available_size() - Vec2::new(1.0, 0.0);
                            if desired_size.x > 0.0 && desired_size.y > 0.0 {
                                ui.allocate_space(desired_size);
                            }
                        });
                    })
                })
            })
            .response;
    }

    fn render_subtraces(
        &mut self,
        ui: &mut egui::Ui,
        trace_kind: TraceKind,
        subtrace_ids: &[TraceId],
    ) {
        match trace_kind {
            TraceKind::Submodule => {
                egui::Frame::none()
                    .inner_margin(Margin {
                        left: 2.0,
                        right: 0.5,
                        top: 0.5,
                        bottom: 2.0,
                    })
                    .fill(Color32::from_rgb(14, 14, 14))
                    .show(ui, |ui| {
                        egui::Frame::none()
                            .fill(Color32::from_rgb(14, 14, 14))
                            .inner_margin(1.)
                            .show(ui, |ui| self.render_traces(subtrace_ids, ui))
                    });
            }
            TraceKind::EagerPatternExpr => todo!(),
            TraceKind::ValItem => {
                egui::Frame::none()
                    .fill(Color32::BLACK)
                    .inner_margin(1.)
                    .show(ui, |ui| self.render_traces(subtrace_ids, ui));
            }
            TraceKind::LazyCall => todo!(),
            TraceKind::LazyCallInput => todo!(),
            TraceKind::LazyExpr => todo!(),
            TraceKind::LazyPatternExpr => todo!(),
            TraceKind::LazyStmt => {
                self.render_traces(subtrace_ids, ui);
            }
            TraceKind::EagerCall => todo!(),
            TraceKind::EagerExpr => todo!(),
            TraceKind::EagerStmt => {
                self.render_traces(subtrace_ids, ui);
            }
            TraceKind::EagerCallInput => todo!(),
        }
    }

    fn render_associated_trace(&mut self, associated_trace_id: TraceId, ui: &mut egui::Ui) {
        egui::Frame::none()
            .inner_margin(3.0)
            .fill(Color32::from_rgb(84, 84, 84))
            .show(ui, |ui| {
                egui::Frame::none()
                    .fill(Color32::from_rgb(44, 44, 44))
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 0.;
                        ui.allocate_space(Vec2::new(ui.available_width(), 0.));
                        self.render_trace_view_tree(associated_trace_id, ui);
                    })
            });
    }

    fn render_line(
        &mut self,
        line_data: &TraceViewLineData,
        trace_id: TraceId,
        entry: &TraceSynchrotronEntry<TraceProtocol>,
        ui: &mut egui::Ui,
    ) {
        for token_data in line_data.tokens_data() {
            self.render_token(token_data, trace_id, entry, ui);
        }
    }

    fn render_token(
        &mut self,
        token_data: &TraceViewTokenData,
        trace_id: TraceId,
        entry: &TraceSynchrotronEntry<TraceProtocol>,
        ui: &mut egui::Ui,
    ) {
        let token_foreground_colors = self
            .settings
            .code_editor_settings()
            .token_foreground_colors();
        let spaces_before = token_data.spaces_before();
        self.render_space_chars(spaces_before, ui);
        let mut label = Label::new(
            RichText::new(token_data.text())
                .family(FontFamily::Monospace)
                .color(token_foreground_colors[token_data.token_class()]),
        );
        if token_data.associated_trace_id().is_some() {
            label = label.sense(Sense::click());
        }
        let label_response = label.ui(ui);
        if let Some(associated_trace_id) = token_data.associated_trace_id() {
            if label_response.clicked() {
                self.add_action(TraceViewAction::ToggleAssociatedTrace {
                    trace_id,
                    associated_trace_id,
                })
            }
            if entry.associated_trace_ids().contains(&associated_trace_id) {
                label_response.highlight();
            } else if label_response.hovered() {
                label_response.highlight();
            }
        }
    }

    fn render_value(&self, value: &ValuePresentation, ui: &mut egui::Ui)
    where
        TraceProtocol: IsTraceProtocol,
    {
        self.render_space_chars(1, ui);
        match value {
            ValuePresentation::Unit(()) => {
                ui.label(format!("()"));
            }
            ValuePresentation::Bool(b) => {
                ui.label(format!("{}", b));
            }
            ValuePresentation::Char(c) => {
                ui.label(format!("'{}'", c));
            }
            ValuePresentation::I8(i) => {
                ui.label(format!("{}i8", i));
            }
            ValuePresentation::I16(i) => {
                ui.label(format!("{}i16", i));
            }
            ValuePresentation::I32(i) => {
                ui.label(format!("{}i32", i));
            }
            ValuePresentation::I64(i) => {
                ui.label(format!("{}i64", i));
            }
            ValuePresentation::I128(i) => {
                ui.label(format!("{}i128", i));
            }
            ValuePresentation::ISize(i) => {
                ui.label(format!("{}isize", i));
            }
            ValuePresentation::U8(u) => {
                ui.label(format!("{}u8", u));
            }
            ValuePresentation::U16(u) => {
                ui.label(format!("{}u16", u));
            }
            ValuePresentation::U32(u) => {
                ui.label(format!("{}u32", u));
            }
            ValuePresentation::U64(u) => {
                ui.label(format!("{}u64", u));
            }
            ValuePresentation::U128(u) => {
                ui.label(format!("{}u128", u));
            }
            ValuePresentation::USize(u) => {
                ui.label(format!("{}usize", u));
            }
            ValuePresentation::R8(r) => {
                ui.label(format!("{}r8", r));
            }
            ValuePresentation::R16(r) => {
                ui.label(format!("{}r16", r));
            }
            ValuePresentation::R32(r) => {
                ui.label(format!("{}r32", r));
            }
            ValuePresentation::R64(r) => {
                ui.label(format!("{}r64", r));
            }
            ValuePresentation::R128(r) => {
                ui.label(format!("{}r128", r));
            }
            ValuePresentation::RSize(r) => {
                ui.label(format!("{}rsize", r));
            }
            ValuePresentation::F32(f) => {
                ui.label(format!("{}f32", f));
            }
            ValuePresentation::F64(f) => {
                ui.label(format!("{}f64", f));
            }
            ValuePresentation::Enum => todo!(),
            ValuePresentation::Struct => todo!(),
            ValuePresentation::AdHoc(s) => {
                // ad hoc
                if s.len() > 30 {
                    ui.label(format!("{}...", &s[..30]));
                } else {
                    ui.label(s);
                }
            }
        }
    }

    /// here space means the char ` `
    fn render_space_chars(&self, n: u32, ui: &mut egui::Ui) {
        if n > 0 {
            ui.allocate_space(Vec2::new(self.glyph_width * (n as f32), 0.));
        }
    }
}
