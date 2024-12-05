import Mathlib

variable {x : ℝ}

/-
Let x ∈ ℝ. Assume x > 0.
-/
def assumption (h : 0 < x) : True := trivial

/-
Then x + 1/x - 2 = (x² + 1 - 2x)/x.
-/
def eq_step1 (h : 0 < x) : x + (1/x) - 2 = (x^2 + 1 - 2*x) / x := by
  -- Since x > 0, we can freely manipulate denominators.
  have hx : x ≠ 0 := ne_of_gt h
  calc
    x + 1/x - 2
    = (x*x)/x + 1/x - 2        := by rw [mul_div_cancel x hx]
    = (x^2 + 1) / x - 2        := by rw [add_div x^2 1 x]
    = (x^2 + 1 - 2*x) / x      := by ring

/-
Then (x² + 1 - 2x)/x = ((x-1)²)/x.
-/
def eq_step2 (h : 0 < x) : (x^2 + 1 - 2*x) / x = ((x - 1)^2) / x := by
  calc
    (x^2 + 1 - 2*x) / x
    = (x^2 - 2*x + 1) / x := by ring
    = ((x - 1)^2) / x     := by rw [←pow_two (x-1)]

/-
Then x + 1/x - 2 = ((x-1)²)/x.
-/
def eq_step3 (h : 0 < x) : x + 1/x - 2 = ((x-1)^2) / x := by
  rw [eq_step1 h, eq_step2 h]

/-
Then ((x-1)²)/x ≥ 0.
-/
def ge_step4 (h : 0 < x) : ((x-1)^2)/x ≥ 0 := by
  apply div_nonneg
  · exact sq_nonneg (x - 1)
  · exact le_of_lt h

/-
Then x + 1/x - 2 ≥ 0.
-/
def ge_step5 (h : 0 < x) : x + 1/x - 2 ≥ 0 := by
  rw [eq_step3 h]
  exact ge_step4 h

/-
Then x + 1/x ≥ 2.
-/
def final_ineq (h : 0 < x) : x + 1/x ≥ 2 := by
  -- Just rewrite the inequality in terms of nonnegativity of x+1/x-2
  rw [← sub_nonneg, eq_step3 h]
  exact ge_step4 h
