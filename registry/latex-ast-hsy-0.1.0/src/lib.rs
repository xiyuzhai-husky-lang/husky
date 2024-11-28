use husky_core::*;
use husky_standard_linket_impl::ugly::*;
use latex_ast::ast::LxAstIdx;

pub struct LxAstId {
    // TODO: file id
    pub idx: LxAstIdx,
}

#[allow(non_upper_case_globals)]
pub static mut __AST__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[allow(non_snake_case)]
pub fn AST() -> LxAstId {
    todo!()
    // MNIST_DATASET.input_leashed(input_id())
}

pub struct AST {}

impl __IsStaticVar<__VarId> for AST {
    fn item_path_id_interface() -> __ItemPathIdInterface {
        todo!()
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
        todo!()
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
        let default = 0usize.into();
        Ok((default, Self::try_set_var_id(default, locked)?))
    }

    type Value = __Value;

    fn get_value() -> Self::Value {
        todo!()
    }

    fn zones() -> &'static [__FigureZone] {
        todo!()
    }
}
