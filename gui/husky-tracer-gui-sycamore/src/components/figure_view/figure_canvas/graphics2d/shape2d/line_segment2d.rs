use super::*;

#[derive(Prop)]
pub struct LineSegment2dProps {
    start: Point2dData,
    end: Point2dData,
}

impl LineSegment2dProps {
    fn transform(&self) -> String {
        let angle = self.start.to(self.end).angle_degrees();
        format!(
            "translate({} {}) rotate({})",
            self.start.x, self.start.y, angle
        )
    }

    fn points(&self) -> String {
        let lineWidth = 1.0;
        let length = 1.0;
        let arrowHeadHeight = 1.0;
        let gamma = 1.0;
        [
            (0.0, (-lineWidth / 2.0)),
            ((length - arrowHeadHeight) / 2.0, -lineWidth / 2.0),
            ((length - arrowHeadHeight) / 2.0, -gamma * lineWidth),
            ((length + arrowHeadHeight) / 2.0, -lineWidth / 2.0),
            (length, -lineWidth / 2.0),
            (length, lineWidth / 2.0),
            ((length + arrowHeadHeight) / 2.0, lineWidth / 2.0),
            ((length - arrowHeadHeight) / 2.0, gamma * lineWidth),
            ((length - arrowHeadHeight) / 2.0, lineWidth / 2.0),
            (0.0, (lineWidth / 2.0)),
        ]
        .into_iter()
        .map(|(from, to)| format!("{from},{to}"))
        .collect::<Vec<_>>()
        .join(" ")
    }
}

#[component]
pub fn LineSegment2d<'a, G: Html>(scope: Scope<'a>, props: LineSegment2dProps) -> View<G> {
    let transform = props.transform();
    let points = props.points();
    view! {
        scope,
        g (transform=transform) {
            polygon (points=points)
        }
    }
}
