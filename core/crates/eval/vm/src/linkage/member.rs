use super::*;
use dev_utils::__StaticDevSource;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct __MemberLinkage {
    pub copy_access: __SpecificRoutineFp,
    pub eval_ref_access: __SpecificRoutineFp,
    pub temp_ref_access: __SpecificRoutineFp,
    pub temp_mut_access: __SpecificRoutineFp,
    pub move_access: __SpecificRoutineFp,
    pub nargs: u8,
    pub dev_src: __StaticDevSource,
}

impl __MemberLinkage {
    pub fn bind(&self, binding: Binding) -> __SpecificRoutineLinkage {
        __SpecificRoutineLinkage {
            fp: match binding {
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
        __MemberLinkage {
            copy_access: __SpecificRoutineFp,
            eval_ref_access: __SpecificRoutineFp,
            temp_ref_access: __SpecificRoutineFp,
            temp_mut_access: __SpecificRoutineFp,
            move_access: __SpecificRoutineFp,
            pub nargs: u8,
            pub dev_src: __static_dev_src!(),
        }
    };
}
