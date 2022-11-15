use crate::*;

impl EntityDefnVariant {
    pub(super) fn collect_trait_impls(
        db: &dyn EntityDefnQueryGroup,
        this_ty: Ty,
        file: FileItd,
        range: TextRange,
    ) -> Vec<Arc<TraitImplDefn>> {
        Self::implicit_trait_impls(db, this_ty, file, range)
    }

    fn implicit_trait_impls(
        _db: &dyn EntityDefnQueryGroup,
        _this_ty: Ty,
        _file: FileItd,
        _range: TextRange,
    ) -> Vec<Arc<TraitImplDefn>> {
        todo!()
        // let mut trait_impl_decls = Vec::<Arc<TraitImplDefn>>::new();
        // let entity_route_menu = db.entity_route_menu();
        // if db.is_copyable(this_ty).unwrap() {
        //     trait_impl_decls.push(Arc::new(TraitImplDefn {
        //         trai: entity_route_menu.copy_trait,
        //         member_impls: vec![],
        //         dev_src: DevSource {
        //             file: file.to_str().unwrap().to_owned(),
        //             line: range.start.line(),
        //         },
        //     }))
        // }
        // if db.is_clonable(this_ty).unwrap() {
        //     msg_once!("much to do here");
        //     let clone_trait = entity_route_menu.clone_trait;
        //     trait_impl_decls.push(Arc::new(TraitImplDefn {
        //         trai: entity_route_menu.copy_trait,
        //         member_impls: vec![EntityDefn::new(
        //             db,
        //             db.it_word("clone").ident(),
        //             EntityDefnVariant::Method {
        //                 spatial_parameters: Default::default(),
        //                 this_modifier: ParameterModifier::None,
        //                 parameters: Default::default(),
        //                 output_ty: RangedEntityRoute {
        //                     route: this_ty,
        //                     range,
        //                 },
        //                 output_modifier: OutputModifier::Transfer,
        //                 method_defn_kind: MethodDefnKind::TraitMethodImpl {
        //                     trai: entity_route_menu.copy_trait,
        //                 },
        //                 opt_source: None,
        //             },
        //             db.ty_as_trai_subroute(
        //                 this_ty,
        //                 clone_trait,
        //                 db.it_word("clone").custom(),
        //                 thin_vec![],
        //             ),
        //             file,
        //             range,
        //         )],
        //         dev_src: DevSource {
        //             file: file.to_str().unwrap().to_owned(),
        //             line: range.start.line(),
        //         },
        //     }))
        // }
        // msg_once!("handle other traits, PartialEq, Eq");
        // trait_impl_decls
    }
}
