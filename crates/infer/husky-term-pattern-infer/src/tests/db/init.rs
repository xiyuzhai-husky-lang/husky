use husky_text::TextRange;

use super::*;

impl TermPatternInferFakeDb {
    pub(super) fn init(&mut self) {
        let entity_path_menu = self.entity_path_menu();
        let term_menu = self.term_menu();
        self.init_entity_tys(&entity_path_menu, &term_menu);
        self.init_decls(&entity_path_menu);
        self.init_prelude_symbols(&entity_path_menu)
    }

    fn init_entity_tys(&mut self, entity_path_menu: &EntityPathMenu, term_menu: &TermMenu) {
        self.entity_tys.extend([
            (entity_path_menu.i32(), term_menu.ty0()),
            (entity_path_menu.i64(), term_menu.ty0()),
            (entity_path_menu.b32(), term_menu.ty0()),
            (entity_path_menu.b64(), term_menu.ty0()),
            (entity_path_menu.f32(), term_menu.ty0()),
            (entity_path_menu.f64(), term_menu.ty0()),
        ]);
    }

    fn init_decls(&mut self, entity_path_menu: &EntityPathMenu) {
        self.decls.extend(
            [
                (entity_path_menu.core(), Decl::Module),
                (entity_path_menu.std(), Decl::Module),
                (
                    entity_path_menu.i32(),
                    Decl::Ty(TyDecl::new(TyFamily::Physical)),
                ),
                (
                    entity_path_menu.i64(),
                    Decl::Ty(TyDecl::new(TyFamily::Physical)),
                ),
                (
                    entity_path_menu.b32(),
                    Decl::Ty(TyDecl::new(TyFamily::Physical)),
                ),
                (
                    entity_path_menu.b64(),
                    Decl::Ty(TyDecl::new(TyFamily::Physical)),
                ),
                (
                    entity_path_menu.f64(),
                    Decl::Ty(TyDecl::new(TyFamily::Physical)),
                ),
                (
                    entity_path_menu.f64(),
                    Decl::Ty(TyDecl::new(TyFamily::Physical)),
                ),
            ]
            .into_iter()
            .map(|(entity_path, decl)| (entity_path, Arc::new(decl))),
        );
    }

    fn init_prelude_symbols(&mut self, entity_path_menu: &EntityPathMenu) {
        self.prelude_symbols.extend([
            Symbol {
                ident: RootBuiltinIdentifier::Core.into(),
                kind: SymbolKind::EntityPath(entity_path_menu.core()),
            },
            Symbol {
                ident: RootBuiltinIdentifier::Std.into(),
                kind: SymbolKind::EntityPath(entity_path_menu.std()),
            },
            Symbol {
                ident: RootBuiltinIdentifier::I32.into(),
                kind: SymbolKind::EntityPath(entity_path_menu.i32()),
            },
            Symbol {
                ident: RootBuiltinIdentifier::I64.into(),
                kind: SymbolKind::EntityPath(entity_path_menu.i64()),
            },
            Symbol {
                ident: RootBuiltinIdentifier::B32.into(),
                kind: SymbolKind::EntityPath(entity_path_menu.i32()),
            },
            Symbol {
                ident: RootBuiltinIdentifier::B64.into(),
                kind: SymbolKind::EntityPath(entity_path_menu.i64()),
            },
            Symbol {
                ident: RootBuiltinIdentifier::F32.into(),
                kind: SymbolKind::EntityPath(entity_path_menu.i32()),
            },
            Symbol {
                ident: RootBuiltinIdentifier::F64.into(),
                kind: SymbolKind::EntityPath(entity_path_menu.i64()),
            },
            Symbol {
                ident: self.it_ident("x"),
                kind: SymbolKind::LocalVariable {
                    init_range: ((0, 0)..(0, 4)).into(),
                },
            },
        ])
    }
}
