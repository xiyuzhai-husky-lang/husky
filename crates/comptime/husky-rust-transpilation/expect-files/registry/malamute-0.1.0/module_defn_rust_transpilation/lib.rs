pub struct Class<Label>

pub struct OneVsAll<Label>

pub struct OneVsAllResult<Label>

impl <Label>Unveil<OneVsAll<Label>> for Class<Label> {
    
    type Output = unit;
}

impl <Label>Unveil<OneVsAllResult<Label>> for OneVsAll<Label> {
    
    type Output = unit;
}