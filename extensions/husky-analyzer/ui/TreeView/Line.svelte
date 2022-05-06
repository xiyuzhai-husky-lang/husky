<script lang="ts">
    import type Trace from "src/trace/Trace";
    import type { TokenProps, LineProps } from "src/trace/Trace";
    import Token from "./Token.svelte";

    export let trace: Trace;
    export let on_group_start_click: () => void;
    export let has_subtraces: boolean;
    export let expanded: boolean;
    export let active: boolean;

    export let line: LineProps;
    export let extra_tokens: TokenProps[];
    $: group_start_kind = get_group_start_kind(trace);

    function get_group_start_kind(trace: Trace): string {
        switch (trace.kind) {
            case "Main":
            case "FeatureStmt":
            case "FeatureBranch":
            case "FeatureExpr":
            case "FuncStmt":
            case "ProcStmt":
            case "LoopFrame":
                return "vscode";
            default:
                return "\u25b6";
        }
    }
</script>

<p>
    <span class="indent" style="padding-left: {line.indent * 9.5}px" />
    {#if line.idx === 0}
        <span
            class={`GroupStart ${trace.kind}`}
            class:has_subtraces
            class:expanded
            on:mousedown={on_group_start_click}
        >
            {#if group_start_kind == "vscode"}
                <svg
                    stroke="currentColor"
                    fill="currentColor"
                    stroke-width="0"
                    viewBox="0 0 16 16"
                    height="1.7em"
                    width="2em"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path
                        fill-rule="evenodd"
                        clip-rule="evenodd"
                        d="M10.072 8.024L5.715 3.667l.618-.62L11 7.716v.618L6.333 13l-.618-.619 4.357-4.357z"
                    />
                </svg>
            {:else}
                {group_start_kind}
            {/if}
        </span>
    {:else}
        <span class="GroupStart" />
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
    span {
        display: flex;
        align-items: center;
    }
    .GroupStart {
        display: flex;
        transition: transform 50ms;
        font-size: 0px;
        text-align: center;
        justify-content: center;
        align-items: center;
        width: 20px;
    }
    .GroupStart:not(:hover) {
        color: transparent;
    }
    .GroupStart.has_subtraces {
        font-size: 10px;
        text-align: center;
    }
    .GroupStart.has_subtraces.expanded {
        transform: rotate(90deg);
    }
</style>
