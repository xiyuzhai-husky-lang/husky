pub trait Projectable {
    type Projectile: Eq;
    type Projection;
    type State;

    fn project(&self, state: &mut Self::State) -> Self::Projectile;
}

pub struct Projector<Value: Projectable> {
    state: Value::State,
    value: Value,
}

impl<Value: Projectable> Projector<Value> {
    pub fn project(&mut self) -> Value::Projectile {
        self.value.project(&mut self.state)
    }
}
