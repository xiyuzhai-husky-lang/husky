#![allow(warnings, non_snake_case)]
use husky_core::*;

pub enum Class<Label> {
    Known(Label),
    Unknown,
} 

pub enum OneVsAll {
    Yes,
    No,
} 

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