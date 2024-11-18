#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LxDivisionKind {
    Chapter,
    Section,
    Subsection,
    Subsubsection,
    Subsubsubsection,
    Stmts,
}
