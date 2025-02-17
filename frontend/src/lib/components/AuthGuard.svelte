<!-- AuthGuard.svelte -->
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { userStore, isAuthenticated } from "$lib/stores/user";
  import { authService } from "$lib/services/auth";

  export let redirectTo = "/login";
  let isLoading = true;

  async function checkAuthentication() {
    try {
      await authService.checkAuth();
    } catch (error) {
      if (error instanceof Error && error.message === "Token refresh failed") {
        console.error("Token refresh failed:", error);
        goto(redirectTo);
      } else {
        // Try to refresh the token
        try {
          await authService.refreshToken();
        } catch (refreshError) {
          console.error("Authentication failed after token refresh:", refreshError);
          goto(redirectTo);
        }
      }
    }
  }

  onMount(async () => {
    try {
      isLoading = true;
      await checkAuthentication();
    } catch (error) {
      console.error("Authentication check failed:", error);
      goto(redirectTo);
    } finally {
      isLoading = false;
    }
  });

  $: if (!isLoading && !$userStore.user) {
    console.log("No user found, redirecting to:", redirectTo);
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
  {:else if $isAuthenticated}
    <slot />
  {/if}
</div>
