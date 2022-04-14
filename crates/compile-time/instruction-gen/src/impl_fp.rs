use static_decl::StaticEntityDecl;
use vm::{MembAccessFp, RoutineLinkage};

use crate::*;

impl<'a> InstructionSheetBuilder<'a> {
    pub(crate) fn routine_fp(&self, routine: EntityRoutePtr) -> Option<RoutineLinkage> {
        match self.db.entity_source(routine).unwrap() {
            EntitySource::Static(builtin_entity_data) => match builtin_entity_data.decl {
                StaticEntityDecl::Func(ref func_decl) => Some(func_decl.compiled),
                StaticEntityDecl::Ty {
                    raw_ty_kind,
                    visualizer,
                } => todo!(),
                StaticEntityDecl::TyTemplate => todo!(),
                StaticEntityDecl::Module => todo!(),
                StaticEntityDecl::Trait { .. } => todo!(),
            },
            EntitySource::WithinBuiltinModule => todo!(),
            EntitySource::WithinModule { .. } => self
                .db
                .linkage_table()
                .entity_routine(self.db.entity_uid(routine)),
            EntitySource::Module { file } => todo!(),
            EntitySource::Input { main } => todo!(),
        }
    }

    pub(crate) fn field_access_fp(
        &self,
        this_ty: EntityRoutePtr,
        field_ident: CustomIdentifier,
    ) -> Option<MembAccessFp> {
        self.db
            .linkage_table()
            .field_access(self.db.entity_uid(this_ty), field_ident)
    }

    pub(crate) fn field_routine_fp(
        &self,
        this_ty: EntityRoutePtr,
        field_ident: CustomIdentifier,
    ) -> Option<RoutineLinkage> {
        self.db
            .linkage_table()
            .field_routine(self.db.entity_uid(this_ty), field_ident)
    }
}
