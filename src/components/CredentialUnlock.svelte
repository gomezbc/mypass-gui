<script lang="ts">
  import { Button, Label, Input, Spinner } from "flowbite-svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { masterKeyStore } from '@/stores/masterKeyStore';
  import Lock from "./icons/Lock.svelte";
  import Unlock from "./icons/Unlock.svelte";

  let unlocking: boolean = false;
  let unlockBtnMsg: String = unlocking ? "Unlocking..." : "Unlock";

  async function handleSubmit(event: Event) {
    event.preventDefault();
    unlocking = true;
    unlockBtnMsg = "Unlocking...";
    let password = (event.target as HTMLFormElement).password.value;
    console.log(password);
    await invoke("check_master_key", { key: password })
      .then((result) => {
        if (result) {
          masterKeyStore.set(password);
        }
      })
      .catch((err) => {
        console.log(err);
      });
    // handle the response...
    unlocking = false;
    unlockBtnMsg = "Unlock";
  }
</script>

<div class="min-w-full my-10 flex flex-col items-center">
  <Lock className="size-12 text-gray-400 dark:text-gray-500 mx-auto" />
  <h1
    class="w-96 text-2xl font-semibold text-center text-gray-900 dark:text-white mt-4"
  >
    Enter your master key to unlock
  </h1>
  <div class="w-96 text-sm text-center text-gray-500 dark:text-gray-400 mt-2">
    Your master key is used to encrypt and decrypt your credentials.
  </div>
  <form class="w-96 mt-6 flex flex-col" on:submit={handleSubmit}>
    <Label class="space-y-2">
      <span>Your password</span>
      <Input type="password" name="password" required />
    </Label>
    <Button color="blue" type="submit" class="mt-4">
      {#if unlocking}
        <Spinner class="me-3" size="4" color="white" />
      {:else}
        <Unlock className="size-4 mr-2" />
      {/if}
      <span class="text-lg">{unlockBtnMsg}</span></Button
    >
  </form>
</div>
