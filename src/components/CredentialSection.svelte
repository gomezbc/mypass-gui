<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import CredentialFooter from "./CredentialFooter.svelte";
  import CredentialHeader from "./CredentialHeader.svelte";
  import CredentialTable from "./CredentialTable.svelte";

  import { type Login } from "@/types/Login";
  import CredentialUnlock from "./CredentialUnlock.svelte";
  import { sharedStateStore } from "@/stores/sharedStateStore";
  import { State } from "@/enums/State";

  let isLocked: boolean = true;

  let logins: Login[] = [];
  let numOfcredentialsInLogin = (login: Login) => login.credentials.length;
  let loginsNum: number = 0;
  let isLoading = false;

  async function loadLogins() {
    isLoading = true;
    logins = (await invoke("get_logins")) as Login[];
    loginsNum = logins.reduce(
      (acc, login) => acc + numOfcredentialsInLogin(login),
      0
    );
    isLoading = false;
  }

  sharedStateStore.subscribe(async (value) => {
    switch (value) {
      case State.RELOAD:
        isLocked = false;
        await loadLogins();
        break;
      case State.UNLOCKED:
        isLocked = false;
        await loadLogins();
        break;
      case State.LOCKED:
        isLocked = true;
        logins = [];
        break;
      default:
        break;
    }
  });
</script>

<!-- Table Section -->
<div class="max-w-[85rem] px-4 py-10 sm:px-6 lg:px-8 lg:py-14 mx-auto">
  <!-- Card -->
  <div class="flex flex-col">
    <div class="-m-1.5 overflow-x-auto">
      <div class="p-1.5 min-w-full inline-block align-middle">
        <div
          class="bg-white border border-gray-200 rounded-xl shadow-sm overflow-hidden dark:bg-slate-900 dark:border-gray-700"
        >
          {#if isLocked}
            <CredentialHeader disableButtons={true} />
            <CredentialUnlock />
            <CredentialFooter loginNum={0} />
          {:else}
            <CredentialHeader disableButtons={false} />
            <CredentialTable {logins} />
            <CredentialFooter loginNum={loginsNum} />
          {/if}
        </div>
      </div>
    </div>
  </div>
</div>
