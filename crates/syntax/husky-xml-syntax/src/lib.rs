use husky_trace_protocol::VisualData;
use husky_word::{Ident, IdentPairMap};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlValue {
    pub tag_kind: HtmlTagKind,
    pub props: IdentPairMap<Value>,
}

impl Serialize for HtmlValue {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HtmlTagKind {
    Point2d,
    Contour,
    Arrow2d,
    LineSegment,
}

impl HtmlTagKind {
    pub fn code(self) -> &'static str {
        match self {
            HtmlTagKind::Point2d => "Point2d",
            HtmlTagKind::Arrow2d => "Arrow2d",
            HtmlTagKind::Contour => "Contour",
            HtmlTagKind::LineSegment => "LineSegment",
        }
    }

    pub fn from_ident(_ident: Ident) -> Self {
        todo!()
        // match ident.as_str() {
        //     "Point2d" => HtmlTagKind::Point2d,
        //     "Contour" => HtmlTagKind::Contour,
        //     "Arrow2d" => HtmlTagKind::Arrow2d,
        //     "LineSegment" => HtmlTagKind::LineSegment,
        //     _ => todo!("{}", ident),
        // }
    }
}

impl From<HtmlValue> for VisualData {
    fn from(_val: HtmlValue) -> Self {
        todo!()
        // let mut data = self.props.take_data();
        // msg_once!("ad hoc");
        // match self.tag_kind.as_str() {
        //     "Contour" => {
        //         should_eq!(data.len(), 1);
        //         let (_ident, value) = data.pop().unwrap();
        //         let points: Vec<Point2dData> = serde_json::from_value(value).unwrap();
        //         VisualData::Contour { points }
        //     }
        //     "LineSegment" => {
        //         should_eq!(data.len(), 2);
        //         // end
        //         let (ident, value) = data.pop().unwrap();
        //         should_eq!(ident.as_str(), "end");
        //         let end: Point2dData = serde_json::from_value(value).unwrap();
        //         // start
        //         let (ident, value) = data.pop().unwrap();
        //         should_eq!(ident.as_str(), "start");
        //         let start: Point2dData = serde_json::from_value(value).unwrap();
        //         VisualData::LineSegment { start, end }
        //     }
        //     _ => todo!(),
        // }
    }
}
