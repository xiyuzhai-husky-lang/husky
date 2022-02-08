<script lang="ts">
    import { TokenProps } from "src/server/types";

    export let token: TokenProps;
    let spaces_before =
        token.spaces_before === undefined ? 1 : token.spaces_before;
    let style = {
        width: `${spaces_before * 9.5}px`,
    };
    $: spacesBeforeStyles = Object.entries(style)
        .map(([key, value]) => `${key}:${value}`)
        .join(";");
</script>

<span style={spacesBeforeStyles} />
{#if token.kind == "special"}
    <code class="special">{token.value} </code>
{:else if token.kind == "scope"}
    <code class="scope">{token.value}</code>
{:else if token.kind == "keyword"}
    <code class="keyword">{token.value}</code>
{:else if token.kind == "ident"}
    <code class="ident">{token.value}</code>
{:else if token.kind == "literal"}
    <code class="literal">{token.value}</code>
{:else if token.kind == "fade"}
    <code class="fade">{token.value}</code>
{:else}
    <code class="error">`unrecognized token type: {token.kind}`</code>
{/if}

<style>
    code {
        font-size: 16px;
    }
    .keyword {
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
    .ident {
        color: rgb(135, 164, 167);
    }
</style>
