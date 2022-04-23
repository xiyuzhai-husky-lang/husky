use avec::Avec;
use entity_route::EntityRouteKind;
use infer_decl::MethodKind;
use static_defn::{MethodStaticDefnKind, StaticLinkageSource};

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum MethodSource {
    Func { stmts: Avec<FuncStmt> },
    Proc { stmts: Avec<ProcStmt> },
    Pattern { stmts: Avec<LazyStmt> },
    Static(StaticLinkageSource),
}

#[derive(Debug, PartialEq, Eq)]
pub enum MethodDefnVariant {
    TypeMethod {
        ty: EntityRoutePtr,
        method_source: MethodSource,
    },
    TraitMethod {
        trai: EntityRoutePtr,
        opt_default_source: Option<MethodSource>,
    },
    TraitMethodImpl {
        trai: EntityRoutePtr,
        opt_source: Option<MethodSource>,
    },
}

impl MethodDefnVariant {
    pub fn from_static(symbol_context: &SymbolContext, method_kind: MethodStaticDefnKind) -> Self {
        match method_kind {
            MethodStaticDefnKind::TypeMethod { source } => MethodDefnVariant::TypeMethod {
                ty: symbol_context.opt_this_ty.unwrap(),
                method_source: MethodSource::Static(source),
            },
            MethodStaticDefnKind::TraitMethod { opt_default_source } => todo!(),
            MethodStaticDefnKind::TraitMethodImpl { opt_source } => {
                panic!("this shouldn't be handled here")
            }
        }
    }
}

impl EntityDefnVariant {
    pub(crate) fn collect_other_members(
        db: &dyn InferQueryGroup,
        arena: &RawExprArena,
        file: FilePtr,
        ty_route: EntityRoutePtr,
        mut children: Peekable<AstIter>,
        members: &mut IdentDict<Arc<EntityDefn>>,
    ) -> SemanticResult<()> {
        while let Some(child) = children.next() {
            let ast = child.value.as_ref()?;
            match ast.kind {
                AstKind::TypeDefnHead {
                    ident,
                    kind,
                    ref generic_placeholders,
                } => todo!(),
                AstKind::MainDefn => todo!(),
                AstKind::RoutineDefnHead(_) => todo!(),
                AstKind::PatternDefnHead => todo!(),
                AstKind::FeatureDecl { ident, ty } => todo!(),
                AstKind::TypeMethodDefnHead(ref head) => {
                    let method_source = match head.routine_kind {
                        RoutineContextKind::Proc => todo!(),
                        RoutineContextKind::Func => {
                            let stmts = semantics_eager::parse_decl_stmts(
                                &head.input_placeholders,
                                db,
                                arena,
                                child.children.unwrap(),
                                file,
                            )?;
                            MethodSource::Func { stmts }
                        }
                        RoutineContextKind::Test => todo!(),
                    };
                    let method_variant = MethodDefnVariant::TypeMethod {
                        ty: ty_route,
                        method_source,
                    };
                    members.insert_new(EntityDefn::new(
                        head.ident.into(),
                        EntityDefnVariant::Method {
                            input_placeholders: head.input_placeholders.clone(),
                            output_ty: head.output_ty,
                            this_contract: head.this_contract,
                            method_variant,
                            output_contract: OutputContract::Pure,
                            generic_placeholders: head.generic_placeholders.clone(),
                        },
                        db.intern_entity_route(EntityRoute {
                            kind: EntityRouteKind::Child {
                                parent: ty_route,
                                ident: head.ident,
                            },
                            generic_arguments: vec![],
                        }),
                        file,
                        ast.range,
                    ))
                }
                AstKind::Use { .. } => todo!(),
                AstKind::FieldDefnHead(_) => todo!(),
                AstKind::DatasetConfigDefnHead => todo!(),
                AstKind::Stmt(_) => todo!(),
                AstKind::EnumVariantDefnHead {
                    ident,
                    variant_class,
                } => todo!(),
            }
        }
        Ok(())
    }
}
