use crate::context::{IsGraphRecursionContext, IsGraphRecursionScheme};

pub(crate) fn calc_full_deps_cropped<
    'db,
    S: IsGraphRecursionScheme,
    C: IsGraphRecursionContext<'db, Scheme = S>,
>(
    ctx: C,
    node: S::Node,
) -> Vec<S::Node> {
    let mut cache = FullDepsCroppedCache::new(ctx, node);
    cache.populate_util_stable();
    cache.finish()
}

struct FullDepsCroppedCache<'db, C: IsGraphRecursionContext<'db>> {
    ctx: C,
    full_deps_cropped: Vec<<C::Scheme as IsGraphRecursionScheme>::Node>,
}

/// # constructor
impl<'db, C: IsGraphRecursionContext<'db>> FullDepsCroppedCache<'db, C> {
    fn new(ctx: C, center: <C::Scheme as IsGraphRecursionScheme>::Node) -> Self {
        Self {
            ctx,
            full_deps_cropped: vec![center],
        }
    }
}
/// actions

impl<'db, C: IsGraphRecursionContext<'db>> FullDepsCroppedCache<'db, C> {
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

    fn finish(self) -> Vec<<C::Scheme as IsGraphRecursionScheme>::Node> {
        self.full_deps_cropped
    }
}
