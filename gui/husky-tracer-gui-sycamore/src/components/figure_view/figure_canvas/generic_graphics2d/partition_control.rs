use super::*;

#[derive(Prop)]
pub struct PartitionControlProps<'a> {
    idx: usize,
    dimension: &'a ReadSignal<PixelDimension>,
}

#[component]
pub fn PartitionControl<'a, G: Html>(
    visibility: Scope<'a>,
    props: PartitionControlProps<'a>,
) -> View<G> {
    let ctx = use_dev_context(visibility);
    view! {
        visibility,
        div (
            class="PartitionControl",
            style=props.dimension.cget().to_style(),
        ) {
            div (class = "PartitionControlLeft") {
                div (
                    class = "PartitionControlLeftItem Add",
                    on:click=ctx.add_partition_handler(props.idx)
                ) {
                    svg (
                        stroke="currentColor",
                        fill="currentColor",
                        stroke-width="0",
                        viewBox="0 0 24 24",
                        height="1em",
                        width="1em",
                        xmlns="http://www.w3.org/2000/svg"
                    ) {
                        path (
                            fill="none",
                            stroke="#000",
                            stroke-width="2",
                            d="M12,22 L12,2 M2,12 L22,12"
                        )
                    }
                }
                div (
                    class = "PartitionControlRightItem PartitionShrink",
                    on:click=ctx.shrink_partition_handler(props.idx)
                ) {
                    svg (
                        stroke="currentColor",
                        fill="currentColor",
                        stroke-width="0",
                        viewBox="0 0 16 16",
                        height="1em",
                        width="1em",
                        xmlns="http://www.w3.org/2000/svg"
                    ) {
                        path (
                            fill-rule="evenodd",
                            d="M8.354 1.646a.5.5 0 010 .708L2.707 8l5.647 5.646a.5.5 0 01-.708.708l-6-6a.5.5 0 010-.708l6-6a.5.5 0 01.708 0z",
                            clip-rule="evenodd"
                        )
                        path (
                            fill-rule="evenodd",
                            d="M12.354 1.646a.5.5 0 010 .708L6.707 8l5.647 5.646a.5.5 0 01-.708.708l-6-6a.5.5 0 010-.708l6-6a.5.5 0 01.708 0z",
                            clip-rule="evenodd"
                        )
                    }
                }
                div (
                    class = "PartitionControlRightItem PartitionExpand",
                    on:click=ctx.expand_partition_handler(props.idx)
                ) {
                    svg (
                        stroke="currentColor",
                        fill="currentColor",
                        stroke-width="0",
                        viewBox="0 0 16 16",
                        height="1em",
                        width="1em",
                        xmlns="http://www.w3.org/2000/svg"
                    ) {
                        path (
                            fill-rule="evenodd",
                            d="M3.646 1.646a.5.5 0 01.708 0l6 6a.5.5 0 010 .708l-6 6a.5.5 0 01-.708-.708L9.293 8 3.646 2.354a.5.5 0 010-.708z",
                            clip-rule="evenodd"
                        )
                        path (
                            fill-rule="evenodd",
                            d="M7.646 1.646a.5.5 0 01.708 0l6 6a.5.5 0 010 .708l-6 6a.5.5 0 01-.708-.708L13.293 8 7.646 2.354a.5.5 0 010-.708z",
                            clip-rule="evenodd"
                        )
                    }
                }
            }
            div (class = "PartitionControlRight") {
                div (
                    class = "PartitionControlRightItem PartitionClose",
                    on:click=ctx.remove_partition_handler(props.idx)
                ) {
                    "X"
                }
            }
        }

    }
}
