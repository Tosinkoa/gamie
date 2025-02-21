<script lang="ts">
  import { goto } from "$app/navigation";
  import type { Game } from "$lib/types";

  const games: Game[] = [
    {
      id: "number-chaos",
      name: "Number Chaos",
      description: "Race against time to arrange numbers in sequence",
      image: "/number-chaos.webp",
      players: "2-4 Players",
    },
  ];

  function handleGameClick(gameId: string) {
    goto(`/rooms/${gameId}`);
  }
</script>

<div
  class="min-h-screen bg-gradient-to-br from-gray-800 to-black px-2 sm:px-4 lg:px-6 py-4 sm:py-6"
>
  <div class="max-w-7xl mx-auto">
    <h1 class="text-2xl sm:text-3xl font-bold text-white text-center mb-4 sm:mb-6">
      Available Games
    </h1>
    <div class="grid grid-cols-2 lg:grid-cols-3 gap-2 sm:gap-4">
      {#each games as game}
        <button
          type="button"
          class="bg-gray-900 rounded-lg p-3 sm:p-4 shadow-xl transform transition-all duration-300 hover:scale-105 text-left w-full"
          on:click={() => handleGameClick(game.id)}
        >
          <div class="flex items-center justify-center mb-2 sm:mb-3">
            <div class="w-full aspect-video rounded-lg overflow-hidden">
              <img src={game.image} alt={game.name} class="w-full h-full object-cover" />
            </div>
          </div>
          <h2 class="text-base sm:text-lg font-semibold text-white mb-1 sm:mb-2 text-center">
            {game.name}
          </h2>
          <p class="text-xs sm:text-sm text-gray-400 text-center mb-2 sm:mb-3 line-clamp-2">
            {game.description}
          </p>
          <div class="flex justify-center text-xs sm:text-sm text-gray-500">
            <span class="flex items-center gap-1">
              👥 {game.players}
            </span>
          </div>
        </button>
      {/each}
    </div>
  </div>
</div>
