mod file;
mod tracker;

use self::{file::*, tracker::*};
use husky_core::*;
use husky_standard_linket_impl::ugly::*;
use idx_arena::ArenaIdx;
use latex_ast::ast::LxAstIdx;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LxAstId {
    pub file: LxFileIdx,
    pub idx: LxAstIdx,
}

impl From<__VarId> for LxAstId {
    fn from(id: __VarId) -> Self {
        let __VarId::Pair([fst, snd]) = id else {
            todo!()
        };
        Self {
            file: fst.into(),
            idx: unsafe { lx_ast_idx_from_u32(snd) },
        }
    }
}

impl Into<__VarId> for LxAstId {
    fn into(self) -> __VarId {
        [self.file.into(), unsafe { lx_ast_idx_to_u32(self.idx) }].into()
    }
}

unsafe fn lx_ast_idx_from_u32(idx: u32) -> LxAstIdx {
    match idx >> 30 {
        0 => LxAstIdx::Math(ArenaIdx::new_ext(idx as usize & 0x3FFFFFFF)),
        1 => LxAstIdx::Rose(ArenaIdx::new_ext(idx as usize & 0x3FFFFFFF)),
        2 => LxAstIdx::Lisp(ArenaIdx::new_ext(idx as usize & 0x3FFFFFFF)),
        3 => LxAstIdx::Root(ArenaIdx::new_ext(idx as usize & 0x3FFFFFFF)),
        _ => unreachable!(),
    }
}

fn lx_ast_idx_to_u32(idx: LxAstIdx) -> u32 {
    match idx {
        LxAstIdx::Math(idx) => {
            assert!(idx.index() <= 0x3FFFFFFF);
            (0b00 << 30) | idx.index() as u32
        }
        LxAstIdx::Rose(idx) => {
            assert!(idx.index() <= 0x3FFFFFFF);
            (0b01 << 30) | idx.index() as u32
        }
        LxAstIdx::Lisp(idx) => {
            assert!(idx.index() <= 0x3FFFFFFF);
            (0b10 << 30) | idx.index() as u32
        }
        LxAstIdx::Root(idx) => {
            assert!(idx.index() <= 0x3FFFFFFF);
            (0b11 << 30) | idx.index() as u32
        }
    }
}

#[test]
fn conversion_between_u32_and_ast_idx_works() {
    // Test Math variant (00)
    let idx = LxAstIdx::Math(unsafe { ArenaIdx::new_ext(0) });
    assert_eq!(
        unsafe { lx_ast_idx_to_u32(idx) },
        0b0000_0000_0000_0000_0000_0000_0000_0000
    );
    assert_eq!(
        unsafe { lx_ast_idx_from_u32(unsafe { lx_ast_idx_to_u32(idx) }) },
        idx
    );

    // Test Rose variant (01)
    let idx = LxAstIdx::Rose(unsafe { ArenaIdx::new_ext(42) });
    assert_eq!(
        unsafe { lx_ast_idx_to_u32(idx) },
        0b0100_0000_0000_0000_0000_0000_0010_1010
    );
    assert_eq!(
        unsafe { lx_ast_idx_from_u32(unsafe { lx_ast_idx_to_u32(idx) }) },
        idx
    );

    // Test Lisp variant (10)
    let idx = LxAstIdx::Lisp(unsafe { ArenaIdx::new_ext(0x3FFFFFFE) });
    assert_eq!(
        unsafe { lx_ast_idx_to_u32(idx) },
        0b1011_1111_1111_1111_1111_1111_1111_1110
    );
    assert_eq!(
        unsafe { lx_ast_idx_from_u32(unsafe { lx_ast_idx_to_u32(idx) }) },
        idx
    );

    // Test Root variant (11)
    let idx = LxAstIdx::Root(unsafe { ArenaIdx::new_ext(0x3FFFFFFF) });
    assert_eq!(
        unsafe { lx_ast_idx_to_u32(idx) },
        0b1111_1111_1111_1111_1111_1111_1111_1111
    );
    assert_eq!(
        unsafe { lx_ast_idx_from_u32(unsafe { lx_ast_idx_to_u32(idx) }) },
        idx
    );
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_idx_overflow() {
    let idx = LxAstIdx::Math(unsafe { ArenaIdx::new_ext(0x40000000) });
    unsafe { lx_ast_idx_to_u32(idx) }; // Should panic due to index being too large
}

#[allow(non_upper_case_globals)]
pub static mut __AST__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[allow(non_snake_case)]
pub fn AST() -> LxAstId {
    ast_id()
}

thread_local! {
    static __AST_ID: std::cell::Cell<Option<LxAstId>> = Default::default();
}

pub(crate) fn ast_id() -> LxAstId {
    __AST_ID.get().unwrap()
}

pub(crate) fn with_ast_id<R>(ast_id: LxAstId, f: impl Fn() -> R) -> R {
    let old = __AST_ID.replace(Some(ast_id));
    let r = f();
    __AST_ID.set(old);
    r
}

pub(crate) fn replace_ast_id(ast_id: LxAstId) -> Option<LxAstId> {
    __AST_ID.replace(Some(ast_id))
}

pub(crate) fn set_ast_id(ast_id: Option<LxAstId>) {
    __AST_ID.set(ast_id)
}

pub struct AST {}

impl __IsStaticVar<__VarId> for AST {
    fn item_path_id_interface() -> __ItemPathIdInterface {
        unsafe { __AST__ITEM_PATH_ID_INTERFACE.unwrap() }
    }

    fn page_var_ids_aux(locked: &[__ItemPathIdInterface]) -> impl Iterator<Item = __VarId> {
        [todo!()].into_iter()
    }

    fn default_page_start(
        figure_zone: __FigureZone,
        locked: &[__ItemPathIdInterface],
    ) -> husky_linket_impl::static_var::StaticVarResult<__VarId, __VarId> {
        todo!()
    }

    fn get_id() -> __VarId {
        AST().into()
    }

    fn try_set_var_id_aux(
        new: __VarId,
        locked: &[__ItemPathIdInterface],
    ) -> __StaticVarResult<impl FnOnce() + 'static> {
        Ok(move || {
            todo!()
            // set_input_id(old);
        })
    }

    fn try_set_default_var_id(
        locked: &[__ItemPathIdInterface],
    ) -> __StaticVarResult<(__VarId, impl FnOnce() + 'static)> {
        // TODO: is this correct?
        todo!();
        let default = [0, 0].into();
        Ok((default, Self::try_set_var_id(default, locked)?))
    }

    type Value = __Value;

    fn get_value() -> Self::Value {
        let id = AST();
        __Value::U32U32Pair(id.file.into(), lx_ast_idx_to_u32(id.idx))
    }

    fn zones() -> &'static [__FigureZone] {
        &[__FigureZone::Text]
    }
}
