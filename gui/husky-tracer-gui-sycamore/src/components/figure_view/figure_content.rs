use super::*;

#[derive(Prop)]
pub struct FigureContentProps {
    data: FigureContentData,
}

#[component]
pub fn FigureContent<'a, G: Html>(scope: Scope<'a>, props: FigureContentProps) -> View<G> {
    todo!()
}
