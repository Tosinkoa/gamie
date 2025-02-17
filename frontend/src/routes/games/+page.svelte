<script lang="ts">
  import Icon from "@iconify/svelte";
  import { goto } from "$app/navigation";
  import type { Game } from "$lib/types";

  const games: Game[] = [
    {
      id: "number-chaos",
      name: "Number Chaos",
      description: "Race against time to arrange numbers in sequence",
      icon: "ph:sort-ascending",
      players: "2-4 Players",
      difficulty: "Medium",
    },
  ];

  function handleGameClick(gameId: string) {
    goto(`/rooms/${gameId}`);
  }
</script>

<div class="min-h-screen bg-gradient-to-br from-gray-800 to-black px-4 py-8 sm:px-6 lg:px-8">
  <div class="max-w-7xl mx-auto">
    <h1 class="text-3xl sm:text-4xl font-bold text-white text-center mb-8">Available Games</h1>
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each games as game}
        <button
          type="button"
          class="bg-gray-900 rounded-lg p-6 shadow-xl transform transition-all duration-300 hover:scale-105 text-left w-full"
          on:click={() => handleGameClick(game.id)}
        >
          <div class="flex items-center justify-center mb-4">
            <div class="w-20 h-20 bg-blue-500/20 rounded-2xl flex items-center justify-center">
              <Icon icon={game.icon} class="w-12 h-12 text-blue-400" />
            </div>
          </div>
          <h2 class="text-xl font-semibold text-white mb-2 text-center">
            {game.name}
          </h2>
          <p class="text-gray-400 text-center mb-4">
            {game.description}
          </p>
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
        </button>
      {/each}
    </div>
  </div>
</div>
