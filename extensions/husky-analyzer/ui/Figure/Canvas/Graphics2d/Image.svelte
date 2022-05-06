<script lang="ts">
    import { onMount } from "svelte";
    import type ImageProps from "src/trace/figure/ImageProps";
    import { ImageLoader } from "src/trace/figure/ImageProps";
    import type Focus from "src/data/Focus";
    import { focus_store } from "src/data/ui";

    export let image_props: ImageProps;
    $: focus = $focus_store;
    $: draw(canvas, image_props);

    let canvas: any;

    function draw(canvas: any, image_props: ImageProps) {
        if (canvas === undefined) {
            return;
        }
        let ctx = canvas.getContext("2d");
        const imageData = ctx.getImageData(0, 0, 900, 900);
        let image_loader = new ImageLoader(image_props);

        const original_height = image_loader.height();
        const original_width = image_loader.width();

        for (let p = 0; p < imageData.data.length; p += 4) {
            const q = p / 4;
            const x = q % canvas.width;
            const y = (q / canvas.width) >>> 0;

            const i = Math.floor((y / canvas.height) * original_height);
            const j = Math.floor((x / canvas.width) * original_width);
            imageData.data[p + 0] = image_loader.r(i, j);
            imageData.data[p + 1] = image_loader.g(i, j);
            imageData.data[p + 2] = image_loader.b(i, j);
            imageData.data[p + 3] = 255;
        }

        ctx.putImageData(imageData, 0, 0);
    }
</script>

<canvas bind:this={canvas} width="900" height="900" />

<style>
    canvas {
        height: 900px;
        width: 900px;
        background-color: #666;
    }
</style>
