<script lang="ts">
  import { writeText } from "@tauri-apps/api/clipboard";

  export let outstr = "5";
  export let outtok = "";

  function copy_output() {
    writeText(outstr);
  }

  function copy_token() {
    writeText(outtok);
  }
</script>

<div class="center">
  {#if outstr.length > 0}
    <table>
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
    font-family: "Ubuntu Bold", sans-serif;
    font-size: 18px;
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

    padding: 7px;
    width: 70px;

    background-color: #152530;
    cursor: pointer;

    transition: all 0.3s ease-in-out;
  }

  .copy-btn:hover {
    background-color: #1a2f3d;
  }
</style>
