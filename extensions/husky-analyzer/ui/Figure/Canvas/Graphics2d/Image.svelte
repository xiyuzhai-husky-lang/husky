<script lang="ts">
    import { onMount } from "svelte";
    import type ImageProps from "src/figsrc/figure/ImagePropsimport { ImageLoader } from "srcsrc/figure/ImageProps    import type Focus from "src/data/Focus";
    import { focus_store } from "src/data/ui";

    export let image_props: ImageProps;
    export let image_height: number;
    export let image_width: number;
    $: focus = $focus_store;
    $: draw(canvas, image_props, image_height, image_width);

    let canvas: any;

    function draw(
        canvas: any,
        image_props: ImageProps,
        image_height: number,
        image_width: number
    ) {
        if (canvas === undefined) {
            return;
        }
        let ctx = canvas.getContext("2d");
        const imageData = ctx.getImageData(0, 0, image_width, image_height);
        let image_loader = new ImageLoader(image_props);

        const original_height = image_loader.height();
        const original_width = image_loader.width();

        for (let p = 0; p < imageData.data.length; p += 4) {
            const q = p / 4;
            const x = q % image_width;
            const y = (q / image_width) >>> 0;

            const i = Math.floor((y / image_height) * original_height);
            const j = Math.floor((x / image_width) * original_width);
            imageData.data[p + 0] = image_loader.r(i, j);
            imageData.data[p + 1] = image_loader.g(i, j);
            imageData.data[p + 2] = image_loader.b(i, j);
            imageData.data[p + 3] = 255;
        }

        ctx.putImageData(imageData, 0, 0);
    }
</script>

<canvas bind:this={canvas} width={image_width} height={image_height} />

<style>
    canvas {
        background-color: #666;
    }
</style>
