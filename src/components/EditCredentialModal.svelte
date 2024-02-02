<script lang="ts">
  import { State } from "../enums/State";
  import { sharedStateStore } from "../stores/sharedStateStore";
  import type { Login } from "../types/Login";
  import { invoke } from "@tauri-apps/api";
  import { Button, Modal, Label, Input, type InputType } from "flowbite-svelte";
  import type { Credential } from "../types/Credential";
  import Edit from "./icons/Edit.svelte";

  let formModal = false;
  export let credential: Credential;
  export let domain: string;
  let passwordFieldType: InputType = "password";
  let passwordFieldShowHide: string = "Show";

  async function handleSubmit(event: Event) {
    event.preventDefault();
    let login: Login = {
      domain: domain,
      credentials: [
        credential
      ],
    };
    console.log(login);
    invoke("update_login", { login: login })
      .then((result) => {
        formModal = false;
        sharedStateStore.set(State.UPDATE);
      })
      .catch((err) => {
        console.log(err);
      });
  }

  function tooglePasswordVision() {
    passwordFieldType = passwordFieldType === "password" ? "text" : "password";
    passwordFieldShowHide = passwordFieldShowHide === "Show" ? "Hide" : "Show";
  }
</script>

<Button
  color="blue"
  on:click={() => (formModal = true)}
  class="px-3 py-2"
>
  <Edit className="size-5" />
</Button>

<Modal bind:open={formModal} size="xs" autoclose={false} class="w-full">
  <form class="flex flex-col space-y-6" on:submit={handleSubmit}>
    <h3 class="mb-1 text-xl font-medium text-gray-900 dark:text-white">
      Update credential
    </h3>
    <Label class="space-y-2">
      <span>Domain</span>
      <Input type="text" name="domain" placeholder="example.com" bind:value={domain} required disabled/>
    </Label>
    <Label class="space-y-2">
      <span>Email</span>
      <Input
        type="email"
        name="email"
        placeholder="name@company.com"
        bind:value={credential.email}
        required
      />
    </Label>
    <Label class="space-y-2">
      <span>Username</span>
      <Input type="text" name="username" placeholder="username" bind:value={credential.usr} required />
    </Label>
    <Label class="space-y-2">
      <span>Your password</span>
      <div class="relative w-full">
        <Input
          type={passwordFieldType}
          class="block w-full pe-20"
          name="password"
          placeholder="••••••••"
          bind:value={credential.pass}
          required
        />
        <div class="absolute inset-y-0 end-0 flex items-center">
          <button
            class="z-10 px-3 h-full w-full border rounded-e-lg border-gray-300 dark:border-gray-600 bg-white hover:bg-gray-50 dark:bg-gray-700 dark:hover:bg-gray-800"
            type="button"
            on:click={tooglePasswordVision}
          >
            <span class="font-semibold text-lg">{passwordFieldShowHide}</span>
          </button>
        </div>
      </div>
    </Label>
    <Button color="blue" type="submit" class="w-full">Update Credential</Button>
  </form>
</Modal>
