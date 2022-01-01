<script>
    import TraceView from "./TraceView.svelte";
</script>

<div class="DebuggerUI">
    <TraceView />
    <div>graphics</div>
</div>

<style>
    .DebuggerUI {
        height: 1000px;
        display: flex;
        flex-direction: row;
    }
</style>
