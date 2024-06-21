use crate::{context::IsGraphScheme, cycle_group::CycleGroup};

pub(crate) fn calc_full_deps_cropped<'db, S: IsGraphScheme, C: IsGraphContext<'db, Scheme = S>>(
    ctx: C,
    node: S::Node,
) -> Vec<S::Node> {
    let mut cache = FullDepsCroppedCache::new(ctx, node);
    cache.populate_util_stable();
    cache.finish()
}

struct FullDepsCroppedCache<'db, C: IsGraphContext<'db>> {
    ctx: C,
    full_deps_cropped: Vec<<C::Scheme as IsGraphScheme>::Node>,
}

/// # constructor
impl<'db, C: IsGraphContext<'db>> FullDepsCroppedCache<'db, C> {
    fn new(ctx: C, center: <C::Scheme as IsGraphScheme>::Node) -> Self {
        Self {
            ctx,
            full_deps_cropped: vec![center],
        }
    }
}
/// actions

impl<'db, C: IsGraphContext<'db>> FullDepsCroppedCache<'db, C> {
    fn populate_util_stable(&mut self) {
        let mut prev_len = 0;
        loop {
            let len = self.full_deps_cropped.len();
            self.populate_from_new_nodes(prev_len);
            if self.full_deps_cropped.len() == len {
                break;
            }
            prev_len = len;
        }
    }

    fn populate_from_new_nodes(&mut self, prev_len: usize) {
        let len = self.full_deps_cropped.len();
        for source_index in prev_len..len {
            self.populate(source_index);
        }
    }

    fn populate(&mut self, source_index: usize) {
        let node = self.full_deps_cropped[source_index];
        for dep in self.ctx.deps_cropped(node) {
            if !self.full_deps_cropped.contains(&dep) {
                self.full_deps_cropped.push(dep)
            }
        }
    }

    fn finish(self) -> Vec<<C::Scheme as IsGraphScheme>::Node> {
        self.full_deps_cropped
    }
}

pub trait IsGraphContext<'db>: Copy {
    type Scheme: IsGraphScheme;
    /// crop deps that definitely are not going to form a cycle
    fn deps_cropped(self, node: Node<Self::Scheme>)
        -> impl IntoIterator<Item = Node<Self::Scheme>>;
    /// final
    fn calc_full_deps_cropped(self, node: Node<Self::Scheme>) -> Vec<Node<Self::Scheme>> {
        calc_full_deps_cropped(self, node)
    }
    /// cached version
    fn full_deps_cropped(self, node: Node<Self::Scheme>) -> &'db [Node<Self::Scheme>];
    /// # cycle group
    /// final
    fn calc_cycle_group(self, node: Node<Self::Scheme>) -> CycleGroup<Self::Scheme>
    where
        [(); <Self::Scheme as IsGraphScheme>::CYCLE_GROUP_N]:,
    {
        CycleGroup::calc(self, node)
    }
    /// cached version
    fn cycle_group_itd(self, node: Node<Self::Scheme>) -> CycleGroupItd<Self::Scheme>
    where
        [(); <Self::Scheme as IsGraphScheme>::CYCLE_GROUP_N]:;
}

type Node<S> = <S as IsGraphScheme>::Node;
type CycleGroupItd<S> = <S as IsGraphScheme>::CycleGroupItd;
