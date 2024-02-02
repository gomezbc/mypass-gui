<script lang="ts">
  import CredentialTableEntry from "./CredentialTableEntry.svelte";
  import { type Login } from "../types/Login";
  export let logins: Login[] = [];
</script>

<table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
  <thead class="bg-gray-50 dark:bg-slate-800">
    <tr>
      <th scope="col" class="ps-6 lg:ps-3 xl:ps-3 pe-6 py-3 text-start">
        <div class="flex items-center gap-x-2">
          <span
            class="text-xs font-semibold uppercase tracking-wide text-gray-800 dark:text-gray-200"
          >
            Domain
          </span>
        </div>
      </th>

      <th scope="col" class="px-6 py-3 text-start">
        <div class="flex items-center gap-x-2">
          <span
            class="text-xs font-semibold uppercase tracking-wide text-gray-800 dark:text-gray-200"
          >
            Information
          </span>
        </div>
      </th>

      <th scope="col" class="px-6 py-3 text-start">
        <div class="flex items-center gap-x-2">
          <span
            class="text-xs font-semibold uppercase tracking-wide text-gray-800 dark:text-gray-200"
          >
            Password
          </span>
        </div>
      </th>
      <th scope="col" class="px-6 py-3 text-end"></th>
    </tr>
  </thead>

  <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
    {#each logins as login}
      <CredentialTableEntry
        domain={login.domain}
        credentials={login.credentials}
      />
    {/each}
  </tbody>
</table>
