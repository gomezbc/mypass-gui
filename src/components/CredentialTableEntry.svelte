<script lang="ts">
  import DeleteModal from "./DeleteModal.svelte";
  import Copied from "./icons/Copied.svelte";
  import Copy from "./icons/Copy.svelte";
  import Hide from "./icons/Hide.svelte";
  import Show from "./icons/Show.svelte";
  import { type Credential } from "@/types/Credential";
  import type { Login } from "@/types/Login";
  import { State } from "@/enums/State";
  import { sharedStateStore } from "@/stores/sharedStateStore";
  import { invoke } from "@tauri-apps/api/tauri";
  import EditCredentialModal from "./EditCredentialModal.svelte";

  enum passwordToooltip {
    show = "Show password",
    hide = "Hide password",
  }

  enum copyPasswordToooltip {
    copy = "Copy to clipboard",
    copied = "Copied!",
  }

  export let domain: string;
  export let credentials: Credential[];

  let copyPasswordTooltip: string = copyPasswordToooltip.copy;
  let showPasswordTooltip: string = passwordToooltip.show;

  let shownCredentials = credentials;

  function updateShownCredentials() {
    shownCredentials = credentials;
    shownCredentials = shownCredentials.map((credential) => {
      return {
        ...credential,
        pass: "********",
      };
    });
  }

  updateShownCredentials();

  let hidden: boolean = true;

  function showPassword(credential: Credential) {
    hidden = !hidden;
    if (hidden) {
      showPasswordTooltip = passwordToooltip.show;
    } else {
      showPasswordTooltip = passwordToooltip.hide;
    }
    let index = shownCredentials.findIndex(
      //!TODO fix this, if you have two credentials with the same email and username, it will show the password of the first one
      (shownCredential) => shownCredential.id === credential.id
    );
    if (hidden) {
      shownCredentials[index].pass = "********";
    } else {
      shownCredentials[index].pass = credentials[index].pass;
    }
  }

  function copyPassword(credential: Credential) {
    let index = shownCredentials.findIndex(
      (shownCredential) => shownCredential.id === credential.id
    );
    navigator.clipboard.writeText(credentials[index].pass);
    copyPasswordTooltip = copyPasswordToooltip.copied;
  }

  async function filterCredentials(event: CustomEvent) {
    let filter: Credential = event.detail as Credential;
    let login: Login = {
      domain: domain,
      credentials: [filter],
    };

    await invoke("delete_login", { login: login }).catch((err) => {
      console.log(err);
    });

    credentials = credentials.filter(
      //!TODO fix this, if you have two credentials with the same email and username, it will delete both
      (credential) => credential.id != filter.id
    );

    sharedStateStore.set(State.DELETE);

    updateShownCredentials();
  }

  function getPlainCredential(credential: Credential): Credential {
    let cred = credentials.find((cred) => cred.id === credential.id);
    if (cred != undefined) {
      return cred;
    }
    return credential;
  }
</script>

{#each shownCredentials as credential}
  <tr>
    <td class="h-px w-px whitespace-nowrap">
      <div class="ps-6 lg:ps-3 xl:ps-3 pe-6 py-3">
        <div class="flex items-center gap-x-3">
          <div class="grow">
            <span
              class="block uppercase text-sm font-semibold text-gray-800 dark:text-gray-200"
              >{domain}</span
            >
          </div>
        </div>
      </div>
    </td>
    <td class="h-px w-72 whitespace-nowrap">
      <div class="px-6 py-3">
        <span
          class="block text-sm font-semibold text-gray-800 dark:text-gray-200"
          >{credential.usr}</span
        >
        <span class="block text-sm text-gray-500">{credential.email}</span>
      </div>
    </td>
    <td class="h-px w-px whitespace-nowrap">
      <div
        class="px-6 py-3 inline-flex items-center text-black/70 dark:text-white"
      >
        {credential.pass}
        <div>
          <span class="group relative">
            <div
              class="absolute bottom-[calc(100%+0.5rem)] left-[50%] -translate-x-[50%] hidden group-hover:block w-auto"
            >
              <div
                class="bottom-full right-0 rounded bg-black px-4 py-1 text-sm text-white dark:bg-gray-200 dark:text-black/70 whitespace-nowrap"
              >
                {showPasswordTooltip}
                <svg
                  class="absolute left-0 top-full h-2 w-full text-black dark:text-gray-200"
                  x="0px"
                  y="0px"
                  viewBox="0 0 255 255"
                  xml:space="preserve"
                  ><polygon
                    class="fill-current"
                    points="0,0 127.5,127.5 255,0"
                  /></svg
                >
              </div>
            </div>
            <button
              on:click={() => showPassword(credential)}
              class="ml-2 text-black/70 dark:text-white"
            >
              {#if showPasswordTooltip === passwordToooltip.show}
                <Show className="size-6" />
              {:else}
                <Hide className="size-6" />
              {/if}
            </button>
          </span>
        </div>
        <div>
          <span class="group relative">
            <div
              class="absolute bottom-[calc(100%+0.5rem)] left-[50%] -translate-x-[50%] hidden group-hover:block w-auto"
            >
              <div
                class="bottom-full right-0 rounded bg-black px-4 py-1 text-sm text-white dark:bg-gray-200 dark:text-black/70 whitespace-nowrap"
              >
                {copyPasswordTooltip}
                <svg
                  class="absolute left-0 top-full h-2 w-full text-black dark:text-gray-200"
                  x="0px"
                  y="0px"
                  viewBox="0 0 255 255"
                  xml:space="preserve"
                  ><polygon
                    class="fill-current"
                    points="0,0 127.5,127.5 255,0"
                  /></svg
                >
              </div>
            </div>
            <button
              on:click={() => copyPassword(credential)}
              on:mouseout={() =>
                (copyPasswordTooltip = copyPasswordToooltip.copy)}
              on:blur={() => (copyPasswordTooltip = copyPasswordToooltip.copy)}
              class="ml-2 text-black/70 dark:text-white"
            >
              {#if copyPasswordTooltip === copyPasswordToooltip.copy}
                <Copy className="size-6" />
              {:else}
                <Copied className="size-6" />
              {/if}
            </button>
          </span>
        </div>
      </div>
    </td>
    <td class="h-px w-px whitespace-nowrap">
      <div class="px-6 py-1.5">
        <span class="mr-2">
          <DeleteModal {credential} on:delete={filterCredentials} />
        </span>
        <span>
          <EditCredentialModal credential={getPlainCredential(credential)} {domain} />
        </span>
      </div>
    </td>
  </tr>
{/each}
