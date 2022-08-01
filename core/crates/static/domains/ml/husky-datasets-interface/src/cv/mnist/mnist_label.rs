#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MnistLabel {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<usize> for MnistLabel {
    fn from(raw: usize) -> Self {
        match raw {
            0 => MnistLabel::Zero,
            1 => MnistLabel::One,
            2 => MnistLabel::Two,
            3 => MnistLabel::Three,
            4 => MnistLabel::Four,
            5 => MnistLabel::Five,
            6 => MnistLabel::Six,
            7 => MnistLabel::Seven,
            8 => MnistLabel::Eight,
            9 => MnistLabel::Nine,
            _ => panic!(),
        }
    }
}
