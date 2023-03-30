/// place preposition in English
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PlacePreposition {
    At,
    In,
    On,
    By,
    Near,
    CloseTo,
    /// something is immediately adjacent to or close to another object, with possibly a slight gap or distance between
    NextTo,
    /// direct contact or closeness with no gap or distance
    Beside,
    Between,
    Behind,
    InFrontOf,
    Above,
    Over,
    /// something is located beneath or under another object, or at a lower point on a vertical axis
    Below,
    Under,
}
