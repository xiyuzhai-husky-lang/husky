use crate::action::TypeAction;
use egui::{Event, Key};
use std::collections::HashMap;

pub(crate) struct ChicagoTypewriterApp {
    actions: Vec<TypeAction>,
    selected: Option<usize>,
    key_bindings: HashMap<Key, UserEvent>,
}

impl Default for ChicagoTypewriterApp {
    fn default() -> Self {
        Self {
            actions: vec![TypeAction::Incr, TypeAction::Decr],
            selected: Default::default(),
            key_bindings: HashMap::from([]),
        }
    }
}

impl eframe::App for ChicagoTypewriterApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        let collect_key_user_events = |events: &[Event]| -> Vec<UserEvent> {
            let mut user_events = vec![];
            for event in events {
                match *event {
                    Event::Text(ref s) => todo!("s = {s}"),
                    Event::Key {
                        key, pressed: true, ..
                    } => {
                        if let Some(user_event) = self.key_pressed(key) {
                            user_events.push(user_event)
                        }
                    }
                    _ => (),
                }
            }
            user_events
        };
        let key_user_events = ctx.input(|input| collect_key_user_events(&input.events));
        for event in key_user_events {
            match event {
                UserEvent::SelectAction { idx } => todo!(),
            }
        }
        self.render_panels(ctx)
    }
}

impl ChicagoTypewriterApp {
    pub(crate) fn suggested_actions(&mut self) -> (&[TypeAction], &mut Option<usize>) {
        (&self.actions, &mut self.selected)
    }

    fn key_pressed(&self, key: Key) -> Option<UserEvent> {
        self.key_bindings.get(&key).copied()
    }
}

#[derive(Debug, Clone, Copy)]
enum UserEvent {
    SelectAction { idx: usize },
}
