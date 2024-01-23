<script>
  import { Button, Modal } from "flowbite-svelte";
  import Trash from "./icons/Trash.svelte";
  import Warning from "./icons/Warning.svelte";
  let popupModal = false;
</script>

<button
  class="inline-flex items-center px-3 py-2 bg-red-600 hover:bg-red-700 text-white font-semibold rounded-md"
  on:click={() => (popupModal = true)}
>
  <Trash className="size-5"/>
</button>

<Modal bind:open={popupModal} size="sm" autoclose>
  <div class="text-center">
      <Warning className="mx-auto mb-4 text-gray-400 w-12 h-12 dark:text-gray-200" />
    <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
      Are you sure you want to delete this login credential?
    </h3>
    <Button color="red" class="me-2">Yes, I'm sure</Button>
    <Button color="dark" class="border hover:bg-current/50">No, cancel</Button>
  </div>
</Modal>
