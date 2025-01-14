import Mathlib
syntax "attack" : tactic

macro_rules
| `(tactic| attack) => `(tactic|
  first
  | simp; done
  | ring; done
  | ring_nf; rw [Real.sq_sqrt]; ring; all_goals attack; done
  | nlinarith; done
  | apply sq_nonneg; all_goals attack; done
  | apply div_nonneg; all_goals attack; done
  | field_simp; ring; done
  | linarith; done
)

macro "obvious": tactic =>`(tactic|
  first
  | attack; done
  | congr; all_goals attack; done
  | gcongr; all_goals attack; done
  | fail "Could not prove this goal automatically. It might not be as obvious as you think!"
)

macro "in_set" : term => `(true)

def h (x y : ℝ) : x ^ 2 + y ^ 2 ≥ (2 : ℝ) * x * y := by
  first
  | have h1 : (x - y) ^ 2 ≥ (0 : ℝ) := by calc
    (x - y) ^ 2 = x ^ 2 - (2 : ℝ) * x * y + y ^ 2 := by obvious
    _ ≥ (0 : ℝ) := by obvious
  | have h2 : x ^ 2 - (2 : ℝ) * x * y + y ^ 2 ≥ (0 : ℝ) := by calc
    x ^ 2 - (2 : ℝ) * x * y + y ^ 2 = (x - y) ^ 2 := by obvious
    _ ≥ (0 : ℝ) := by obvious
  have h3 : x ^ 2 + y ^ 2 ≥ (2 : ℝ) * x * y := by obvious
  obvious
