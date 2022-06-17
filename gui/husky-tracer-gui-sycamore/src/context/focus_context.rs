use super::*;

#[derive(Debug, Default)]
pub struct FocusContext {
    pub focus: Rc<Signal<Focus>>,
    last_focus: RefCell<Focus>,
    focus_locked_store: Signal<bool>,
}
impl FocusContext {
    pub(super) fn init(&self, focus: Focus) {
        self.focus_locked_store.set(true);
        self.focus.set(focus);
    }

    pub(super) fn opt_input_id(&self) -> Option<usize> {
        return self.focus.get().opt_input_id();
    }

    pub(super) fn did_lock_focus(&mut self, focus: Focus) {
        self.focus.set(focus);
        self.focus_locked_store.set(true);
    }

    pub(super) fn focus(&self) -> Focus {
        return self.focus.cget();
    }

    pub(super) fn toggled_focus_kind(&self) -> Focus {
        let last_last_focus = self.last_focus.replace(self.focus.cget());

        if std::mem::discriminant(&last_last_focus)
            != std::mem::discriminant(&self.last_focus.borrow())
        {
            last_last_focus
        } else {
            match *self.last_focus.borrow() {
                Focus::Specific { .. } => Focus::default(),
                Focus::Generic { .. } => Focus::Specific {
                    input_id: ask_for_input_id(),
                },
            }
        }
    }
}

fn ask_for_input_id() -> usize {
    let window = web_sys::window().unwrap();
    let mut last_error: Option<String> = None;
    loop {
        let answer = match last_error {
            Some(error) => window.prompt_with_message(&format!("{:?}\ninput id = ", error)),
            None => window.prompt_with_message("input id = "),
        };
        match answer {
            Ok(Some(input_id_str)) => match input_id_str.parse::<usize>() {
                Ok(input_id) => break input_id,
                Err(e) => {
                    last_error = Some(format!("expect a valid number, but get {:?} instead", e))
                }
            },
            Ok(None) => last_error = Some(format!("expect a valid number, but get nothing")),
            Err(ref e) => last_error = Some(js_sys::JSON::stringify(e).unwrap().into()),
        }
    }
}
