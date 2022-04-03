<script lang="ts">
    import type { TokenProps, LineProps } from "src/trace/Trace";
    import Token from "./Token.svelte";

    export let on_group_start_click: () => void;
    export let has_subtraces: boolean;
    export let expanded: boolean;
    export let active: boolean;

    export let line: LineProps;
    export let extra_tokens: TokenProps[];
</script>

<p>
    <span class="indent" style="padding-left: {line.indent * 9.5}px" />
    {#if line.idx === 0}
        <span
            class="GroupStart"
            class:has_subtraces
            class:expanded
            on:click={on_group_start_click}
            >&#x25b6
        </span>
    {:else}
        <span class="GroupStartPlaceholder" />
    {/if}
    {#each line.tokens as token}
        <Token {token} within_active_node={active} />
    {/each}
    {#each extra_tokens as token}
        <Token {token} within_active_node={active} />
    {/each}
</p>

<style>
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
    .GroupStartPlaceholder {
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
