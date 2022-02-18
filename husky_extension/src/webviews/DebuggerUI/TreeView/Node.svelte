<script lang="ts">
    import { Trace } from "src/trace/Trace";
    import Token from "./Token.svelte";
    export let trace: Trace;
    export let on_click: () => void;
    export let on_double_click: () => void;
    export let on_indent_click: () => void;
    export let on_group_start_click: () => void;
    export let has_subtraces: boolean;
    export let expanded: boolean;
    export let active: boolean;
</script>

<div
    class="TraceNode disable-select"
    on:click={on_click}
    on:dblclick={on_double_click}
    lang="ts"
>
    <div class="inner" class:active>
        <p>
            <span
                class="indent"
                style="padding-left: {trace.indent * 9.5}px"
                on:click={on_indent_click}
            />
            <span
                class="GroupStart"
                class:has_subtraces
                class:expanded
                on:click={on_group_start_click}>&#x25b6</span
            >
            {#each trace.tokens as token}
                <Token {token} within_active_node={active} />
            {/each}
        </p>
    </div>
</div>

<style>
    .TraceNode {
        height: 20px;
        padding: 2px;
        padding-right: 3px;
    }
    .inner {
        padding: 0px;
    }
    .active {
        outline: rgb(202, 132, 2) solid 2px;
    }
    p {
        height: 100%;
        margin: 0;
        padding: 0;
        cursor: default;
        display: flex;
        justify-content: flex-start;
    }
    .GroupStart {
        /* cursor: pointer; */
        display: flex;
        transition: transform 50ms;
        font-size: 0px;
        text-align: center;
        justify-content: center;
        align-items: center;
        width: 18px;
    }
    .GroupStart.has_subtraces {
        font-size: 10px;
        text-align: center;
    }
    .GroupStart.has_subtraces.expanded {
        transform: rotate(90deg);
    }
</style>
