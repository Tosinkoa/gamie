<script lang="ts">
  import Icon from "@iconify/svelte";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import RedirectGuard from "$lib/components/RedirectGuard.svelte";
  import { authService } from "$lib/services/auth";

  let username = "";
  let email = "";
  let password = "";
  let confirmPassword = "";
  let loading = false;
  let error = "";
  let fieldErrors: { [key: string]: string } = {};
  let success = false;
  let showPassword = false;
  let showConfirmPassword = false;

  // @Todo Add email confirmation for signup

  // Password strength validation
  function validatePasswordStrength(password: string): { valid: boolean; message?: string } {
    if (password.length < 8) {
      return { valid: false, message: "Password must be at least 8 characters long" };
    }

    const hasLowercase = /[a-z]/.test(password);
    const hasUppercase = /[A-Z]/.test(password);
    const hasNumber = /\d/.test(password);
    const hasSpecial = /[@$!%*?&]/.test(password);

    if (!hasLowercase || !hasUppercase || !hasNumber || !hasSpecial) {
      return {
        valid: false,
        message:
          "Password must contain at least one uppercase letter, one lowercase letter, one number, and one special character (@$!%*?&)",
      };
    }

    return { valid: true };
  }

  // Form validation
  function validateForm(): boolean {
    // Username validation
    if (username.length < 3 || username.length > 20) {
      error = "Username must be between 3 and 20 characters";
      return false;
    }

    if (!/^[a-zA-Z0-9_]+$/.test(username)) {
      error = "Username can only contain letters, numbers, and underscores";
      return false;
    }

    // Email validation
    if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
      error = "Please enter a valid email address";
      return false;
    }

    // Password validation
    const passwordCheck = validatePasswordStrength(password);
    if (!passwordCheck.valid) {
      error = passwordCheck.message || "Invalid password";
      return false;
    }

    if (password !== confirmPassword) {
      error = "Passwords do not match";
      return false;
    }

    return true;
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();
    error = "";
    fieldErrors = {};
    loading = true;

    if (!validateForm()) {
      loading = false;
      return;
    }

    try {
      await authService.signup({
        username,
        email: email.toLowerCase(),
        password,
        password_verify: confirmPassword,
      });

      success = true;
      setTimeout(() => {
        goto("/login");
      }, 2000);
    } catch (err) {
      if (err instanceof TypeError && err.message === "Failed to fetch") {
        error = "Unable to connect to the server. Please check your internet connection.";
      } else {
        error = err instanceof Error ? err.message : "An error occurred during signup";
      }
    } finally {
      if (!success) {
        loading = false;
      }
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
        Create Your Account
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
          class="min-h-screen bg-gradient-to-br from-gray-800 to-black flex items-center justify-center"
        >
          <div
            class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500"
          ></div>
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

      <form on:submit|preventDefault={handleSubmit}>
        <div class="mb-4">
          <label class="block text-gray-300 mb-1 text-sm sm:text-base" for="username"
            >Username</label
          >
          <input
            id="username"
            type="text"
            bind:value={username}
            placeholder="Your username"
            class="w-full px-3 sm:px-4 py-2 bg-gray-700 border {fieldErrors.username
              ? 'border-red-500'
              : 'border-gray-600'} rounded focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm sm:text-base disabled:opacity-50"
            required
            disabled={loading}
            minlength="3"
            maxlength="20"
            pattern="[a-zA-Z0-9_]+"
          />
          {#if fieldErrors.username}
            <p class="mt-1 text-xs text-red-400">{fieldErrors.username}</p>
          {/if}
        </div>

        <div class="mb-4">
          <label class="block text-gray-300 mb-1 text-sm sm:text-base" for="email">Email</label
          >
          <input
            id="email"
            type="email"
            bind:value={email}
            on:input={(e) => (email = e.currentTarget.value.toLowerCase())}
            placeholder="Your email"
            class="w-full px-3 sm:px-4 py-2 bg-gray-700 border {fieldErrors.email
              ? 'border-red-500'
              : 'border-gray-600'} rounded focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm sm:text-base disabled:opacity-50"
            required
            disabled={loading}
          />
          {#if fieldErrors.email}
            <p class="mt-1 text-xs text-red-400">{fieldErrors.email}</p>
          {/if}
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
              class="w-full px-3 sm:px-4 py-2 bg-gray-700 border {fieldErrors.password
                ? 'border-red-500'
                : 'border-gray-600'} rounded focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm sm:text-base disabled:opacity-50"
              required
              disabled={loading}
              minlength="8"
            />
            <button
              type="button"
              class="absolute right-2 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-300"
              on:click={() => (showPassword = !showPassword)}
            >
              <Icon icon={showPassword ? "ph:eye-slash" : "ph:eye"} class="w-5 h-5" />
            </button>
          </div>
          {#if fieldErrors.password}
            <p class="mt-1 text-xs text-red-400">{fieldErrors.password}</p>
          {/if}
          <p class="mt-1 text-xs text-gray-400">
            Password must be at least 8 characters long and contain uppercase, lowercase,
            number, and special character (@$!%*?&)
          </p>
        </div>

        <div class="mb-6">
          <label class="block text-gray-300 mb-1 text-sm sm:text-base" for="confirmPassword"
            >Confirm Password</label
          >
          <div class="relative">
            <input
              id="confirmPassword"
              type={showConfirmPassword ? "text" : "password"}
              bind:value={confirmPassword}
              placeholder="Confirm your password"
              class="w-full px-3 sm:px-4 py-2 bg-gray-700 border border-gray-600 rounded focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm sm:text-base disabled:opacity-50"
              required
              disabled={loading}
            />
            <button
              type="button"
              class="absolute right-2 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-300"
              on:click={() => (showConfirmPassword = !showConfirmPassword)}
            >
              <Icon icon={showConfirmPassword ? "ph:eye-slash" : "ph:eye"} class="w-5 h-5" />
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
            Creating Account...
          {:else}
            Sign Up
          {/if}
        </button>
      </form>

      <p class="mt-4 text-center text-gray-300 text-sm sm:text-base">
        Already have an account?
        <a href="/login" class="text-blue-400 hover:underline">Login</a>
      </p>
    </div>
  </div>
</RedirectGuard>
