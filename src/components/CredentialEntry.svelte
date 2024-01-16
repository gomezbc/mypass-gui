<script lang="ts">
  import { slide } from "svelte/transition";
  import ArrowDown from "@/components/icons/arrowDown.svelte";

  export let domain: string;
  export let email: string;
  export let username: string;
  export let password: string;

  let isOpen = false;

  function toggleDropdown() {
    isOpen = !isOpen;
  }
</script>

<li>
  <button class="flex items-center" on:click={toggleDropdown}>
    <ArrowDown className="size-3" />
    <span class="ml-2">{domain}</span>
  </button>
  {#if isOpen}
    <div transition:slide={{ duration: 200 }}>
      <span>{email}</span><br />
      <span>{username}</span><br />
      <span>{password}</span>
    </div>
  {/if}
</li>
