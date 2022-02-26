<script lang="ts">
    import { get_shown_store, get_trace_future } from "src/state/client";

    import type { TokenProps } from "src/trace/Trace";
    import { request_toggle_show } from "src/websocket/websocket_client";
    export let token: TokenProps;
    export let within_active_node: boolean;
    $: spacesBeforeStyles = spacesStyle(countSpacesBefore(token.value));
    $: spacesAfterStyles = spacesStyle(countSpacesAfter(token.value));
    $: associated = token.associated_trace !== null;
    $: associated_trace_store =
        token.associated_trace !== null
            ? get_trace_future(token.associated_trace)
            : null;
    $: associated_trace =
        associated_trace_store !== null ? $associated_trace_store : null;
    $: associated_trace_shown_store =
        associated_trace !== null ? get_shown_store(associated_trace) : null;
    $: associated_trace_shown =
        associated_trace_shown_store !== null
            ? $associated_trace_shown_store
            : false;

    function countSpacesBefore(text: string): number {
        for (let i = 0; i < text.length; i++) {
            if (text.charAt(i) != " ") {
                return i;
            }
        }
        return text.length;
    }
    function countSpacesAfter(text: string): number {
        for (let i = 0; i < text.length; i++) {
            if (text.charAt(text.length - 1 - i) != " ") {
                return i;
            }
        }
        return 0;
    }
    function spacesStyle(n: number): string {
        const rawStyle = {
            width: `${n * 9.5}px`,
        };
        return Object.entries(rawStyle)
            .map(([key, value]) => `${key}:${value}`)
            .join(";");
    }
    function handleClick() {
        if (within_active_node) {
            if (token.associated_trace !== null) {
                request_toggle_show(token.associated_trace);
            }
        }
    }
</script>

<span style={spacesBeforeStyles} />
<code
    class={token.kind}
    class:associated
    class:associated_trace_shown
    on:click={handleClick}
>
    {token.value}
</code>
<span style={spacesAfterStyles} />

<style>
    code {
        font-size: 16px;
    }
    code.associated_trace_shown {
        text-decoration: underline cyan;
    }
    code.associated:hover {
        background: rgb(78, 78, 78);
    }
    .keyword {
        color: rgb(28, 142, 195);
    }
    .label {
        color: rgb(28, 142, 195);
    }
    .special {
        color: white;
    }
    .scope {
        color: rgb(189, 189, 105);
    }
    .literal {
        color: rgb(18, 155, 75);
    }
    .fade {
        color: rgb(97, 97, 97);
    }
    .error {
        color: red;
    }
    .ident {
        color: rgb(135, 164, 167);
    }
</style>
