mod error;

use self::error::{DerivedSemExprHtmxError, SemExprHtmxResult};
use crate::SemExprRegion;
use either::*;
use husky_entity_path::path::major_item::ty::PreludeTypePath;
use husky_eth_term::term::EthTerm;
use husky_term_prelude::ItemPathTerm;
use husky_visual_protocol::plot::PlotClass;

#[derive(Debug, PartialEq, Eq)]
pub struct SemExprHtmxRegion {
    plot_class: SemExprHtmxResult<PlotClass>,
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
    plot_class: Option<PlotClass>,
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
            plot_class: None,
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
            plot_class: self
                .plot_class
                .ok_or(DerivedSemExprHtmxError::PlotClassNotInferred.into()),
            errors: self.errors,
        }
    }
}
