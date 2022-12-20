use crate::*;

pub struct ImplementationMap {
    data: Vec<(EntityPath, Vec<AstIdx>)>,
}

impl ImplementationMap {
    pub(crate) fn insert(&mut self, entity_path: EntityPath, ast_idx: AstIdx) {
        if let Some((_, implementations)) =
            self.data.iter_mut().find(|(path, _)| *path == entity_path)
        {
            match implementations.binary_search(&ast_idx) {
                Ok(_pos) => todo!(), // element already in vector @ `pos` ,
                Err(pos) => implementations.insert(pos, ast_idx),
            }
        } else {
            todo!()
        }
    }
}
