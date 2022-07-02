use super::*;
use dev_utils::StaticDevSource;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MemberLinkage {
    pub copy_access: SpecificRoutineFp,
    pub eval_ref_access: SpecificRoutineFp,
    pub temp_ref_access: SpecificRoutineFp,
    pub temp_mut_access: SpecificRoutineFp,
    pub move_access: SpecificRoutineFp,
    pub nargs: u8,
    pub dev_src: StaticDevSource,
}

impl MemberLinkage {
    pub fn bind(&self, binding: Binding) -> SpecificRoutineLinkage {
        SpecificRoutineLinkage {
            call: match binding {
                Binding::EvalRef => self.eval_ref_access,
                Binding::TempRef => self.temp_ref_access,
                Binding::TempRefMut => self.temp_mut_access,
                Binding::Move => self.move_access,
                Binding::Copy => self.copy_access,
            },
            nargs: self.nargs,
            dev_src: self.dev_src,
        }
    }
}

#[macro_export]
macro_rules! index_usize_linkage {
    ($Type: ty) => {
        MemberLinkage {
            copy_access: SpecificRoutineFp,
            eval_ref_access: SpecificRoutineFp,
            temp_ref_access: SpecificRoutineFp,
            temp_mut_access: SpecificRoutineFp,
            move_access: SpecificRoutineFp,
            pub nargs: u8,
            pub dev_src: static_dev_src!(),
        }
    };
}
