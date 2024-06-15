open Printf
let say_hello name = printf "Hello, %s!\n" name

let boolAttention f qs ks vs = List.map (fun q -> f (
  List.combine ks vs |> List.filter (fun (k, _) -> q k) |> List.map (fun (_, v) -> v)
)) qs

let floatAttention (f: (float * 'v) list -> 'o) qs ks vs = List.map (fun q -> f (
  List.combine ks vs |> List.map (fun (k, v) -> (q k, v))
)) qs