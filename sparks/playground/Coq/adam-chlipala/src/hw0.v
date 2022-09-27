Check (1 + 1 = 2).

Inductive day : Type :=
  | monday
  | tuesday
  | wednesday
  | thursday
  | friday
  | saturday
  | sunday.

Eval compute in (43+55).

Inductive Nat : Type :=
  | Zero
  | Succ : Nat -> Nat.

Lemma haha : forall n m : Nat, Nat.Succ n = m -> Nat.Succ (Nat.Succ n) = Nat.Succ m.
Proof.
sorry.
Qed.