mod agency;
mod connection;
mod direction;
mod instrument;
mod origin;
mod place;
mod reason;
mod time;

pub use self::agency::*;
pub use self::connection::*;
pub use self::direction::*;
pub use self::instrument::*;
pub use self::origin::*;
pub use self::place::*;
pub use self::reason::*;
pub use self::time::*;

/// preposition in English
#[derive(Debug, PartialEq, Eq)]
pub enum Preposition {
    Time(TimePreposition),
    Place(PlacePreposition),
    Direction(DirectionPreposition),
    Agency(AgencyPreposition),
    Instrument(InstrumentPreposition),
    Reason(ReasonPreposition),
    Connection(ConnectionPreposition),
    Origin(OriginPreposition),
}
