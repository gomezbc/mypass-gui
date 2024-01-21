<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import CredentialFooter from "./CredentialFooter.svelte";
  import CredentialHeader from "./CredentialHeader.svelte";
  import CredentialTable from "./CredentialTable.svelte";

  import {type Login} from "@/types/Login";
  import {type Credential} from "@/types/Credential";
  

  async function getLogins(): Promise<Login[]> {
    let logins = (await invoke("get_logins")) as Login[];
    console.log(logins);
    return logins;
  }

  let logins: Login[] = [];
  let isLoading = true;

  getLogins().then((result) => {
    isLoading = true;
    logins = result;
    logins.forEach((login) => {
      login.credentials.forEach((credential) => {
        credential.pass = "•".repeat(10);
      });
    });
    isLoading = false;
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
          <CredentialHeader />
          <CredentialTable logins={logins}/>
          <CredentialFooter loginNum={logins.length} />
        </div>
      </div>
    </div>
  </div>
</div>
