let () = print_endline "Hello, World!"


let x = ref 1

let y = !x + 1

let () =
  let old_x = !x in
  x := 5;
  let y_value = y in
  x := old_x;
  Printf.printf  "y = %d" y_value