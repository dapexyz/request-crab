<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let url = $state("https://httpbin.org/get");
  let method = $state("GET");
  let response = $state("");
  let loading = $state(false);

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
