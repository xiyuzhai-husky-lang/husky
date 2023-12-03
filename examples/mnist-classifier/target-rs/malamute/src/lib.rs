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

impl <Label, >Unveil<OneVsAll> for Class<Label> {
    
    type Output = ();
}

impl Unveil<OneVsAllResult> for OneVsAll {
    
    type Output = ();
}