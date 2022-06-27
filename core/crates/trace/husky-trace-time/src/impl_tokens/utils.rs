use super::*;

pub(super) fn add_control_tokens(control: &ControlSnapshot, tokens: &mut Vec<TraceTokenData>) {
    match control {
        ControlSnapshot::None => (),
        ControlSnapshot::Return(ref value) => {
            tokens.push(fade!(" = "));
            tokens.push(keyword!("return"));
            tokens.push(value.eval().into());
        }
        ControlSnapshot::Break => {
            tokens.push(fade!(" = "));
            tokens.push(keyword!("break"));
        }
        ControlSnapshot::Err(ref e) => {
            tokens.push(fade!(" = "));
            tokens.push(e.clone().into());
        }
    }
}
