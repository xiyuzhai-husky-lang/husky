
type  base_repr = { is_non_negative: bool; base:int; digits_reversed: int list; }
  [@@deriving show]

let rec calc_base_repr a base =
  if a >=0 then
    if a < base then  {is_non_negative=true; base; digits_reversed= [a]} else
      let x = a / base in
      let y = a mod base in
      let x_base_repr = calc_base_repr x base in
      {is_non_negative=true; base; digits_reversed= (y::x_base_repr.digits_reversed)}
  else {(calc_base_repr (-a) base) with is_non_negative = false}

let rec calc_num_from_digits base digits =
  match digits with
  | [] -> 0
  | digit::digits -> base * (calc_num_from_digits base digits) + digit

(* let () = print_endline (string_of_int (calc_num_from_digits 10 [1;1;1])) *)

let%test "calc_num_from_digits1" =
  Int.equal (calc_num_from_digits 10 [1;1;1]) 11

let%test "rev2" =
  List.equal Int.equal (List.rev [ 3; 2; 1 ]) [ 1; 2; 3 ]
