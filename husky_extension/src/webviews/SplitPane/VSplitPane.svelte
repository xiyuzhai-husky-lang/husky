<script>
    import { onMount, onDestroy } from 'svelte';
    let separator;
    export let updateCallback = () => {
        // do nothing
        return;
    }

    let md;
    const onMouseDown = (e) => {
        e.preventDefault();
        if (e.button !== 0) return;
        md = {e,
            offsetLeft:  separator.offsetLeft,
            offsetTop:   separator.offsetTop,
            firstHeight:  top.offsetHeight,
            secondHeight: down.offsetHeight
        };
        window.addEventListener('mousemove', onMouseMove);
        window.addEventListener('mouseup', onMouseUp);
        window.addEventListener('touchmove', onMouseMove);
        window.addEventListener('touchend', onMouseUp);
    };
    const onMouseMove = (e) => {
        e.preventDefault();
        if (e.button !== 0) return;
        var delta = {x: e.clientX - md.e.clientX,
                y: e.clientY - md.e.clientY};
        // Prevent negative-sized elements
        delta.y = Math.min(Math.max(delta.y, -md.firstHeight),
                    md.secondHeight);

        separator.style.top = md.offsetTop + delta.y + "px";
        top.style.height = (md.firstHeight + delta.y) + "px";
        down.style.height = (md.secondHeight - delta.y) + "px";
        updateCallback();
    }
    const onMouseUp = (e) => {
        if (e) {
            e.preventDefault();
            if (e.button !== 0) return;
        }
        updateCallback();
        window.removeEventListener('mousemove', onMouseMove);
        window.removeEventListener('mouseup', onMouseUp);
        window.removeEventListener('touchmove', onMouseMove);
        window.removeEventListener('touchend', onMouseUp);
    }
    function resetSize() {
        if (top) top.removeAttribute('style');
        if (down) down.removeAttribute('style');
        if (separator) separator.removeAttribute('style');
    }
    function onResize() {
        onMouseUp();
        resetSize();
    }
    onMount(() => {
        window.addEventListener('resize', onResize);
    });
    onDestroy(() => {
        window.removeEventListener('resize', onResize);
    });
    let top, down;
    export let topPanelSize = '50%';
    export let downPanelSize = '50%';
    export let minTopPaneSize = '0';
    export let minDownPaneSize = '0';
    $: topPanelSize && resetSize();
    $: downPanelSize && resetSize();
</script>

<style>
    div.wrapper {
        width: 100%;
        height: 100%;
        /* background-color: yellow; */
        display: flex;
        flex-direction: column;
    }
    div.separator {
        cursor: row-resize;
        width: 100%;
        height: 4px;
        margin-top: -2px;
        z-index: 1;
        background-color: #aaa;
    }
    div.top {
        height: var(--top-panel-size);
        min-height: var(--min-top-panel-size);
        width: 100%;
    }
    div.down {
        height: var(--down-panel-size);
        min-height: var(--min-down-panel-size);
        width: 100%;
    }
</style>

<div class="wrapper" style="--top-panel-size: {topPanelSize}; --down-panel-size: {downPanelSize}; --min-top-panel-size:{minTopPaneSize}; --min-down-panel-size: {minDownPaneSize};">
    <div bind:this={top} class="top">
        <slot name="top">
            <div style="background-color: red;">
                Top Contents goes here...
            </div>
        </slot>
    </div>
    <div bind:this={separator} class="separator" on:mousedown={onMouseDown} on:touchstart={onMouseDown}>
    </div>
    <div bind:this={down} class="down">
        <slot name="down">
            <div style="background-color: yellow;">
                Down Contents goes here...
            </div>
        </slot>
    </div>
</div>
