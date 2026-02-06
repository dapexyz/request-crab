<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { check } from "@tauri-apps/plugin-updater";
  import { relaunch } from "@tauri-apps/plugin-process";

  let url = $state("https://httpbin.org/get");
  let method = $state("GET");
  let response = $state("");
  let loading = $state(false);

  let update: Awaited<ReturnType<typeof check>> = $state(null);
  let updating = $state(false);

  async function checkForUpdate() {
    try {
      update = await check();
    } catch (e) {
      console.error("Update check failed:", e);
    }
  }

  async function installUpdate() {
    if (!update) return;
    updating = true;
    await update.downloadAndInstall();
    await relaunch();
  }

  checkForUpdate();

  async function sendRequest() {
    loading = true;
    try {
      const result = await invoke("request", {
        payload: {
          method,
          url,
          headers: [],
          body: null,
        },
      });

      response = JSON.stringify(result, null, 2);
    } catch (e) {
      response = "Error: " + e;
    }
    loading = false;
  }
</script>

<main>
  <h1>Request Crab</h1>

  {#if update}
    <button class="update-btn" onclick={installUpdate} disabled={updating}>
      {updating ? "Updating..." : `Update ${update.version} verfügbar`}
    </button>
  {/if}

  <div class="request-bar">
    <select bind:value={method}>
      <option>GET</option>
      <option>POST</option>
      <option>PUT</option>
      <option>PATCH</option>
      <option>DELETE</option>
      <option>HEAD</option>
      <option>OPTIONS</option>
    </select>

    <input bind:value={url} placeholder="URL" />

    <button onclick={sendRequest} disabled={loading}>
      {loading ? "Loading..." : "Send"}
    </button>
  </div>

  <pre class="response">{response}</pre>
</main>

<style>
</style>
