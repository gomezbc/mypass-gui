<script lang="ts">
  import { slide } from "svelte/transition";
  import ArrowDown from "@/components/icons/ArrowDown.svelte";
  import Show from "./icons/Show.svelte";

  type Credential = {
    email: string;
    username: string;
    password: string;
  };

  export let domain: string;
  export let credentials: Credential[];

  let isOpen = false;

  function toggleDropdown() {
    isOpen = !isOpen;
  }
</script>

<li>
  <button
    class="w-full flex items-center bg-[#323638be] rounded-lg px-2 py-1"
    on:click={toggleDropdown}
  >
    <ArrowDown className="size-3" />
    <span class="ml-2">{domain}</span>
  </button>
  {#if isOpen}
    <div class="g-[#202223be]" transition:slide={{ duration: 200 }}>
      <div class="w-full grid grid-cols-3 px-2 pt-1">
        <span class="font-semibold text-sm">Email</span>
        <span class="font-semibold text-sm">Username</span>
        <span class="font-semibold text-sm">Password </span>
      </div>
      <hr />
      {#each credentials as credential}
        <div class="w-full grid grid-cols-3 px-2 py-1">
          <span class="overflow-scroll">{credential.email}</span>
          <span class="overflow-scroll">{credential.username}</span>
          <span class="overflow-scroll flex items-center"
            >{credential.password}
            <button>
              <Show className="size-4 ml-2" />
            </button>
          </span>
        </div>
      {/each}
    </div>
  {/if}
</li>
