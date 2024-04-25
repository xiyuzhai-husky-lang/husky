let () = print_endline "Hello, World!"

type colour =
  | Red | Green | Blue | Yellow | Black | White
  | RGB of {r : int; g : int; b : int}

type design = { c1: colour; c2: colour}

let a (judge: design) =
  match judge with
  | { c1 = Red | Green; c2 = Red } -> true
  | _ -> true