import Mathlib

-- success
example (x : ℝ) (h : x > 0) : 1/x + 1 = (x+1)/x := by
  field_simp; ring

-- unsolved goals
-- x : ℝ
-- h : x > 0
-- ⊢ 1 + x⁻¹ = x * x⁻¹ + x⁻¹Lean 4
example (x : ℝ) (h : x > 0) : 1/x + 1 = (x+1)/x := by
  ring <;> (field_simp ;ring)

-- unsolved goals
-- x : ℝ
-- h : x > 0
-- ⊢ 1 + x⁻¹ = x * x⁻¹ + x⁻¹Lean 4
example (x : ℝ) (h : x > 0) : 1/x + 1 = (x+1)/x := by
  first | ring | (field_simp ;ring)
