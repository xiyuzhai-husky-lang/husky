theory First
  imports Main
begin

lemma "1 + 1 = 2"
  by simp

(* Define a simple function that adds 1 to a natural number *)
definition inc :: "nat ⇒ nat" where
  "inc n = n + 1"
declare inc_def[simp]

(* A simple lemma showing that inc of 0 equals 1 *)
lemma inc_zero: "inc 0 = 1" by simp

(* A slightly more complex lemma showing that inc is always positive *)
lemma inc_positive: "inc n > 0"
  using inc_def by force

end
