inductive Case
  | Snake

inductive Boundary

inductive Pattern

structure Converter where
  boundaries: List Boundary
  pattern: Option Pattern
  delim: String

structure StateConverter (α : Type) where
    s: α
    conv: Converter

structure ToCase (α : Type) where
    toCase : α -> Case -> String
    fromCase : α -> Case -> StateConverter α
    withBoundaries: α -> List Boundary -> StateConverter α
    isCase : α -> Case -> Bool