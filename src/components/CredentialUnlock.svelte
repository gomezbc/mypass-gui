<script lang="ts">
  import { Button, Label, Input, Spinner } from "flowbite-svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import Lock from "./icons/Lock.svelte";
  import Unlock from "./icons/Unlock.svelte";
  import { sharedStateStore } from "../stores/sharedStateStore";
  import { State } from "../enums/State";

  let unlocking: boolean = false;
  let isPasswordCorrect: boolean = true;

  async function handleSubmit(event: Event) {
    event.preventDefault();
    unlocking = true;
    let password = (event.target as HTMLFormElement).password.value;
    await invoke("check_master_key", { key: password })
      .then((result) => {
        if (result === true) {
          isPasswordCorrect = true;
          sharedStateStore.set(State.UNLOCK);
        } else {
          isPasswordCorrect = false;
        }
      })
      .catch((err) => {
        console.log(err);
      });
    unlocking = false;
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
      {#if !isPasswordCorrect}
        <span class="text-red-500">Wrong password</span>
      {/if}
    </Label>
    <Button color="blue" type="submit" class="mt-4">
      {#if unlocking}
        <Spinner class="me-3" size="4" color="white" />
      {:else}
        <Unlock className="size-4 mr-2" />
      {/if}
      <span class="text-lg">{unlocking ? "Unlocking..." : "Unlock"}</span></Button
    >
  </form>
</div>
