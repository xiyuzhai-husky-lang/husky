pub trait Projector {
    type Change: Eq;
    type Projection;

    fn project_change(&mut self) -> Self::Change;
}

pub trait Syncable {}
