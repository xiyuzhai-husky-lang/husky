pub enum Thing {
    MathFormula { text: String },
    Dog { weight: f32, bark: fn() -> () },
    Cat { weight: f32 },
    GameCard(GameCard),
}

pub struct GameCard {
    price: f32,
}

fn describe_a_thing(thing: Thing) -> String {
    match thing {
        Thing::MathFormula { text } => format!("math formulat with text \"{}\"", text),
        Thing::Dog { weight, .. } => todo!(),
        Thing::GameCard(card) => format!("game card with price \"{}\"", card.price),
        _ => todo!(),
    }
}

impl Thing {
    fn weight(&self) -> Option<f32> {
        match self {
            Thing::Dog { weight, .. } | Thing::Cat { weight } => Some(*weight),
            _ => None,
        }
    }
}
