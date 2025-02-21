<!-- RedirectGuard.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { isAuthenticated, userStore } from "$lib/stores/user";
  import { authService } from "$lib/services/auth";

  export let redirectTo = "/profile";
  let isLoading = true;

  onMount(async () => {
    try {
      await authService.checkAuth();
      if ($userStore.user) {
        goto(redirectTo);
      }
    } finally {
      isLoading = false;
    }
  });
</script>

{#if !$userStore.user}
  {#if isLoading}
    <div
      class="h-[calc(100dvh-4rem)] flex items-center justify-center bg-gradient-to-br from-gray-800 to-black"
    >
      <div
        class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500"
      ></div>
    </div>
  {:else}
    <slot />
  {/if}
{/if}
