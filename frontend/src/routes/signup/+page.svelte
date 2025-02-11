<script lang="ts">
  import Icon from "@iconify/svelte";

  function handleSubmit(e: Event) {
    e.preventDefault();
    // TODO: implement signup action
    console.log("Signup submitted");
  }
</script>

<div
  class="min-h-screen flex items-center justify-center bg-gradient-to-br from-gray-800 to-black px-4 sm:px-6 lg:px-8"
>
  <div class="bg-gray-800 p-6 sm:p-8 rounded-lg shadow-xl w-full max-w-md relative">
    <a href="/" class="absolute top-4 left-4 text-sm text-blue-400 hover:underline"
      >&larr; Back to Home</a
    >
    <h2 class="text-2xl sm:text-3xl font-bold text-white text-center mt-8 mb-6">
      Create Your Account
    </h2>
    <button
      class="w-full py-2 mb-4 bg-gray-700 hover:bg-gray-600 text-white rounded-md transition flex items-center justify-center"
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
        <label class="block text-gray-300 mb-1 text-sm sm:text-base" for="name">Username</label
        >
        <input
          id="username"
          type="text"
          placeholder="Your username"
          class="w-full px-3 sm:px-4 py-2 bg-gray-700 border border-gray-600 rounded focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm sm:text-base"
          required
        />
      </div>
      <div class="mb-4">
        <label class="block text-gray-300 mb-1 text-sm sm:text-base" for="email">Email</label>
        <input
          id="email"
          type="email"
          placeholder="Your email"
          class="w-full px-3 sm:px-4 py-2 bg-gray-700 border border-gray-600 rounded focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm sm:text-base"
          required
        />
      </div>
      <div class="mb-6">
        <label class="block text-gray-300 mb-1 text-sm sm:text-base" for="password"
          >Password</label
        >
        <input
          id="password"
          type="password"
          placeholder="Your password"
          class="w-full px-3 sm:px-4 py-2 bg-gray-700 border border-gray-600 rounded focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm sm:text-base"
          required
        />
      </div>
      <div class="mb-6">
        <label class="block text-gray-300 mb-1 text-sm sm:text-base" for="confirmPassword"
          >Confirm Password</label
        >
        <input
          id="confirmPassword"
          type="password"
          placeholder="Confirm your password"
          class="w-full px-3 sm:px-4 py-2 bg-gray-700 border border-gray-600 rounded focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm sm:text-base"
          required
        />
      </div>
      <button
        type="submit"
        class="w-full py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition"
      >
        Sign Up
      </button>
    </form>
    <p class="mt-4 text-center text-gray-300 text-sm sm:text-base">
      Already have an account?
      <a href="/login" class="text-blue-400 hover:underline">Login</a>
    </p>
  </div>
</div>
