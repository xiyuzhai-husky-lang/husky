use super::*;
use husky_print_utils::p;
use husky_syn_decl::PropsStructTypeSynDecl;
use husky_syn_expr::{PropsFieldSynInitialization, PropsFieldSyndicate};

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct PropsStructTypeHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    #[return_ref]
    pub fields: SmallVec<[PropsStructFieldHirDecl; 4]>,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDeclDb, jar= HirDeclJar)]
pub struct PropsStructFieldHirDecl {
    ident: Ident,
    ty: HirType,
    pub initialization: Option<PropsFieldHirInitialization>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PropsFieldHirInitialization {
    Bind { value: HirEagerExprIdx },
    Default {},
}

impl PropsStructTypeHirDecl {
    pub(super) fn from_syn(
        path: TypePath,
        syn_decl: PropsStructTypeSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), &builder);
        let fields = syn_decl
            .fields(db)
            .iter()
            .map(|field| PropsStructFieldHirDecl::from_syn(field, &builder))
            .collect();
        Self::new(
            db,
            path,
            template_parameters,
            fields,
            hir_eager_expr_region(syn_decl.syn_expr_region(db), db),
        )
    }
}

impl PropsStructFieldHirDecl {
    fn from_syn(field: &PropsFieldSyndicate, builder: &HirDeclBuilder) -> Self {
        Self {
            ident: field.ident(),
            ty: builder.hir_ty(field.ty_syn_expr_idx()).unwrap(),
            initialization: field.initialization().map(|initialization| {
                PropsFieldHirInitialization::from_syn(initialization, builder)
            }),
        }
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn ty(&self) -> HirType {
        self.ty
    }
}

impl PropsFieldHirInitialization {
    fn from_syn(
        initialization: PropsFieldSynInitialization,
        builder: &HirDeclBuilder,
    ) -> PropsFieldHirInitialization {
        // builder.sema_expr_region_data().sema_expr_roots();
        match initialization {
            PropsFieldSynInitialization::Bind {
                colon_eq_token,
                value,
            } => PropsFieldHirInitialization::Bind {
                value: builder.hir_eager_expr_idx(value).unwrap(),
            },
            PropsFieldSynInitialization::Default {} => todo!(),
        }
    }
}
