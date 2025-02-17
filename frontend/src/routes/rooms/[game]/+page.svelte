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

<div class="min-h-screen bg-gradient-to-br from-gray-800 to-black px-4 py-8 sm:px-6 lg:px-8">
  <div class="max-w-7xl mx-auto">
    <div class="flex justify-between items-center mb-8">
      <div>
        <a href="/games" class="text-blue-400 hover:underline mb-2 inline-block">
          ← Back to Games
        </a>
        <h1 class="text-3xl sm:text-4xl font-bold text-white">
          {gameTitles[gameId] || gameId} Rooms
        </h1>
      </div>
      <button
        on:click={toggleCreateForm}
        class="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-md transition-colors duration-300 flex items-center"
      >
        <Icon icon="ph:plus-circle" class="w-5 h-5 mr-2" />
        {showCreateForm ? "Cancel" : "Create Room"}
      </button>
    </div>

    {#if showCreateForm}
      <div class="fixed inset-0 flex items-center justify-center z-50">
        <!-- Modal backdrop -->
        <div class="absolute inset-0 bg-black bg-opacity-50" on:click={toggleCreateForm}></div>
        <!-- Modal content -->
        <div class="bg-gray-900 rounded-xl p-6 relative z-10 max-w-xl min-w-[500px] mx-auto">
          <!-- Close button -->
          <button
            on:click={toggleCreateForm}
            class="absolute top-2 right-2 text-white text-2xl focus:outline-none"
            >&times;</button
          >
          <h2 class="text-xl font-semibold text-white mb-4">Create a New Room</h2>
          <!-- Display settings fields directly -->
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-4">
            <div>
              <label class="block text-gray-300 mb-1">Order</label>
              <select
                bind:value={order}
                class="w-full p-2 bg-gray-800 border border-gray-700 rounded"
              >
                <option value="asc">Ascending</option>
                <option value="desc">Descending</option>
              </select>
            </div>
            <div>
              <label class="block text-gray-300 mb-1">Time (seconds)</label>
              <input
                type="number"
                bind:value={time}
                min="10"
                class="w-full p-2 bg-gray-800 border border-gray-700 rounded"
              />
            </div>
            <div>
              <label class="block text-gray-300 mb-1">Range</label>
              <select
                bind:value={range}
                class="w-full p-2 bg-gray-800 border border-gray-700 rounded"
              >
                <option value="1-100">1-100</option>
                <option value="101-200">101-200</option>
                <option value="201-300">201-300</option>
              </select>
            </div>
            <div>
              <label class="block text-gray-300 mb-1">Number of Players</label>
              <input
                type="number"
                bind:value={players}
                min="2"
                max="4"
                class="w-full p-2 bg-gray-800 border border-gray-700 rounded"
              />
            </div>
          </div>
          <button
            on:click={handleCreateRoom}
            class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition"
          >
            Create Room
          </button>
        </div>
      </div>
    {/if}

    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each rooms as room}
        <RoomCard {room} />
      {/each}
    </div>

    {#if rooms.length === 0}
      <div class="text-center text-gray-400 py-12">
        <Icon icon="ph:game-controller" class="w-16 h-16 mx-auto mb-4" />
        <p class="text-xl">No rooms available for {gameTitles[gameId] || gameId}</p>
        <p class="mt-2">Be the first to create a room!</p>
      </div>
    {/if}
  </div>
</div>
