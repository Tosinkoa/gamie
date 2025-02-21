<script lang="ts">
  import Icon from "@iconify/svelte";
  import { onMount } from "svelte";
  import type { Game } from "$lib/types";

  //@Todo Add create room dialog

  const featuredGames: Game[] = [
    {
      id: "number-chaos",
      name: "Number Chaos",
      description: "Race against time to arrange numbers in sequence",
      icon: "ph:sort-ascending",
      players: "2-4 Players",
      difficulty: "Medium",
    },
  ];

  const stats = [
    { icon: "ph:users-three", label: "Active Players", value: "1,000+" },
    { icon: "ph:trophy", label: "Daily Tournaments", value: "10+" },
    { icon: "ph:sort-ascending", label: "Games Played", value: "50K+" },
    { icon: "ph:crown-simple", label: "Premium Players", value: "500+" },
  ];

  let mounted = false;
  onMount(() => {
    mounted = true;
  });
</script>

<div class="min-h-screen bg-gradient-to-br from-gray-800 to-black">
  <!-- Hero Section -->
  <div class="relative overflow-hidden">
    <!-- Animated Background -->
    <div class="absolute inset-0 overflow-hidden">
      <div class="absolute inset-0 bg-gradient-to-br from-blue-500/10 to-purple-500/10"></div>
      {#each Array(20) as _, i}
        <div
          class="absolute animate-float"
          style="
            left: {Math.random() * 100}%;
            top: {Math.random() * 100}%;
            animation-delay: {i * 0.5}s;
            animation-duration: {5 + Math.random() * 5}s;
          "
        >
          <div
            class="w-4 h-4 bg-gradient-to-br from-blue-400 to-purple-400 rounded-full shadow-lg"
            style="transform: scale({0.5 + Math.random()}); backdrop-filter: blur(8px);"
          ></div>
        </div>
      {/each}
    </div>

    <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pt-20 pb-16">
      <div class="text-center">
        <h1 class="text-5xl sm:text-6xl lg:text-7xl font-bold mb-8">
          <span
            class="text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-purple-500"
          >
            Number
          </span>
          <span class="text-white">Chaos</span>
        </h1>
        <p class="text-xl sm:text-2xl text-gray-300 max-w-3xl mx-auto mb-12">
          Challenge your mind with our fast-paced number sorting game. Compete with players
          worldwide and climb the rankings!
        </p>
        <div class="flex flex-col sm:flex-row justify-center gap-4 mb-16">
          <a
            href="/signup"
            class="px-8 py-4 bg-blue-600 hover:bg-blue-700 text-white rounded-xl transition-all duration-300 transform hover:scale-105 flex items-center justify-center group"
          >
            <Icon icon="ph:play-circle" class="w-6 h-6 mr-2" />
            Play Now - It's Free!
          </a>
          <a
            href="/premium"
            class="px-8 py-4 bg-gradient-to-r from-purple-600 to-blue-600 text-white rounded-xl transition-all duration-300 transform hover:scale-105 flex items-center justify-center relative overflow-hidden group"
          >
            <div
              class="absolute inset-0 bg-white/10 group-hover:bg-white/20 transition-colors duration-300"
            ></div>
            <Icon icon="ph:crown-simple" class="w-6 h-6 mr-2" />
            Go Premium
            <span class="ml-2 text-sm bg-white/20 px-2 py-0.5 rounded">50% OFF!</span>
          </a>
        </div>
      </div>

      <!-- Stats Section -->
      <div class="grid grid-cols-2 md:grid-cols-4 gap-6 max-w-4xl mx-auto">
        {#each stats as stat, index}
          <div
            class="bg-gray-800/50 backdrop-blur-sm rounded-xl p-6 text-center transform hover:scale-105 transition-all duration-300 {mounted
              ? 'animate-fade-in'
              : ''}"
            style="animation-delay: {index * 100}ms"
          >
            <Icon icon={stat.icon} class="w-8 h-8 text-blue-400 mx-auto mb-3" />
            <div class="text-2xl font-bold text-white mb-1">{stat.value}</div>
            <div class="text-sm text-gray-400">{stat.label}</div>
          </div>
        {/each}
      </div>
    </div>
  </div>

  <!-- Featured Games -->
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
    <h2 class="text-3xl font-bold text-white text-center mb-12">Featured Games</h2>
    <div class="flex justify-center">
      <div class="w-full max-w-md">
        {#each featuredGames as game, index}
          <a
            href="/rooms/{game.id}"
            class="block w-full bg-gray-800/50 backdrop-blur-sm rounded-xl p-8 transform hover:scale-105 transition-all duration-300 border border-gray-700/50 group {mounted
              ? 'animate-fade-in'
              : ''}"
            style="animation-delay: {(index + stats.length) * 100}ms"
          >
            <div class="flex items-center justify-center mb-6">
              <div
                class="w-20 h-20 bg-blue-500/20 rounded-2xl flex items-center justify-center group-hover:scale-110 transition-transform duration-300"
              >
                <Icon icon={game.icon} class="w-12 h-12 text-blue-400" />
              </div>
            </div>
            <h3 class="text-xl font-semibold text-white text-center mb-2">{game.name}</h3>
            <p class="text-gray-400 text-center mb-4">{game.description}</p>
            <div class="flex justify-between text-sm text-gray-500">
              <span class="flex items-center">
                <Icon icon="ph:users" class="w-4 h-4 mr-1" />
                {game.players}
              </span>
              <span class="flex items-center">
                <Icon icon="ph:gauge" class="w-4 h-4 mr-1" />
                {game.difficulty}
              </span>
            </div>
          </a>
        {/each}
      </div>
    </div>
  </div>

  <!-- Premium Features Highlight -->
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
    <h2 class="text-3xl font-bold text-white text-center mb-12">Why Go Premium?</h2>
    <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
      <div
        class="bg-gray-800/50 backdrop-blur-sm rounded-xl p-6 transform hover:scale-105 transition-all duration-300"
      >
        <div class="flex items-center justify-center mb-4">
          <div class="w-16 h-16 bg-blue-500/20 rounded-xl flex items-center justify-center">
            <Icon icon="ph:trophy" class="w-8 h-8 text-blue-400" />
          </div>
        </div>
        <h3 class="text-xl font-semibold text-white text-center mb-2">Daily Tournaments</h3>
        <p class="text-gray-400 text-center">
          Compete in exclusive tournaments with premium prizes and recognition.
        </p>
      </div>
      <div
        class="bg-gray-800/50 backdrop-blur-sm rounded-xl p-6 transform hover:scale-105 transition-all duration-300"
      >
        <div class="flex items-center justify-center mb-4">
          <div class="w-16 h-16 bg-purple-500/20 rounded-xl flex items-center justify-center">
            <Icon icon="ph:chart-line-up" class="w-8 h-8 text-purple-400" />
          </div>
        </div>
        <h3 class="text-xl font-semibold text-white text-center mb-2">Advanced Stats</h3>
        <p class="text-gray-400 text-center">
          Track your progress with detailed performance analytics and insights.
        </p>
      </div>
      <div
        class="bg-gray-800/50 backdrop-blur-sm rounded-xl p-6 transform hover:scale-105 transition-all duration-300"
      >
        <div class="flex items-center justify-center mb-4">
          <div class="w-16 h-16 bg-green-500/20 rounded-xl flex items-center justify-center">
            <Icon icon="ph:users-three" class="w-8 h-8 text-green-400" />
          </div>
        </div>
        <h3 class="text-xl font-semibold text-white text-center mb-2">Private Rooms</h3>
        <p class="text-gray-400 text-center">
          Create private rooms to challenge friends and organize your own tournaments.
        </p>
      </div>
    </div>
  </div>

  <!-- Call to Action -->
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
    <div
      class="bg-gradient-to-r from-blue-600 to-purple-600 rounded-2xl p-8 sm:p-12 text-center relative overflow-hidden"
    >
      <div class="relative z-10">
        <h2 class="text-3xl sm:text-4xl font-bold text-white mb-4">
          Ready to Challenge Yourself?
        </h2>
        <p class="text-lg text-white/90 mb-8 max-w-2xl mx-auto">
          Join thousands of players and start your journey to become a Number Chaos champion!
        </p>
        <div class="flex flex-col sm:flex-row justify-center gap-4">
          <a
            href="/signup"
            class="px-8 py-4 bg-white text-blue-600 rounded-xl font-semibold hover:bg-gray-100 transition-colors duration-300 flex items-center justify-center"
          >
            <Icon icon="ph:play-circle" class="w-6 h-6 mr-2" />
            Start Playing Free
          </a>
          <a
            href="/premium"
            class="px-8 py-4 bg-gray-900/30 text-white rounded-xl font-semibold hover:bg-gray-900/40 transition-colors duration-300 flex items-center justify-center group"
          >
            <Icon icon="ph:crown-simple" class="w-6 h-6 mr-2" />
            Upgrade to Premium
            <span class="ml-2 text-sm bg-white/20 px-2 py-0.5 rounded group-hover:bg-white/30"
              >50% OFF!</span
            >
          </a>
        </div>
      </div>
      <!-- Decorative Elements -->
      {#each Array(3) as _, i}
        <div
          class="absolute w-64 h-64 bg-white/10 rounded-full"
          style="
            left: {Math.random() * 100}%;
            top: {Math.random() * 100}%;
            transform: translate(-50%, -50%);
            filter: blur(40px);
          "
        ></div>
      {/each}
    </div>
  </div>
</div>

<style>
  .animate-float {
    animation: float 5s ease-in-out infinite;
  }

  @keyframes float {
    0%,
    100% {
      transform: translateY(0);
    }
    50% {
      transform: translateY(-20px);
    }
  }

  .animate-fade-in {
    animation: fadeIn 0.5s ease-out forwards;
    opacity: 0;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @keyframes wiggle {
    0%,
    100% {
      transform: rotate(0deg);
    }
    25% {
      transform: rotate(-10deg);
    }
    75% {
      transform: rotate(10deg);
    }
  }
</style>
