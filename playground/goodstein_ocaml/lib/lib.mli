type base_repr = {
  is_non_negative : bool;
  base : int;
  digits_reversed : int list;
}

val calc_base_repr : int -> int -> base_repr
val calc_num_from_digits : int -> int list -> int
