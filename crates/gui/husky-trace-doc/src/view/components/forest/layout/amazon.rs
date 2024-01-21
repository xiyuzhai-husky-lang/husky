use egui::Separator;

use super::*;

impl<'a, TraceProtocol, Settings> TraceDocView<'a, TraceProtocol, Settings>
where
    TraceProtocol: IsTraceProtocol,
    TraceProtocol::Figure: ui::visual_widget::VisualWidget<egui::Ui>,
    Settings: HasTraceDocSettings,
{
    pub(super) fn render_amazon_forest(&mut self, ui: &mut egui::Ui) {
        Frame::none().inner_margin(CONSTANT1).show(ui, |ui| {
            ui.vertical(|ui| {
                ui.style_mut().spacing.item_spacing = vec2(CONSTANT1, CONSTANT1);
                self.render_bundles(ui);
                ui.allocate_space(ui.available_size())
            })
        });
    }

    fn render_bundles(&mut self, ui: &mut egui::Ui) {
        for trace_bundle in self.trace_synchrotron.trace_id_bundles() {
            self.render_bundle(ui, trace_bundle);
        }
    }

    fn render_bundle(&mut self, ui: &mut egui::Ui, trace_bundle: &TraceIdBundle) {
        Frame::none()
            .inner_margin(0.0)
            .stroke((0.5, Color32::WHITE))
            .rounding(4.0)
            .show(ui, |ui| self.render_bundle_inner(trace_bundle, ui));
    }

    fn render_bundle_inner(&mut self, trace_bundle: &TraceIdBundle, ui: &mut egui::Ui) {
        ui.style_mut().spacing.item_spacing = vec2(0.0, 0.0);
        Frame::none().inner_margin(1.0).show(ui, |ui| {
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
        Separator::default().spacing(1.0).ui(ui);
        Frame::none().inner_margin(4.0).show(ui, |ui| {
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
        let trace_view_inner_margin: Margin = Margin {
            left: 5.0,
            right: 5.0,
            top: 0.0,
            bottom: 0.0,
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
        let mut frame = egui::Frame::none()
            .inner_margin(trace_view_inner_margin)
            .rounding(3.0);
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
                    ui.spacing_mut().item_spacing.x = 2.;
                    let accompanied = self.trace_synchrotron.accompanied(trace_id);
                    self.render_accompany_toggler(trace_id, accompanied, ui);
                    self.render_expansion_toggler(
                        trace_id,
                        entry.view_data().have_subtraces(),
                        entry.expanded(),
                        ui,
                    );
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
                    .show(ui, |ui| {
                        egui::Frame::none()
                            .inner_margin(1.)
                            .show(ui, |ui| self.render_traces(subtrace_ids, ui))
                    });
            }
            TraceKind::EagerPatternExpr => todo!(),
            TraceKind::ValItem => {
                egui::Frame::none()
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
        egui::Frame::none().inner_margin(3.0).show(ui, |ui| {
            egui::Frame::none().show(ui, |ui| {
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
}
