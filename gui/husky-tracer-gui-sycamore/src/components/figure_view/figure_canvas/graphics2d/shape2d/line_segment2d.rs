use super::*;

#[derive(Prop)]
pub struct LineSegment2dProps {
    start: Point2dData,
    end: Point2dData,
    line_width: f32,
    fill: String,
}

impl LineSegment2dProps {
    fn transform(&self) -> String {
        let angle = self.start.to(self.end).angle() / 3.1415926 * 180.0;
        format!(
            "translate({} {})\nrotate({})",
            self.start.x, self.start.y, angle
        )
    }

    fn points(&self) -> String {
        let line_width = self.line_width;
        let length = self.start.to(self.end).norm();
        let gamma = 1.7;
        let arrow_head_height = (length / 2.0).min(gamma * line_width);
        [
            (0.0, (-line_width / 2.0)),
            ((length - arrow_head_height) / 2.0, -line_width / 2.0),
            ((length - arrow_head_height) / 2.0, -gamma * line_width),
            ((length + arrow_head_height) / 2.0, -line_width / 2.0),
            (length, -line_width / 2.0),
            (length, line_width / 2.0),
            ((length + arrow_head_height) / 2.0, line_width / 2.0),
            ((length - arrow_head_height) / 2.0, gamma * line_width),
            ((length - arrow_head_height) / 2.0, line_width / 2.0),
            (0.0, (line_width / 2.0)),
        ]
        .into_iter()
        .map(|(from, to)| format!("{from},{to}"))
        .collect::<Vec<_>>()
        .join(" ")
    }
}

#[component]
pub fn LineSegment2d<'a, G: Html>(visibility: Scope<'a>, props: LineSegment2dProps) -> View<G> {
    let transform = props.transform();
    let points = props.points();
    view! {
        visibility,
        g (transform=transform) {
            polygon (
                points=points,
                fill=props.fill
            )
        }
    }
}
