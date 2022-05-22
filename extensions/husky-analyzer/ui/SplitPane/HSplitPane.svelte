<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    let separator: any;
    export let updateCallback = () => {
        // do nothing
        return;
    };
    let md: any;
    const onMouseDown = (e: any) => {
        e.preventDefault();
        if (e.button !== 0) return;
        md = {
            e,
            offsetLeft: separator.offsetLeft,
            offsetTop: separator.offsetTop,
            firstWidth: left.offsetWidth,
            secondWidth: right.offsetWidth,
        };
        window.addEventListener("mousemove", onMouseMove);
        window.addEventListener("mouseup", onMouseUp);
        window.addEventListener("touchmove", onMouseMove);
        window.addEventListener("touchend", onMouseUp);
    };
    const onMouseMove = (e: any) => {
        e.preventDefault();
        if (e.button !== 0) return;
        var delta = {
            x: e.clientX - md.e.clientX,
            y: e.clientY - md.e.clientY,
        };
        // Prevent negative-sized elements
        delta.x = Math.min(Math.max(delta.x, -md.firstWidth), md.secondWidth);
        separator.style.left = md.offsetLeft + delta.x + "px";
        left.style.width = md.firstWidth + delta.x + "px";
        right.style.width = md.secondWidth - delta.x + "px";
        updateCallback();
    };
    const onMouseUp = (e?: any) => {
        if (e) {
            e.preventDefault();
            if (e.button !== 0) return;
        }
        updateCallback();
        window.removeEventListener("mousemove", onMouseMove);
        window.removeEventListener("mouseup", onMouseUp);
        window.removeEventListener("touchmove", onMouseMove);
        window.removeEventListener("touchend", onMouseUp);
    };
    function resetSize() {
        if (left) left.removeAttribute("style");
        if (right) right.removeAttribute("style");
        if (separator) separator.removeAttribute("style");
    }
    function onResize() {
        onMouseUp();
        resetSize();
    }
    onMount(() => {
        window.addEventListener("resize", onResize);
    });
    onDestroy(() => {
        window.removeEventListener("resize", onResize);
    });

    let left: any, right: any;
    export let leftPaneSize = "50%";
    export let rightPaneSize = "50%";
    export let minLeftPaneSize = "0";
    export let minRightPaneSize = "0";
    $: leftPaneSize && resetSize();
    $: rightPaneSize && resetSize();
</script>

<div
    class="wrapper"
    style="--left-panel-size: {leftPaneSize}; --right-panel-size: {rightPaneSize}; --min-left-panel-size: {minLeftPaneSize}; --min-right-panel-size: {minRightPaneSize};"
>
    <div bind:this={left} class="left">
        <slot name="left">
            <div style="background-color: red;">Left Contents goes here...</div>
        </slot>
    </div>
    <div
        bind:this={separator}
        class="separator disable-select"
        on:mousedown={onMouseDown}
        on:touchstart={onMouseDown}
    />
    <div bind:this={right} class="right">
        <slot name="right">
            <div style="background-color: yellow;">
                Right Contents goes here...
            </div>
        </slot>
    </div>
</div>

<style>
    div.wrapper {
        width: 100%;
        height: 100%;
        display: inline-flex;
    }
    div.separator {
        cursor: col-resize;
        height: 100%;
        width: 2px;
        margin-left: -2px;
        z-index: 1;
        background-color: rgb(78, 78, 78);
    }
    div.left {
        width: var(--left-panel-size);
        min-width: var(--min-left-panel-size);
        height: 100%;
    }
    div.right {
        width: var(--right-panel-size);
        min-width: var(--min-right-panel-size);
        height: 100%;
    }
</style>
