<script lang="ts">
    export let view: string;
    import { Button } from "../../node_modules/sveltestrap";
</script>

<main>
    <h1>Thanks for the feedback!</h1>
    <Button
        block
        on:click={() => {
            view = "main";
        }}>Back</Button
    >
</main>
