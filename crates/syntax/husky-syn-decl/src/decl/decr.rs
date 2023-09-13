mod derive;

pub use self::derive::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum DecrSynNodeDecl {
    Derive(DeriveDecrSynNodeDecl),
}

impl DecrSynNodeDecl {
    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> SynExprRegion {
        match self {
            DecrSynNodeDecl::Derive(syn_node_decl) => syn_node_decl.syn_expr_region(db),
        }
    }
}

impl HasSynNodeDecl for DecrSynNodePath {
    type NodeDecl = DecrSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
        decr_syn_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
fn decr_syn_node_decl(db: &dyn SynDeclDb, syn_node_path: DecrSynNodePath) -> DecrSynNodeDecl {
    // ad hoc
    let coword_menu = db.coword_menu();
    let decr_ident = syn_node_path.ident(db);
    if decr_ident == coword_menu.derive_ident() {
        DecrSynNodeDecl::Derive(DeriveDecrSynNodeDecl::new(db, syn_node_path))
    } else {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum DecrSynDecl {
    Derive(DeriveDecrSynDecl),
}

impl DecrSynDecl {
    #[inline(always)]
    fn from_node_decl(
        db: &dyn SynDeclDb,
        path: DecrPath,
        syn_node_decl: DecrSynNodeDecl,
    ) -> DeclResult<Self> {
        Ok(match syn_node_decl {
            DecrSynNodeDecl::Derive(syn_node_decl) => {
                DeriveDecrSynDecl::from_node_decl(db, path, syn_node_decl)?.into()
            }
        })
    }
}

impl HasSynDecl for DecrPath {
    type Decl = DecrSynDecl;

    fn syn_decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl> {
        decr_syn_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn decr_syn_decl(db: &dyn SynDeclDb, path: DecrPath) -> DeclResult<DecrSynDecl> {
    DecrSynDecl::from_node_decl(db, path, path.syn_node_path(db).syn_node_decl(db))
}

#[test]
fn decr_decl_works() {}
