pub enum Class<Label>

pub enum OneVsAll<Label>

pub enum OneVsAllResult<Label>

impl <Label>Unveil<OneVsAll<Label>> for Class<Label> {
    
    type Output = unit;
}

impl <Label>Unveil<OneVsAllResult<Label>> for OneVsAll<Label> {
    
    type Output = unit;
}