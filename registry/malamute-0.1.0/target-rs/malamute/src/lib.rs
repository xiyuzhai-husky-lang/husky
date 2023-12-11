#![allow(warnings, non_snake_case)]
    use husky_core::*;
    
    

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Class<Label> {
    Known(Label),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum OneVsAll {
    Yes,
    No,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum OneVsAllResult {
    ConfidentYes,
    ConfidentNo,
    Unconfident,
}

impl Default for crate::OneVsAll {
    fn default() -> crate::OneVsAll {
        OneVsAll::No
    }
}

impl <Label, >Unveil<crate::OneVsAll> for crate::Class<Label> {
    
    type Output = ();
}

impl Unveil<crate::OneVsAllResult> for crate::OneVsAll {
    
    type Output = ();
}