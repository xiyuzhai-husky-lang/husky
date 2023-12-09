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

impl Default for OneVsAll {
    fn default() -> OneVsAll {
        OneVsAll::No
    }
}

impl <Label, >Unveil<OneVsAll> for Class<Label> {
    
    type Output = ();
}

impl Unveil<OneVsAllResult> for OneVsAll {
    
    type Output = ();
}