// inductive Animal
//   | Cat (weight: Weight)
//   | Dog (weight: Weight)

type Weight = i32;

enum Animal {
    Cat { weight: Weight },
}

#[test]
fn test_cat() {
    let cat = Animal::Cat { weight: 25 };
}
