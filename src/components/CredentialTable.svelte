<script>
  import CredentialTableEntry from "./CredentialTableEntry.svelte";
</script>

<table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
  <thead class="bg-gray-50 dark:bg-slate-800">
    <tr>
      <th scope="col" class="ps-6 lg:ps-3 xl:ps-0 pe-6 py-3 text-start">
        <div class="flex items-center gap-x-2">
          <span
            class="text-xs font-semibold uppercase tracking-wide text-gray-800 dark:text-gray-200"
          >
            Name
          </span>
        </div>
      </th>

      <th scope="col" class="px-6 py-3 text-start">
        <div class="flex items-center gap-x-2">
          <span
            class="text-xs font-semibold uppercase tracking-wide text-gray-800 dark:text-gray-200"
          >
            Position
          </span>
        </div>
      </th>

      <th scope="col" class="px-6 py-3 text-start">
        <div class="flex items-center gap-x-2">
          <span
            class="text-xs font-semibold uppercase tracking-wide text-gray-800 dark:text-gray-200"
          >
            Status
          </span>
        </div>
      </th>

      <th scope="col" class="px-6 py-3 text-start">
        <div class="flex items-center gap-x-2">
          <span
            class="text-xs font-semibold uppercase tracking-wide text-gray-800 dark:text-gray-200"
          >
            Portfolio
          </span>
        </div>
      </th>

      <th scope="col" class="px-6 py-3 text-start">
        <div class="flex items-center gap-x-2">
          <span
            class="text-xs font-semibold uppercase tracking-wide text-gray-800 dark:text-gray-200"
          >
            Created
          </span>
        </div>
      </th>

      <th scope="col" class="px-6 py-3 text-end"></th>
    </tr>
  </thead>

  <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
    <CredentialTableEntry />
  </tbody>
</table>
