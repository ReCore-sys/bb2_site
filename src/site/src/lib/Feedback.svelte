<script lang="ts">
    import { Styles, Form, FormGroup, Label, Input, Button } from "sveltestrap";

    import { post } from "./api";
    import * as jq from "jquery";

    async function Submit(a: SubmitEvent) {
        a.preventDefault();
        var formData = new FormData(jq("form")[0]);
        await post("/api/feedback/", formData.get("text"));
        view = "thanks";
    }

    export let view: string;
</script>

<form
    class="form-feedback"
    on:submit={async (e) => {
        await Submit(e);
    }}
>
    <FormGroup>
        <label for="exampleEmail"><h2>Username</h2></label>
        <Input name="username" />

        <label for="exampleText" class="label"
            ><h2>What do you think?</h2></label
        >
        <Input type="textarea" name="feedback" id="exampleText" />

        <label for="exampleText" class="label"
            ><h2>Any ideas or suggestions?</h2></label
        >
        <Input type="textarea" name="ideas" id="exampleText" />
        <Button block class="submit-feedback">Submit</Button>
    </FormGroup>
</form>

<style>
    .label {
        margin: 5% 0;
    }

    form {
        padding: 50px;
    }
</style>
