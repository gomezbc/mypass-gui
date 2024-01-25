<script lang="ts">
  import { State } from "@/enums/State";
  import { sharedStateStore } from "@/stores/sharedStateStore";
  import type { Login } from "@/types/Login";
  import { invoke } from "@tauri-apps/api";
  import { Button, Modal, Label, Input } from "flowbite-svelte";
  import { v4 as uuidv4 } from "uuid";

  let formModal = false;
  export let disableButtons: boolean;

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
      <Input type="password" name="password" placeholder="••••••••" required />
    </Label>
    <Button color="blue" type="submit" class="w-full">Add Credential</Button>
  </form>
</Modal>
