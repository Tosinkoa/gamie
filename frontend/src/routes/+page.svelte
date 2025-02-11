<script lang="ts">
  import Icon from "@iconify/svelte";
  import { onMount } from "svelte";
  import type { Game } from "$lib/types";

  //@Todo Add create room dialog

  const featuredGames: Game[] = [
    {
      id: "chess",
      name: "Chess",
      description: "Strategic board game of kings and queens",
      icon: "game-icons:chess-knight",
      players: "2 Players",
      difficulty: "Hard",
    },
    {
      id: "tictactoe",
      name: "Tic Tac Toe",
      description: "Classic game of X's and O's",
      icon: "game-icons:tic-tac-toe",
      players: "2 Players",
      difficulty: "Easy",
    },
    {
      id: "connect4",
      name: "Connect 4",
      description: "Connect four pieces in a row to win",
      icon: "game-icons:connect",
      players: "2 Players",
      difficulty: "Medium",
    },
  ];

  const stats = [
    { icon: "ph:users-three", label: "Active Players", value: "10,000+" },
    { icon: "ph:trophy", label: "Tournaments", value: "500+" },
    { icon: "ph:game-controller", label: "Games Played", value: "1M+" },
    { icon: "ph:crown-simple", label: "Champions", value: "1,000+" },
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
      <div class="absolute inset-0 bg-gradient-to-br from-blue-500/10 to-purple-500/10" />
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
            class="w-4 h-4 bg-blue-500/20 rounded-full backdrop-blur-sm"
            style="transform: scale({0.5 + Math.random()});"
          />
        </div>
      {/each}
    </div>

    <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pt-20 pb-16">
      <div class="text-center">
        <h1 class="text-5xl sm:text-6xl lg:text-7xl font-bold mb-8">
          <span
            class="text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-purple-500"
          >
            Game
          </span>
          <span class="text-white">Connect</span>
        </h1>
        <p class="text-xl sm:text-2xl text-gray-300 max-w-3xl mx-auto mb-12">
          Connect with players worldwide, compete in tournaments, and become a champion in your
          favorite games!
        </p>
        <div class="flex flex-col sm:flex-row justify-center gap-4 mb-16">
          <a
            href="/signup"
            class="px-8 py-4 bg-blue-600 hover:bg-blue-700 text-white rounded-xl transition-all duration-300 transform hover:scale-105 flex items-center justify-center group"
          >
            <Icon icon="ph:game-controller" class="w-6 h-6 mr-2 group-hover:animate-wiggle" />
            Start Playing Now
          </a>
          <a
            href="/games"
            class="px-8 py-4 bg-gray-700 hover:bg-gray-600 text-white rounded-xl transition-all duration-300 transform hover:scale-105 flex items-center justify-center"
          >
            <Icon icon="ph:play-circle" class="w-6 h-6 mr-2" />
            Explore Games
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
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
      {#each featuredGames as game, index}
        <a
          href="/rooms/{game.id}"
          class="group bg-gray-800/50 backdrop-blur-sm rounded-xl p-8 transform hover:scale-105 transition-all duration-300 border border-gray-700/50 {mounted
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

  <!-- Call to Action -->
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
    <div
      class="bg-gradient-to-r from-blue-600 to-purple-600 rounded-2xl p-8 sm:p-12 text-center relative overflow-hidden"
    >
      <div class="relative z-10">
        <h2 class="text-3xl sm:text-4xl font-bold text-white mb-4">Ready to Join the Fun?</h2>
        <p class="text-lg text-white/90 mb-8 max-w-2xl mx-auto">
          Create your account now and join thousands of players in epic gaming battles!
        </p>
        <div class="flex flex-col sm:flex-row justify-center gap-4">
          <a
            href="/signup"
            class="px-8 py-4 bg-white text-blue-600 rounded-xl font-semibold hover:bg-gray-100 transition-colors duration-300 flex items-center justify-center"
          >
            <Icon icon="ph:user-plus" class="w-6 h-6 mr-2" />
            Create Account
          </a>
          <a
            href="/premium"
            class="px-8 py-4 bg-gray-900/30 text-white rounded-xl font-semibold hover:bg-gray-900/40 transition-colors duration-300 flex items-center justify-center"
          >
            <Icon icon="ph:crown-simple" class="w-6 h-6 mr-2" />
            Go Premium
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
        />
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

  .group-hover\:animate-wiggle:hover {
    animation: wiggle 0.3s ease-in-out infinite;
  }
</style>
