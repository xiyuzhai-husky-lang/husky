use self::term::PlotClassTerm;
use super::{error::DerivedSemExprHtmxError, *};
use helpers::region::sem_expr_region_from_region_path;
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
    plot_class_term: Option<PlotClassTerm>,
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
            plot_class_term: None,
            errors: vec![],
        })
    }
}

impl<'db> SemExprHtmxRegionBuilder<'db> {
    fn determine_plot_class(&mut self) {
        // Implement the logic to determine the plot class based on sem_expr_region.
        // This is a placeholder implementation.
        // self.db can be used here to access necessary database methods.
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
                .ok_or(DerivedSemExprHtmxError::PlotClassNotInferred.into()),
            errors: self.errors,
        }
    }
}

#[test]
fn sem_expr_htmx_region_works() {
    DB::ast_rich_test_debug_with_db(
        |db, item_path: ItemPath| {
            let sem_expr_region =
                sem_expr_region_from_region_path(RegionPath::ItemDefn(item_path), db)?;
            sem_expr_htmx_region(db, sem_expr_region).as_ref()
        },
        &AstTestConfig::new(
            "sem_expr_htmx_region",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::SEMANTICS,
        ),
    )
}
