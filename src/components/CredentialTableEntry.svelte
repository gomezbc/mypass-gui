<script lang="ts">
  import Show from "./icons/Show.svelte";
  import { type Credential } from "@/types/Credential";

  export let domain: string;
  export let credentials: Credential[];

  let shownCredentials = credentials;

  shownCredentials = shownCredentials.map((credential) => {
    return {
      ...credential,
      pass: "********",
    };
  });

  let hidden: boolean = true;

  function showPassword(credential: Credential) {
    hidden = !hidden;
    let index = shownCredentials.findIndex(
      (shownCredential) =>
        shownCredential.email === credential.email &&
        shownCredential.usr === credential.usr
    );
    if (hidden) {
      shownCredentials[index].pass = "********";
    } else {
      shownCredentials[index].pass = credentials[index].pass;
    }
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
      <div class="px-6 py-3 inline-flex items-center">
        {credential.pass}
        <button
          on:click={() => showPassword(credential)}
          class="ml-2 text-black/70 dark:text-white"
          ><Show className="size-6" /></button
        >
      </div>
    </td>
    <td class="h-px w-px whitespace-nowrap">
      <div class="px-6 py-1.5">
        <a
          class="inline-flex items-center gap-x-1 text-sm text-blue-600 decoration-2 hover:underline font-medium dark:focus:outline-none dark:focus:ring-1 dark:focus:ring-gray-600"
          href="#"
        >
          Edit
        </a>
      </div>
    </td>
  </tr>
{/each}
