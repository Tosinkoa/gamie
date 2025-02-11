<script lang="ts">
  import Icon from "@iconify/svelte";
  import { goto } from "$app/navigation";
  import type { Game } from "$lib/types";

  const games: Game[] = [
    {
      id: "tictactoe",
      name: "Tic Tac Toe",
      description: "Classic game of X's and O's",
      icon: "game-icons:tic-tac-toe",
      players: "2 Players",
      difficulty: "Easy",
    },
    {
      id: "chess",
      name: "Chess",
      description: "Strategic board game of kings and queens",
      icon: "game-icons:chess-knight",
      players: "2 Players",
      difficulty: "Hard",
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

  function handleGameClick(gameId: string) {
    goto(`/rooms/${gameId}`);
  }
</script>

<div class="min-h-screen bg-gradient-to-br from-gray-800 to-black px-4 py-8 sm:px-6 lg:px-8">
  <div class="max-w-7xl mx-auto">
    <h1 class="text-3xl sm:text-4xl font-bold text-white mb-8 text-center">Available Games</h1>

    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each games as game}
        <button
          type="button"
          class="bg-gray-900 rounded-lg p-6 shadow-xl transform transition-all duration-300 hover:scale-105 text-left w-full"
          on:click={() => handleGameClick(game.id)}
        >
          <div class="flex items-center justify-center mb-4">
            <Icon icon={game.icon} class="w-16 h-16 text-blue-500" />
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
