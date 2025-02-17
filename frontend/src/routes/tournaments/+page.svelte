<script lang="ts">
  import Icon from "@iconify/svelte";
  import { onMount } from "svelte";

  let mounted = false;

  onMount(() => {
    mounted = true;
  });
</script>

<div
  class="min-h-screen bg-gradient-to-br from-gray-800 to-black px-4 py-8 sm:px-6 lg:px-8 flex items-center justify-center"
>
  <div class="text-center">
    <h1 class="text-5xl sm:text-6xl font-bold text-white mb-6">Tournaments</h1>
    <p class="text-xl text-gray-300 mb-8">
      Exciting tournaments are on the way! Stay tuned for epic battles and amazing prizes.
    </p>
    <div class="flex justify-center gap-4 mb-12">
      <div class="bg-gray-800 px-6 py-4 rounded-xl">
        <span class="block text-3xl font-bold text-blue-400 animate-pulse"> Coming Soon </span>
      </div>
    </div>
    <div class="flex justify-center">
      <a
        href="/"
        class="px-8 py-4 bg-blue-600 hover:bg-blue-700 text-white rounded-xl transition-all duration-300 transform hover:scale-105 flex items-center justify-center"
      >
        <Icon icon="ph:arrow-left" class="w-6 h-6 mr-2" />
        Back to Home
      </a>
    </div>
  </div>
</div>

<style>
  .animate-pulse {
    animation: pulse 2s infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
  }
</style>
