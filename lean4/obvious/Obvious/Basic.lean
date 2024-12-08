import Mathlib

namespace Obvious

def hello := "world"

syntax "obvious" : tactic

macro_rules
  | `(tactic| obvious) => `(tactic|
      congr;
      try first
      | simp; done
      | (nlinarith)
      | (apply sq_nonneg; repeat obvious)
      | ((apply div_nonneg; repeat obvious))
      | (field_simp; ring)
      | linarith
    )

end Obvious
