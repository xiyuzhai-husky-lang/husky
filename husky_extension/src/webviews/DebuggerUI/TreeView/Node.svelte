<script lang="ts">
    import Token from "./Token.svelte";
    export let trace: any;
    export let onClick: () => void;
    export let onDoubleClick: () => void;
    export let hasChildren: boolean;
    export let expanded: boolean;
    export let activeTrace: boolean;

    // function onClick() {
    //     if (validSingleClick) {
    //         validSingleClick = false;
    //     } else {
    //         validSingleClick = true;
    //         setTimeout(() => {
    //             if (validSingleClick) {
    //                 validSingleClick = false;
    //                 onSingleClick();
    //             }
    //         }, 200);
    //     }
    // }
</script>

<div
    class="TraceNode disable-select"
    on:click={onClick}
    on:dblclick={onDoubleClick}
    lang="ts"
>
    <div class="inner" class:activeTrace>
        <p>
            <span class="GroupStart" class:hasChildren class:expanded
                >&#x25b6</span
            >
            {#each trace.tokens as token}
                <Token {token} />
            {/each}
        </p>
    </div>
</div>

<style>
    .TraceNode {
        height: 20px;
        padding: 2px;
    }
    .activeTrace {
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
        cursor: pointer;
        display: flex;
        transition: transform 50ms;
        font-size: 0px;
        text-align: center;
        justify-content: center;
        align-items: center;
        width: 20px;
    }
    .GroupStart.hasChildren {
        font-size: 10px;
        text-align: center;
    }
    .GroupStart.hasChildren.expanded {
        transform: rotate(90deg);
    }
</style>
