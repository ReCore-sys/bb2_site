<script lang="ts">
    import Counter from "../node_modules/svelte-counter/src";
    import { get, post } from "./lib/api";
    import type { Configs } from "./defs";
    import * as jq from "jquery";
    $: view = "main";
    import {
        Styles,
        Form,
        FormGroup,
        Label,
        Input,
        Button,
    } from "../node_modules/sveltestrap";
    import Feedback from "./lib/feedback.svelte";
    import Thanks from "./lib/Thx.svelte";
    $: users = 0;
    $: counters = {
        users: users,
    };
    async function GetConfig() {
        const response = await get("/api/config/");
        const data = await response.data;
    }

    async function GetUsers() {
        await GetConfig();
        const response = await get(`/api/users/`);
        const fetched_users = response.data.users;

        users = fetched_users.length;
    }
    GetUsers();
</script>

<main>
    {#if view == "main"}
        <h1>Bottombot, a verified crime</h1>
        {#if users != 0}
            <Counter
                values={counters}
                duration="1000"
                random="false"
                minspeed="10"
                let:counterResult
            >
                <div>{counterResult.users} users</div>
            </Counter>
            <Feedback bind:view />
        {/if}
    {/if}
    {#if view == "thanks"}
        <Thanks bind:view />
    {/if}
</main>
