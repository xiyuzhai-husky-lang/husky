pub mod destructive;
pub mod restructive;

use self::{destructive::VmirDestructivePattern, restructive::VmirRestructivePattern};

pub enum VmirPattern {
    Destructive(VmirDestructivePattern),
    Restructive(VmirRestructivePattern),
}
