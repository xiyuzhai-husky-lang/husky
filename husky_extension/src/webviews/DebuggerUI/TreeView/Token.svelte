<script>
    export let token;
    let spaces_before =
        token.spaces_before === undefined ? 1 : token.spaces_before;
    let style = {
        width: `${spaces_before * 9.5}px`,
    };
    $: cssVarStyles = Object.entries(style)
        .map(([key, value]) => `${key}:${value}`)
        .join(";");
</script>

<span style={cssVarStyles} />
{#if token.type == "special"}
    <code class="special">{token.value} </code>
{:else if token.type == "scope"}
    <code class="scope">{token.value}</code>
{:else if token.type == "keyword"}
    <code class="keyword">{token.value}</code>
{:else if token.type == "ident"}
    <code class="ident">{token.value}</code>
{:else if token.type == "literal"}
    <code class="literal">{token.value}</code>
{:else if token.type == "fade"}
    <code class="fade">{token.value}</code>
{:else}
    <code class="error">`unrecognized token type: {token.type}`</code>
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
        color: rgb(12, 75, 148);
        /* color: cyan; */
    }
    .fade {
        color: rgb(97, 97, 97);
    }
    .ident {
        color: rgb(135, 164, 167);
    }
</style>
