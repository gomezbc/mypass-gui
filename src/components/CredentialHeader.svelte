<script lang="ts">
  import { sharedStateStore } from "@/stores/sharedStateStore";
  import NewCredentialModal from "./NewCredentialModal.svelte";
  import Refresh from "./icons/Refresh.svelte";
  import { State } from "@/enums/State";
  export let disableButtons: boolean;
</script>

<div
  class="px-6 py-4 grid gap-3 md:flex md:justify-between md:items-center border-b border-gray-200 dark:border-gray-700"
>
  <div>
    <h2 class="text-xl font-semibold text-gray-800 dark:text-gray-200">
      Credentials
    </h2>
    <p class="text-sm text-gray-600 dark:text-gray-400">
      Add credentials, edit and more.
    </p>
  </div>

  <div>
    <div class="inline-flex gap-x-2">
      <!-- svelte-ignore a11y-invalid-attribute -->
      <button
        class="py-2 px-3 inline-flex items-center gap-x-2 text-sm font-medium rounded-lg border border-gray-200 bg-white text-gray-800 shadow-sm hover:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none dark:bg-slate-900 dark:border-gray-700 dark:text-white dark:hover:bg-gray-800 dark:focus:outline-none dark:focus:ring-1 dark:focus:ring-gray-600"
        disabled={disableButtons}
        on:click={() => {
          sharedStateStore.set(State.REFRESH);
        }}
      >
        <Refresh className="flex-shrink-0 size-4" />
        <span>Refresh</span>
      </button>
      <NewCredentialModal {disableButtons} />
    </div>
  </div>
</div>
