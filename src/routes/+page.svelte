<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { getVersion } from "@tauri-apps/api/app";
  import { check } from "@tauri-apps/plugin-updater";
  import { relaunch } from "@tauri-apps/plugin-process";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { loadRequests, saveRequest, deleteRequest, type SavedRequest } from "$lib/db";

  getVersion().then((v) => {
    getCurrentWindow().setTitle(`request-crab v${v}`);
  });

  let url = $state("https://httpbin.org/get");
  let method = $state("GET");
  let requestName = $state("");
  let currentRequestId = $state<string | null>(null);
  let response = $state("");
  let loading = $state(false);
  let savedRequests = $state<SavedRequest[]>([]);

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
  refreshSavedRequests();

  async function refreshSavedRequests() {
    savedRequests = await loadRequests();
  }

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

  async function handleSave() {
    const name = requestName.trim() || url;
    const id = currentRequestId || crypto.randomUUID();
    await saveRequest({ id, name, method, url, headers: "[]", body: null });
    currentRequestId = id;
    requestName = name;
    await refreshSavedRequests();
  }

  function selectRequest(req: SavedRequest) {
    currentRequestId = req.id;
    requestName = req.name;
    method = req.method;
    url = req.url;
    response = "";
  }

  async function handleDelete(id: string) {
    await deleteRequest(id);
    if (currentRequestId === id) {
      currentRequestId = null;
      requestName = "";
      method = "GET";
      url = "https://httpbin.org/get";
      response = "";
    }
    await refreshSavedRequests();
  }

  function newRequest() {
    currentRequestId = null;
    requestName = "";
    method = "GET";
    url = "";
    response = "";
  }
</script>

<main>
  <h1>Request Crab</h1>

  {#if update}
    <button class="update-btn" onclick={installUpdate} disabled={updating}>
      {updating ? "Updating..." : `Update ${update.version} verfügbar`}
    </button>
  {/if}

  <fieldset>
    <legend>Saved Requests</legend>
    <button onclick={newRequest}>New</button>
    <ul>
      {#each savedRequests as req}
        <li>
          <button onclick={() => selectRequest(req)}>{req.method} {req.name}</button>
          <button onclick={() => handleDelete(req.id)}>x</button>
        </li>
      {/each}
    </ul>
  </fieldset>

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
    <button onclick={handleSave}>Save</button>
  </div>

  <input bind:value={requestName} placeholder="Request name" />

  <pre class="response">{response}</pre>
</main>

<style>
</style>
