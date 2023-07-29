use crate::*;

pub trait __RegularValueSnapshotDyn: std::fmt::Debug {
    fn clone_into_box_stand(&self) -> Box<dyn __RegularValueStandDyn> {
        todo!()
        // Box::new(self.clone())
    }
}
