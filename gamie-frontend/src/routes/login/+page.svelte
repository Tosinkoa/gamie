<script lang="ts">
  import Icon from "@iconify/svelte";
  import { goto } from "$app/navigation";
  import RedirectGuard from "$lib/components/RedirectGuard.svelte";
  import { authService } from "$lib/services/auth";

  let usernameOrEmail = "";
  let password = "";
  let loading = false;
  let error = "";
  let success = false;
  let showPassword = false;

  async function handleSubmit(e: Event) {
    e.preventDefault();
    error = "";
    loading = true;

    try {
      await authService.login({
        username_or_email: usernameOrEmail,
        password,
      });

      success = true;
      setTimeout(() => {
        goto("/profile");
      }, 1000);
    } catch (err) {
      if (err instanceof TypeError && err.message === "Failed to fetch") {
        error = "Unable to connect to the server. Please check your internet connection.";
      } else {
        error = err instanceof Error ? err.message : "An error occurred during login";
      }
    } finally {
      loading = false;
    }
  }
</script>

<RedirectGuard>
  <div
    class="h-[calc(100dvh-4rem)] flex items-center justify-center bg-gradient-to-br from-gray-800 to-black px-4 sm:px-6 lg:px-8"
  >
    <div class="bg-gray-800 p-6 sm:p-8 rounded-lg shadow-xl w-full max-w-md relative">
      <a href="/" class="absolute top-4 left-4 text-sm text-blue-400 hover:underline"
        >&larr; Back to Home</a
      >
      <h2 class="text-2xl sm:text-3xl font-bold text-white text-center mt-8 mb-6">
        Login to Your Account
      </h2>

      {#if error}
        <div
          class="mb-4 p-3 bg-red-500/10 border border-red-500/20 rounded text-red-400 text-sm"
        >
          {error}
        </div>
      {/if}

      {#if success}
        <div
          class="mb-4 p-3 bg-green-500/10 border border-green-500/20 rounded text-green-400 text-sm"
        >
          Login successful! Redirecting...
        </div>
      {/if}

      <button
        class="w-full py-2 mb-4 bg-gray-700 hover:bg-gray-600 text-white rounded-md transition flex items-center justify-center disabled:opacity-50 disabled:cursor-not-allowed"
        disabled={loading}
      >
        <Icon icon="logos:google-gmail" class="h-5 w-5 mr-2" />
        Continue with Gmail
      </button>

      <div class="flex items-center mb-4">
        <hr class="flex-grow border-gray-600" />
        <span class="mx-2 text-gray-400">or</span>
        <hr class="flex-grow border-gray-600" />
      </div>

      <form on:submit={handleSubmit}>
        <div class="mb-4">
          <label class="block text-gray-300 mb-1 text-sm sm:text-base" for="usernameOrEmail"
            >Username or Email</label
          >
          <input
            id="usernameOrEmail"
            type="text"
            bind:value={usernameOrEmail}
            placeholder="Your username or email"
            class="w-full px-3 sm:px-4 py-2 bg-gray-700 border border-gray-600 rounded focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm sm:text-base disabled:opacity-50"
            required
            disabled={loading}
          />
        </div>

        <div class="mb-6">
          <label class="block text-gray-300 mb-1 text-sm sm:text-base" for="password"
            >Password</label
          >
          <div class="relative">
            <input
              id="password"
              type={showPassword ? "text" : "password"}
              bind:value={password}
              placeholder="Your password"
              class="w-full px-3 sm:px-4 py-2 bg-gray-700 border border-gray-600 rounded focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm sm:text-base disabled:opacity-50"
              required
              disabled={loading}
            />
            <button
              type="button"
              class="absolute right-2 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-300"
              on:click={() => (showPassword = !showPassword)}
            >
              <Icon icon={showPassword ? "ph:eye-slash" : "ph:eye"} class="w-5 h-5" />
            </button>
          </div>
        </div>

        <button
          type="submit"
          class="w-full py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition flex items-center justify-center disabled:opacity-50 disabled:cursor-not-allowed"
          disabled={loading}
        >
          {#if loading}
            <Icon icon="ph:circle-notch" class="w-5 h-5 mr-2 animate-spin" />
            Logging in...
          {:else}
            Login
          {/if}
        </button>
      </form>

      <p class="mt-4 text-center text-gray-300 text-sm sm:text-base">
        Don't have an account?
        <a href="/signup" class="text-blue-400 hover:underline">Sign Up</a>
      </p>
    </div>
  </div>
</RedirectGuard>
