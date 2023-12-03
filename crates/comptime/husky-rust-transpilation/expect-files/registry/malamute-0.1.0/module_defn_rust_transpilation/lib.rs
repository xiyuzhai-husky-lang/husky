use husky_core::*;

pub enum Class<Label> {
    Known{Label},
    Unknown,
} 

pub enum OneVsAll<Label> {
    Yes,
    No,
} 

pub enum OneVsAllResult<Label> {
    ConfidentYes,
    ConfidentNo,
    Unconfident,
} 

impl <Label>Unveil<OneVsAll<Label>> for Class<Label> {
    
    type Output = unit;
}

impl <Label>Unveil<OneVsAllResult<Label>> for OneVsAll<Label> {
    
    type Output = unit;
}