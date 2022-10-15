use crate::*;

pub struct TermApplicationPattern<'a> {
    m: &'a TermPattern<'a>,
    n: &'a TermPattern<'a>,
}
