import Mathlib

namespace Obvious

def hello := "world"

syntax "obvious" : tactic

macro_rules
  | `(tactic| obvious) => `(tactic|
      first
      | ring; done
      | (congr;(try (
        first
        | simp; done
        | (nlinarith)
        | (apply sq_nonneg; repeat obvious)
        | ((apply div_nonneg; repeat obvious))
        | ((try field_simp); ring)
        | linarith)))
    )

end Obvious
