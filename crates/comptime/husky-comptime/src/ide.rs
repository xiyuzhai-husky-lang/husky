use husky_diagnostics::{Diagnostic, DiagnosticQuery};
use husky_hover_contents::HoverContentsQuery;

use crate::*;

impl DiagnosticQuery for HuskyComptime {}

impl HoverContentsQuery for HuskyComptime {}
