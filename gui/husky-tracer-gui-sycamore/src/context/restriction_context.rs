use super::*;

#[derive(Debug)]
pub struct RestrictionContext {
    pub restriction: &'static Signal<Restriction>,
    last_restriction: RefCell<Restriction>,
    restriction_locked_store: Signal<bool>,
}
impl RestrictionContext {
    pub(super) fn new<'a>(scope: Scope<'a>) -> Self {
        Self {
            restriction: create_static_signal(scope, Restriction::default()),
            last_restriction: Default::default(),
            restriction_locked_store: Default::default(),
        }
    }

    pub(super) fn init(&self, restriction: Restriction) {
        self.restriction_locked_store.set(true);
        self.restriction.set(restriction);
    }

    pub(super) fn opt_sample_id(&self) -> Option<SampleId> {
        self.restriction.get().opt_sample_id()
    }

    pub(super) fn did_lock_restriction(&mut self, restriction: Restriction) {
        self.restriction.set(restriction);
        self.restriction_locked_store.set(true);
    }

    pub(super) fn restriction(&self) -> Restriction {
        return self.restriction.cget();
    }

    pub(super) fn toggled_restriction_kind(&self) -> Restriction {
        let last_last_restriction = self.last_restriction.replace(self.restriction.cget());

        if std::mem::discriminant(&last_last_restriction)
            != std::mem::discriminant(&self.last_restriction.borrow(file!(), line!()))
        {
            last_last_restriction
        } else {
            match *self.last_restriction.borrow(file!(), line!()) {
                Restriction::Specific { .. } => Restriction::default(),
                Restriction::Generic { .. } => Restriction::Specific {
                    sample_id: ask_for_sample_id(),
                },
            }
        }
    }
}

fn ask_for_sample_id() -> SampleId {
    let window = web_sys::window().unwrap();
    let mut last_error: Option<String> = None;
    loop {
        let answer = match last_error {
            Some(error) => window.prompt_with_message(&format!("{:?}\ninput id = ", error)),
            None => window.prompt_with_message("input id = "),
        };
        match answer {
            Ok(Some(sample_id_str)) => match sample_id_str.parse::<usize>() {
                Ok(raw) => break SampleId(raw),
                Err(e) => {
                    last_error = Some(format!("expect a valid number, but get {:?} instead", e))
                }
            },
            Ok(None) => last_error = Some(format!("expect a valid number, but get nothing")),
            Err(ref e) => last_error = Some(unsafe { js_sys::JSON::stringify(e) }.unwrap().into()),
        }
    }
}
