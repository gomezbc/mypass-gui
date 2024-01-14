<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
  
    let uri :String = '';
    let resMsg :String = 'asd';
  
    function connect() {
        invoke('connect_to_db', { inputUri : uri }).then((res) => { 
            console.log(res)
            
        })
    }
</script>

<div
      class="w-full bg-gray-800 rounded-lg shadow border md:mt-0 sm:max-w-md xl:p-0 border-gray-700"
    >
      <div class="p-6 space-y-4 md:space-y-6 sm:p-8">
        <h1
          class="text-xl font-bold leading-tight tracking-tight md:text-2xl text-white"
        >
          Paste your mongodb URI
        </h1>
        <div class="space-y-4 md:space-y-6">
          <div>
            <label for="uri" class="block mb-2 text-sm font-medium text-white"
              >Your URI</label
            >
            <input
              type="text"
              name="uri"
              id="uri"
              class="border sm:text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500"
              placeholder="localhost"
              bind:value={uri}
              required={true}
            />
          </div>
          <div class="flex justify-center">
            <button
              class="cursor-pointer h-10 transition-all bg-sky-600 text-white px-6 py-2 rounded-lg border-blue-600 border-b-[4px] hover:brightness-110 active:border-b-[2px] active:brightness-90 active:translate-y-[2px]"
              on:click={connect}
            >
              Connect
            </button>
          </div>
          <p class="text-center h-2">{resMsg}</p>
        </div>
      </div>
    </div>