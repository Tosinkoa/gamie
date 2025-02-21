<!-- AuthGuard.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { userStore } from "$lib/stores/user";
  import { authService } from "$lib/services/auth";

  export let redirectTo = "/login";
  let isLoading = true;

  async function checkAuthentication() {
    try {
      await authService.checkAuth();
    } catch (error) {
      console.error("Authentication check failed:", error);
      userStore.logout();
      goto(redirectTo);
    }
  }

  onMount(async () => {
    try {
      isLoading = true;
      await checkAuthentication();
    } finally {
      isLoading = false;
    }
  });

  // Immediately redirect if no user is present
  $: if (!isLoading && !$userStore.user) {
    goto(redirectTo);
  }
</script>

<div class="min-h-screen bg-gradient-to-br from-gray-800 to-black">
  {#if isLoading}
    <div class="min-h-screen w-full flex items-center justify-center">
      <div
        class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500"
      ></div>
    </div>
  {:else if $userStore.user}
    <slot />
  {/if}
</div>
