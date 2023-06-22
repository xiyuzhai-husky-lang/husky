use vec_like::VecPairMap;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DecrId {
    parent: DecrParent,
    ident: Ident,
    disambiguator: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DecrParent {
    Defn(EntityPath),
}

impl DecrId {
    pub fn module_path(self, db: &dyn AstDb) -> ModulePath {
        match self.parent {
            DecrParent::Defn(path) => path.module_path(db),
        }
    }

    pub fn toolchain(self, db: &dyn AstDb) -> Toolchain {
        self.module_path(db).toolchain(db)
    }

    pub fn parent(&self) -> DecrParent {
        self.parent
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn disambiguator(&self) -> u8 {
        self.disambiguator
    }
}

#[derive(Debug)]
struct DecrRegistry {
    parent: DecrParent,
    next_decr_disambiguators: VecPairMap<Ident, u8>,
}

impl DecrRegistry {
    fn new(decr_parent: DecrParent) -> Self {
        Self {
            parent: decr_parent,
            next_decr_disambiguators: Default::default(),
        }
    }
    fn issue(&mut self, ident: Ident) -> DecrId {
        let next_disambiguator = self
            .next_decr_disambiguators
            .get_value_mut_or_insert_default(ident);
        let disambiguator = *next_disambiguator;
        *next_disambiguator += 1;
        DecrId {
            parent: self.parent,
            ident,
            disambiguator,
        }
    }
}

impl AstSheet {
    // todo: needs testing
    #[inline(always)]
    pub fn decrs<'a, D, E>(
        &self,
        target: AstIdx,
        f: impl Fn(AstIdx, TokenGroupIdx, DecrId) -> Result<D, E>,
        invalid_parent: impl FnOnce() -> E,
    ) -> Result<Vec<D>, E> {
        let decr_parent = match self.ast_arena[target] {
            Ast::Defn { block, .. } => DecrParent::Defn(match block {
                DefnBlock::Fugitive { path, body } => todo!(),
                DefnBlock::Submodule { path } => todo!(),
                DefnBlock::Type { path, variants } => path.into(),
                DefnBlock::Trait { path, items } => todo!(),
                DefnBlock::AssociatedItem { body } => todo!(),
            }),
            // Some(entity_path) => entity_path,
            // None => return Err(invalid_parent()),
            _ => todo!(),
        };
        let mut registry = DecrRegistry::new(decr_parent);
        let mut decrs: Vec<D> = vec![];
        for (ast_idx, token_group_idx, ident) in self
            .siblings
            .iter()
            .filter_map(move |siblings| siblings.contains(target).then_some(siblings.start()))
            .map(move |siblings_start| {
                (siblings_start..target)
                    .rev()
                    .map(|ast_idx| (ast_idx, &self.ast_arena[ast_idx]))
                    .take_while(|(_, ast)| match ast {
                        Ast::Attr { .. } | Ast::Decr { .. } => true,
                        _ => false,
                    })
                    .filter_map(|(ast_idx, ast)| match ast {
                        Ast::Attr { .. } => None,
                        Ast::Decr {
                            token_group_idx,
                            ident,
                        } => Some((ast_idx, *token_group_idx, *ident)),
                        _ => unreachable!(),
                    })
            })
            .flatten()
        {
            decrs.push(f(ast_idx, token_group_idx, registry.issue(ident))?)
        }
        Ok(decrs)
    }
}
