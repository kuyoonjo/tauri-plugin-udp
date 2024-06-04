<script>
  import * as udp from "@kuyoonjo/tauri-plugin-udp";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  let local = "0.0.0.0:8080";
  let remote = "255.255.255.255:9090";
  let message = "hello";

  async function bind() {
    try {
      await udp.bind("xxx", local, true);
    } catch (e) {
      console.log({ e });
    }
  }

  async function unbind() {
    try {
      await udp.unbind("xxx");
    } catch (e) {
      console.log({ e });
    }
  }

  async function send() {
    try {
      await udp.send("xxx", remote, message);
    } catch (e) {
      console.log({ e });
    }
  }

  onMount(() => {
    listen("plugin://udp", (x) => console.log(x.payload));
  });
</script>

<main class="container">
  <div class="row">
    <input placeholder="e.g. 0.0.0.0:8080" bind:value={local} />
    <button on:click={bind}> Bind </button>
    <button on:click={unbind}> Unbind </button>
  </div>
  <div class="row">
    <input placeholder="e.g. 255.255.255.255:9090" bind:value={remote} />
  </div>
  <div class="row">
    <input placeholder="e.g. hello" bind:value={message} />
    <button on:click={send}> Send </button>
  </div>
</main>
