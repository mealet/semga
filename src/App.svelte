<script lang="ts">
  import "./app.css";

  import { getVersion } from "@tauri-apps/api/app";
  import { onMount } from "svelte";

  import Input from "./modules/Input.svelte";
  import Output from "./modules/Output.svelte";

  let version = "0.0.0";
  let sem_config = {
    input: "",
    token: "",
    mode: "",
  };
  let sem_activated = false;

  let output_string = "";
  let output_token = "";

  onMount(async () => {
    // fetching current app version
    version = await getVersion();
  });
</script>

<main>
  <div class="center center-text">
    <p class="title">Semga <span>v{version}</span></p>
    <p class="subtitle">String Encryption Method</p>
    <Input
      bind:config={sem_config}
      bind:activated={sem_activated}
      bind:outstr={output_string}
      bind:outtok={output_token}
    ></Input>

    <br />

    <Output></Output>
  </div>
</main>
