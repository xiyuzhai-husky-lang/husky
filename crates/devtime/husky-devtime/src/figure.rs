use crate::Devtime;
use husky_devsoul::devsoul::IsDevsoul;
use husky_entity_path::path::{major_item::MajorItemPath, ItemPath, ItemPathId};
use husky_figure_zone_protocol::chunk_base::{text::FigureTextChunkBaseData, FigureChunkBase};
use husky_item_path_interface::ItemPathIdInterface;
use husky_linket::linket::Linket;
use husky_linket_impl::ugly::__IsLinketImpl;
use husky_visual_protocol::synchrotron::VisualSynchrotron;

impl<Devsoul: IsDevsoul> Devtime<Devsoul> {
    pub fn figure_chunk_base(
        &self,
        static_var: ItemPathIdInterface,
        chunk_idx: u32,
        visual_synchrotron: &mut VisualSynchrotron,
    ) -> FigureChunkBase {
        self.figure_chunk_base_cache
            .entry((static_var, chunk_idx))
            .or_insert_with(|| {
                visual_synchrotron.alloc_figure_text_chunk_base(
                    self.calc_figure_chunk_base(static_var, chunk_idx),
                )
            })
            .clone()
    }

    fn calc_figure_chunk_base(
        &self,
        static_var: ItemPathIdInterface,
        chunk_idx: u32,
    ) -> FigureTextChunkBaseData {
        let db = self.db();
        let item_path_id: ItemPathId = static_var.into();
        let item_path: ItemPath = item_path_id.item_path(db);
        let ItemPath::MajorItem(MajorItemPath::Form(major_form_path)) = item_path else {
            unreachable!()
        };
        let linket = Linket::new_var(major_form_path, db);
        let linket_impl = self.runtime.comptime().linket_impl(linket);
        linket_impl
            .static_var_svtable()
            .figure_chunk_base(chunk_idx)
    }
}
