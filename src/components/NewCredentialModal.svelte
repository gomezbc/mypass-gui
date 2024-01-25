<script lang="ts">
  import { State } from "@/enums/State";
  import { sharedStateStore } from "@/stores/sharedStateStore";
  import type { Login } from "@/types/Login";
  import { invoke } from "@tauri-apps/api";
  import { Button, Modal, Label, Input, type InputType } from "flowbite-svelte";
  import { v4 as uuidv4 } from "uuid";
  import Dice from "./icons/Dice.svelte";

  let formModal = false;
  let password: String;
  export let disableButtons: boolean;
  let passwordFieldType: InputType = "password";
  let passwordFieldShowHide: string = "Show";

  async function handleSubmit(event: Event) {
    event.preventDefault();
    let login: Login = {
      domain: (event.target as HTMLFormElement).domain.value,
      credentials: [
        {
          id: uuidv4(),
          email: (event.target as HTMLFormElement).email.value,
          usr: (event.target as HTMLFormElement).username.value,
          pass: (event.target as HTMLFormElement).password.value,
        },
      ],
    };
    invoke("add_login", { login: login })
      .then((result) => {
        formModal = false;
        sharedStateStore.set(State.ADD);
      })
      .catch((err) => {
        console.log(err);
      });
  }

  function tooglePasswordVision() {
    passwordFieldType = passwordFieldType === "password" ? "text" : "password";
    passwordFieldShowHide = passwordFieldShowHide === "Show" ? "Hide" : "Show";
  }

  function generatePassword() {
    var length = 16,
      charset =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~",
      retVal = "";
    for (var i = 0, n = charset.length; i < length; ++i) {
      retVal += charset.charAt(Math.floor(Math.random() * n));
    }
    password = retVal;
  }
</script>

<Button
  color="blue"
  on:click={() => (formModal = true)}
  disabled={disableButtons}
>
  <svg
    class="flex-shrink-0 w-3 h-3"
    xmlns="http://www.w3.org/2000/svg"
    width="16"
    height="16"
    viewBox="0 0 16 16"
    fill="none"
  >
    <path
      d="M2.63452 7.50001L13.6345 7.5M8.13452 13V2"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
    />
  </svg> <span class="ml-2">Add Credential</span>
</Button>

<Modal bind:open={formModal} size="xs" autoclose={false} class="w-full">
  <form class="flex flex-col space-y-6" on:submit={handleSubmit}>
    <h3 class="mb-4 text-xl font-medium text-gray-900 dark:text-white">
      Add new credential
    </h3>
    <Label class="space-y-2">
      <span>Domain</span>
      <Input type="text" name="domain" placeholder="example.com" required />
    </Label>
    <Label class="space-y-2">
      <span>Email</span>
      <Input
        type="email"
        name="email"
        placeholder="name@company.com"
        required
      />
    </Label>
    <Label class="space-y-2">
      <span>Username</span>
      <Input type="text" name="username" placeholder="username" required />
    </Label>
    <Label class="space-y-2">
      <span>Your password</span>
      <div class="relative w-full">
        <Input
          type={passwordFieldType}
          class="block w-full pe-20"
          name="password"
          placeholder="••••••••"
          bind:value={password}
          required
        />
        <div class="absolute inset-y-0 end-0 flex items-center">
          <button
            class="z-10 px-3 h-full w-full border-l border-y border-gray-300 dark:border-gray-600 bg-white hover:bg-gray-50 dark:bg-gray-700 dark:hover:bg-gray-800"
            type="button"
            on:click={tooglePasswordVision}
          >
            <span class="font-semibold text-lg">{passwordFieldShowHide}</span>
          </button>
          <button
            class="z-10 p-3 h-full w-full rounded-e-lg border border-gray-300 dark:border-gray-600 bg-white hover:bg-gray-50 dark:bg-gray-700 dark:hover:bg-gray-800"
            type="button"
            on:click={generatePassword}
          >
            <Dice className="size-5" />
          </button>
        </div>
      </div>
    </Label>
    <Button color="blue" type="submit" class="w-full">Add Credential</Button>
  </form>
</Modal>
