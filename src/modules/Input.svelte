<script lang="ts">
  import { invoke } from "@tauri-apps/api";

  export let config = {
    input: "",
    token: "",
    mode: "encrypt",
  };

  export let outstr = "";
  export let outtok = "";
  export let activated = false;

  let input_field = "";
  let token_field = "";

  let error_str = "";

  function writeConfig(mode: string) {
    // writing config
    config = {
      input: input_field,
      token: token_field,
      mode: mode,
    };

    // checking config
    invoke("check_config", config)
      .then((_) => {
        activated = true;
      })
      .catch((e) => {
        error_str = "Error: " + e;
      });
  }

  function generateToken() {
    invoke("generate_token").then((token) => {
      token_field = token;
    });
  }
</script>

<div class="center">
  <form>
    <p class="state-text">Waiting for you'r input:</p>
    <input
      type="text"
      class="input-area"
      placeholder="Message"
      bind:value={input_field}
      required
    />
    <p></p>
    <div class="token-space">
      <input
        type="text"
        class="input-area token-field"
        placeholder="Token (optional)"
        bind:value={token_field}
      />
      <button on:click|preventDefault={generateToken}>Generate</button>
    </div>
    <p class="error-text">{error_str}</p>
    <div class="btns">
      <input
        type="button"
        value="Encrypt"
        class="confirm-btn"
        on:click|preventDefault={() => writeConfig("encrypt")}
      />
      <input
        type="button"
        value="Decrypt"
        class="confirm-btn"
        on:click|preventDefault={() => writeConfig("decrypt")}
      />
    </div>
  </form>
</div>

<style>
  :root {
    user-select: none;
  }

  .center {
    left: 50%;
    transform: translate(-50%, 0);
    align-content: center;
    justify-content: center;
    text-align: center;
    display: block;
  }

  .state-text {
    position: relative;
    left: 25px;

    width: 320px;

    font-family: "Ubuntu Medium", sans-serif;
    font-size: 28px;
    text-align: center;
  }

  .error-text {
    width: 400px;
    color: #e65a50;

    font-family: "Ubuntu Regular", sans-serif;
  }

  .input-area {
    display: block;
    padding: 15px;

    position: relative;
    bottom: 5px;

    width: 370px;
    height: 40px;

    font-family: "Ubuntu Medium", sans-serif;
    font-size: 20px;

    color: #ffffff;
    background-color: #0e181f;

    box-sizing: border-box;
    border: 0.5px solid #1c3140;
    border-radius: 5px;
  }

  .input-area:focus {
    outline: none;
  }

  .btns {
    display: block;
    position: absolute;

    width: 340px;
    height: 40px;

    padding: 5px;

    left: 50%;
    transform: translate(-50%, 0);
  }

  .btns > input {
    padding: 5px;
  }

  .confirm-btn {
    width: 120px;
    height: 38px;

    border: none;
    border-radius: 4px;

    user-select: none;
    cursor: pointer;

    font-family: "Ubuntu Medium", sans-serif;
    font-size: 16px;

    padding: 5px;
    text-transform: uppercase;
    background-color: #152530;
    transition: all 0.3s ease-in-out;
  }

  .confirm-btn:hover {
    background-color: #1a2f3d;
  }

  .token-space {
    display: flex;
    padding: 5px;
  }

  .token-space > button {
    position: relative;
    bottom: 5px;
    left: 10px;

    height: 40px;
    width: 75px;

    text-align: center;

    font-size: 14px;
    font-family: Arial, Helvetica, sans-serif;
    font-weight: 500;

    border: 0.5px solid #1c3140;
    border-radius: 4px;

    background-color: #152530;

    cursor: pointer;
    transition: all 0.3s ease-in-out;
  }

  .token-space > button:hover {
    background-color: #1a2f3d;
  }

  .token-field {
    width: 285px;
  }
</style>
