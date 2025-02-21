<script lang="ts">
  import Icon from "@iconify/svelte";
  import RoomCard from "$lib/components/RoomCard.svelte";
  import { page } from "$app/stores";
  import type { Room } from "$lib/types";
  import { gameTitles } from "$lib/types";
  import { goto } from "$app/navigation";

  const gameId = $page.params.game;

  // Mock data - replace with actual API call
  const rooms: Room[] = [
    {
      id: "1",
      name: "Casual Game Room",
      game: gameId,
      players: { current: 1, max: 2 },
      status: "waiting",
      creator: "Player1",
      difficulty: "Easy",
      avatars: [
        "https://api.dicebear.com/7.x/avataaars/svg?seed=User1",
        "https://api.dicebear.com/7.x/avataaars/svg?seed=User2",
        "https://api.dicebear.com/7.x/avataaars/svg?seed=User3",
      ],
    },
    {
      id: "2",
      name: "Pro Players Only",
      game: gameId,
      players: { current: 2, max: 2 },
      status: "playing",
      creator: "Player2",
      difficulty: "Medium",
      avatars: [
        "https://api.dicebear.com/7.x/avataaars/svg?seed=User4",
        "https://api.dicebear.com/7.x/avataaars/svg?seed=User5",
      ],
    },
    {
      id: "3",
      name: "Beginners Welcome",
      game: gameId,
      players: { current: 0, max: 2 },
      status: "waiting",
      creator: "Player3",
      difficulty: "Easy",
      avatars: ["https://api.dicebear.com/7.x/avataaars/svg?seed=User6"],
    },
  ];

  let showCreateForm = false;

  // Game settings dummy data
  let order: string = "asc";
  let time: number = 60; // seconds
  let range: string = "1-100";
  let players: number = 2;

  // We prefill selectedGame with gameId from URL
  let selectedGame: string = gameId;

  function toggleCreateForm() {
    showCreateForm = !showCreateForm;
  }

  function handleCreateRoom() {
    // No need to check selectedGame because it's preset
    const settings = { game: selectedGame, order, time, range, players };
    console.log("Create Room with settings:", settings);
    // TODO: send settings to backend
    // After creation, redirect to the new room
    goto(`/game/${selectedGame}/room-id`); // room-id will be returned by backend
    showCreateForm = false;
  }
</script>

<div
  class="min-h-screen bg-gradient-to-br from-gray-800 to-black px-2 sm:px-4 lg:px-6 py-4 sm:py-6"
>
  <div class="max-w-7xl mx-auto">
    <div class="flex flex-col gap-2 sm:gap-4 mb-4 sm:mb-6">
      <a href="/games" class="text-blue-400 hover:underline text-sm"> ← Back to Games </a>
      <div class="flex flex-col sm:flex-row sm:justify-between sm:items-center gap-3">
        <h1 class="text-2xl sm:text-3xl font-bold text-white">
          {gameTitles[gameId] || gameId} Rooms
        </h1>
        <button
          on:click={toggleCreateForm}
          class="px-3 py-1.5 sm:px-4 sm:py-2 bg-green-600 hover:bg-green-700 text-white rounded-md transition-colors duration-300 flex items-center text-sm sm:text-base w-fit"
        >
          <Icon icon="ph:plus-circle" class="w-4 h-4 sm:w-5 sm:h-5 mr-1.5" />
          {showCreateForm ? "Cancel" : "Create Room"}
        </button>
      </div>
    </div>

    {#if showCreateForm}
      <div class="fixed inset-0 flex items-center justify-center z-50 px-4">
        <!-- Modal backdrop -->
        <div class="absolute inset-0 bg-black bg-opacity-50" on:click={toggleCreateForm}></div>
        <!-- Modal content -->
        <div class="bg-gray-900 rounded-xl p-4 sm:p-6 relative z-10 w-full max-w-xl mx-auto">
          <!-- Close button -->
          <button
            on:click={toggleCreateForm}
            class="absolute top-2 right-2 text-white text-2xl focus:outline-none"
            >&times;</button
          >
          <h2 class="text-lg sm:text-xl font-semibold text-white mb-4">Create a New Room</h2>
          <!-- Display settings fields directly -->
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-4">
            <div>
              <label class="block text-gray-300 text-sm mb-1">Order</label>
              <select
                bind:value={order}
                class="w-full p-2 text-sm bg-gray-800 border border-gray-700 rounded"
              >
                <option value="asc">Ascending</option>
                <option value="desc">Descending</option>
              </select>
            </div>
            <div>
              <label class="block text-gray-300 text-sm mb-1">Time (seconds)</label>
              <input
                type="number"
                bind:value={time}
                min="10"
                class="w-full p-2 text-sm bg-gray-800 border border-gray-700 rounded"
              />
            </div>
            <div>
              <label class="block text-gray-300 text-sm mb-1">Range</label>
              <select
                bind:value={range}
                class="w-full p-2 text-sm bg-gray-800 border border-gray-700 rounded"
              >
                <option value="1-100">1-100</option>
                <option value="101-200">101-200</option>
                <option value="201-300">201-300</option>
              </select>
            </div>
            <div>
              <label class="block text-gray-300 text-sm mb-1">Number of Players</label>
              <input
                type="number"
                bind:value={players}
                min="2"
                max="4"
                class="w-full p-2 text-sm bg-gray-800 border border-gray-700 rounded"
              />
            </div>
          </div>
          <button
            on:click={handleCreateRoom}
            class="w-full sm:w-auto px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition text-sm"
          >
            Create Room
          </button>
        </div>
      </div>
    {/if}

    <div class="grid grid-cols-2 lg:grid-cols-3 gap-2 sm:gap-4">
      {#each rooms as room}
        <RoomCard {room} />
      {/each}
    </div>

    {#if rooms.length === 0}
      <div class="text-center text-gray-400 py-8 sm:py-12">
        <Icon
          icon="ph:game-controller"
          class="w-12 h-12 sm:w-16 sm:h-16 mx-auto mb-3 sm:mb-4"
        />
        <p class="text-lg sm:text-xl">No rooms available for {gameTitles[gameId] || gameId}</p>
        <p class="mt-2 text-sm sm:text-base">Be the first to create a room!</p>
      </div>
    {/if}
  </div>
</div>
