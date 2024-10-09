//! the implementation is quite ad hoc
//!
//! we shall carefully consider how we shall statically analyze this in the future
use self::term::PlotClassTerm;
use super::{error::DerivedSemExprHtmxError, *};
use helpers::path::sem_expr_region_from_region_path;
use husky_entity_path::{path::major_item::ty::PreludeTypePath, region::RegionPath};
use husky_eth_term::term::EthTerm;
use husky_term_prelude::ItemPathTerm;
use husky_visual_protocol::plot::PlotClass;
use path::ItemPath;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SemExprHtmxRegion {
    path: RegionPath,
    plot_class_term: SemExprHtmxResult<PlotClassTerm>,
    // ad hoc
    errors: Vec<()>,
}

#[salsa::tracked(return_ref)]
pub fn sem_expr_htmx_region(
    db: &::salsa::Db,
    sem_expr_region: SemExprRegion,
) -> Option<SemExprHtmxRegion> {
    let mut builder = SemExprHtmxRegionBuilder::new(sem_expr_region, db)?;
    builder.determine_plot_class();
    builder.scan_for_errors();
    Some(builder.finish())
}

struct SemExprHtmxRegionBuilder<'db> {
    db: &'db ::salsa::Db,
    sem_expr_region_data: &'db SemExprRegionData,
    plot_class_term: SemExprHtmxResult<Option<PlotClassTerm>>,
    errors: Vec<()>,
}

impl<'db> SemExprHtmxRegionBuilder<'db> {
    fn new(sem_expr_region: SemExprRegion, db: &'db ::salsa::Db) -> Option<Self> {
        let sem_expr_region_data = sem_expr_region.data(db);
        let EthTerm::ItemPath(ItemPathTerm::TypeOntology(return_ty)) =
            sem_expr_region_data.return_ty()?
        else {
            return None;
        };
        if return_ty.refine(db) != Left(PreludeTypePath::VISUAL) {
            return None;
        }
        Some(Self {
            db,
            sem_expr_region_data,
            plot_class_term: Ok(None),
            errors: vec![],
        })
    }
}

impl<'db> SemExprHtmxRegionBuilder<'db> {
    fn determine_plot_class(&mut self) {
        self.determine_plot_class_from_returned_expr(self.sem_expr_region_data.root_body())
    }

    fn determine_plot_class_from_returned_expr(&mut self, expr: SemExprIdx) {
        let db = self.db;
        match expr.data(self.sem_expr_region_data.sem_expr_arena()) {
            SemExprData::EmptyHtmxTag {
                empty_htmx_bra_idx,
                function_ident,
                arguments,
                empty_htmx_ket,
            } => todo!(),
            // todo: NonEmptyHmtxTag
            SemExprData::Block { stmts } | SemExprData::NestedBlock { stmts, .. } => {
                self.determine_plot_class_from_last_stmt(stmts.last().unwrap())
            }
            SemExprData::MethodRitchieCall { dispatch, .. } => {
                match sem_expr_htmx_region_from_item_path(dispatch.signature().path().into(), db) {
                    Some(sem_expr_htmx_region) => {
                        self.plot_class_term = sem_expr_htmx_region
                            .plot_class_term
                            .as_ref()
                            .map(|&term| Some(term))
                            .map_err(|_| DerivedSemExprHtmxError::MethodRitchieCall.into())
                    }
                    None => {
                        use ::husky_print_utils::p;
                        use ::salsa::DebugWithDb;
                        p!(dispatch.signature().path().debug(db));
                        todo!()
                    }
                }
            }
            other => {
                use ::husky_print_utils::p;
                use ::salsa::DebugWithDb;
                let db = self.db;
                p!(other.debug(db));
                todo!()
            }
        }
        todo!()
    }

    fn determine_plot_class_from_last_stmt(&mut self, stmt: SemStmtIdx) {
        match *stmt.data(self.sem_expr_region_data.sem_stmt_arena()) {
            SemStmtData::Return { result: expr, .. } | SemStmtData::Eval { expr, .. } => {
                self.determine_plot_class_from_returned_expr(expr)
            }
            _ => todo!(),
        }
    }

    fn scan_for_errors(&mut self) {
        // todo: Implement the logic to scan for errors and populate the errors vector.
        // This is a placeholder implementation.
        // Push any detected errors into self.errors.
    }

    fn finish(self) -> SemExprHtmxRegion {
        SemExprHtmxRegion {
            path: self.sem_expr_region_data.path(),
            plot_class_term: self
                .plot_class_term
                .map(|plot_class_term| plot_class_term.unwrap_or(PlotClass::Any.into())),
            errors: self.errors,
        }
    }
}

// got stuck
#[test]
#[ignore]
fn sem_expr_htmx_region_works() {
    DB::ast_rich_test_debug_with_db(
        |db, item_path: ItemPath| sem_expr_htmx_region_from_item_path(item_path, db),
        &AstTestConfig::new(
            "sem_expr_htmx_region",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SEMANTICS,
        ),
    )
}

fn sem_expr_htmx_region_from_item_path(
    item_path: ItemPath,
    db: &salsa::Db,
) -> Option<&SemExprHtmxRegion> {
    let sem_expr_region = sem_expr_region_from_region_path(RegionPath::ItemDefn(item_path), db)?;
    sem_expr_htmx_region(db, sem_expr_region).as_ref()
}
