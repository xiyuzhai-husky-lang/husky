use static_decl::StaticEntityDecl;
use vm::Linkage;

use crate::*;

impl<'a> InstructionSheetBuilder<'a> {
    pub(crate) fn routine_fp(&self, routine: EntityRoutePtr) -> Option<Linkage> {
        match self.db.entity_source(routine).unwrap() {
            EntitySource::StaticModuleItem(builtin_entity_data) => match builtin_entity_data.decl {
                StaticEntityDecl::Func(ref func_decl) => Some(func_decl.linkage),
                StaticEntityDecl::Type(_) => todo!(),
                StaticEntityDecl::Module => todo!(),
                StaticEntityDecl::Trait { .. } => todo!(),
            },
            EntitySource::WithinBuiltinModule => todo!(),
            EntitySource::WithinModule { .. } => {
                self.db.linkage_table().routine(self.db.entity_uid(routine))
            }
            EntitySource::Module { file } => todo!(),
            EntitySource::Input { main } => todo!(),
            EntitySource::StaticTypeMember => todo!(),
        }
    }

    pub(crate) fn field_access_fp(
        &self,
        this_ty: EntityRoutePtr,
        field_ident: CustomIdentifier,
    ) -> Option<Linkage> {
        self.db
            .linkage_table()
            .struct_field_access(self.db.entity_uid(this_ty), field_ident)
    }
}
