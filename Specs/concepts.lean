import Specs.concepts.Term
import Specs.concepts.Paradigm
import Specs.concepts.decl

-- I believe that category theory is an overkill for describing type system
-- I failed to see things corresponding to `isomorphism` `universal property` in type system
-- so here we organise the concepts without establishing connection with category
-- afterall, it's just a machinery, designed for a very different purpose
-- still, it's a good place to borrow names, like monad

inductive ControlFlow
  | Break
