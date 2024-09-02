<script lang="ts">
  import { writeText } from "@tauri-apps/api/clipboard";
  import { fade } from "svelte/transition";

  export let outstr = "";
  export let outtok = "";

  let old_str = "";

  function copy_output() {
    writeText(outstr);
  }

  function copy_token() {
    writeText(outtok);
  }
</script>

<div class="center">
  {#if outstr.length > 0}
    <table transition:fade={{ duration: 250 }}>
      <tr>
        <th>Output</th>
        <th
          ><button class="copy-btn" on:click|preventDefault={copy_output}
            >Copy</button
          ></th
        >
      </tr>
      <tr>
        <th>Token</th>
        <th
          ><button class="copy-btn" on:click|preventDefault={copy_token}
            >Copy</button
          ></th
        >
      </tr>
    </table>
  {/if}
</div>

<style>
  .center {
    width: 400px;
    height: 200px;

    position: absolute;
    transform: translate(-50%, 0);
    top: 420px;

    align-items: center;
    justify-content: center;
    text-align: center;
  }

  .center {
    font-family: "Ubuntu Medium", sans-serif;
    font-size: 22px;
  }

  .center > table {
    position: absolute;

    left: 50%;
    transform: translate(-50%, 0);

    table-layout: fixed;
    color: white;
    width: 50%;
  }

  .copy-btn {
    border: none;
    border-radius: 4px;

    padding: 9px;
    width: 70px;

    font-size: 18px;

    background-color: #152530;
    cursor: pointer;

    transition: all 0.3s ease-in-out;
  }

  .copy-btn:hover {
    background-color: #1a2f3d;
  }

  .copy-btn:active {
    background-color: #000000;
  }
</style>
