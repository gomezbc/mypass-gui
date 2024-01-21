<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
  
    let uri :String = '';
    let resMsg :String = '';
  
    function connect() {
        resMsg = 'Connecting...';
        invoke('connect_to_db', { inputUri : uri }).then((res) => { 
            resMsg = res as String;
            if (resMsg == 'Connection successful') 
              window.location.href = '/dashboard';
        })
    }
</script>

<div
      class="mt-7 bg-white border border-gray-200 rounded-xl shadow-sm dark:bg-gray-800 dark:border-gray-700"
    >
      <div class="p-6 space-y-4 md:space-y-6 sm:p-8">
        <h1
          class="text-xl font-bold leading-tight tracking-tight md:text-2xl text-gray-800 dark:text-white"
        >
          Paste your mongodb URI
        </h1>
        <div class="space-y-4 md:space-y-6">
          <div>
            <label for="uri" class="block mb-2 text-sm font-medium dark:text-white"
              >Your URI</label
            >
            <input
              type="text"
              name="uri"
              id="uri"
              class="py-3 px-4 block w-full border-gray-200 border-2 rounded-lg text-xs focus:border-blue-500 focus:ring-blue-500 disabled:opacity-50 disabled:pointer-events-none dark:bg-slate-900 dark:border-gray-700 dark:text-gray-400 dark:focus:ring-gray-600"
              placeholder="mongodb://[username]:[password]@[host]:[port]/"
              bind:value={uri}
              required={true}
            />
          </div>
          <div class="flex justify-center">
            <button
            class="w-full py-3 px-4 inline-flex justify-center items-center gap-x-2 text-sm font-semibold rounded-lg border border-transparent bg-blue-600 text-white hover:bg-blue-700 disabled:opacity-50 disabled:pointer-events-none dark:focus:outline-none dark:focus:ring-1 dark:focus:ring-gray-600"
              on:click={connect}
            >
              Connect
            </button>
          </div>
          <p class="text-center h-2 dark:text-white font-semibold">{resMsg}</p>
        </div>
      </div>
    </div>