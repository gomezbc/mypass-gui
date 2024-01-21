<script lang="ts">
  import { Button, Spinner } from "flowbite-svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  let uri: String = "";
  let resMsg: String = "";
  let connecting: boolean = false;
  let connectBtnMsg: String = "Connect";

  function connect() {
    connecting = true;
    connectBtnMsg = "Connecting...";
    invoke("connect_to_db", { inputUri: uri }).then((res) => {
      resMsg = res as String;
      if (resMsg == "Connection successful")
        window.location.href = "/dashboard";
      connecting = false;
      connectBtnMsg = "Connect";
    });
  }
</script>

<div
  class="w-8/12 max-w-[700px] mt-0 bg-white border border-gray-200 rounded-xl shadow-sm dark:bg-gray-800 dark:border-gray-700"
>
  <div class="p-6 space-y-4 md:space-y-6 sm:p-8">
    <h1
      class="text-xl font-bold leading-tight tracking-tight md:text-2xl text-gray-800 dark:text-white"
    >
      Paste your mongodb URI
    </h1>
    <div class="space-y-4 md:space-y-6">
      <div>
        <label
          for="uri"
          class="block mb-2 text-sm font-medium text-black dark:text-white"
          >Your URI</label
        >
        <input
          type="text"
          name="uri"
          id="uri"
          class="py-3 px-4 block w-full border-gray-200 border-2 rounded-lg text-sm text-black/70 focus:border-blue-500 focus:ring-blue-500 disabled:opacity-50 disabled:pointer-events-none dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400 dark:focus:ring-gray-600"
          placeholder="mongodb://[username]:[password]@[host]:[port]/"
          bind:value={uri}
          required={true}
        />
      </div>
      <div class="flex justify-center">
        <Button
          color="blue"
          class="w-full mx-2"
          type="button"
          on:click={connect}
        >
          {#if connecting}
            <Spinner class="me-3" size="4" color="white" />
          {/if}
          {connectBtnMsg}
        </Button>
      </div>
      <p class="text-center h-2 text-black/70 dark:text-white font-semibold">
        {resMsg}
      </p>
    </div>
  </div>
</div>
