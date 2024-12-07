import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Tactic.FieldSimp
import Mathlib.Data.Real.Basic

namespace Example5

syntax "obvious" : tactic

macro_rules
  | `(tactic| obvious) => `(tactic| first | (nlinarith) | ((apply div_nonneg; repeat obvious))| (field_simp; ring) | linarith  | (congr; ring))

theorem am_gm_special (x : ℝ) (h : x > 0) : x + 1/x ≥ 2 := by
  have h1 : x + 1/x - 2 = (x^2 + 1 - 2*x)/x := by obvious

  have h2 : (x^2 + 1 - 2*x)/x = (x - 1)^2/x := by obvious

  have h3 : x + 1/x - 2 = (x - 1)^2/x := by obvious

  have h4 : (x - 1)^2/x ≥ 0 := by obvious

  have h5 : x + 1/x - 2 ≥ 0 := by obvious

  obvious

end Example5
