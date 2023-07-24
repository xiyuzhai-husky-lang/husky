use husky_rust_code_repr::registration::NonPrimitiveTypeRegistration;
use husky_write_utils::w;
use std::fmt::Write;

use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn gen_registration_content(&mut self) {
        self.write(
            r#"use crate::*;
pub(crate) use __husky::registration::*;

"#,
        );
        let main_module = self.db.module(self.target_entrance).unwrap();
        let item_link_dependees = self.db.item_link_dependees(main_module);
        for item_path in item_link_dependees.iter() {
            if !item_path.contains_any() {
                let item_defn = self.db.item_defn(*item_path).unwrap();
                self.might_gen_ty_registration(*item_path, &item_defn);
            }
        }
    }

    fn might_gen_ty_registration(&mut self, item_path: EtherealTerm, item_defn: &EntityDefn) {
        if self.db.is_defn_static(item_path) && !self.db.contains_spatial_parameters(item_path) {
            return;
        }
        match item_defn.variant {
            EntityDefnVariant::EtherealTerm { ty_kind, .. } => {
                match ty_kind {
                    TyKind::Record => return,
                    TyKind::HigherKind => return,
                    _ => (),
                }
                self.gen_ty_registration(item_path)
            }
            _ => (),
        }
    }

    fn gen_ty_registration(&mut self, item_path: EtherealTerm) {
        let mangled_intrinsic_ty = self.db.mangled_intrinsic_ty(item_path);
        let needs_eval_ref = self.db.item_route_contains_eval_ref(item_path);
        write!(
            self.result,
            "type {}{} = ",
            mangled_intrinsic_ty,
            match needs_eval_ref {
                true => "<'eval>",
                false => "",
            }
        )
        .unwrap();
        self.gen_item_route(item_path, EntityRouteRole::Decl);
        self.write(";\n");
        w!(self.result; (NonPrimitiveTypeRegistration { ty: &mangled_intrinsic_ty }))
    }
}
