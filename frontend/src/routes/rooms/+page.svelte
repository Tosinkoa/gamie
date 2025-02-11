<script lang="ts">
  import Icon from "@iconify/svelte";
  import RoomCard from "$lib/components/RoomCard.svelte";
  import type { Room } from "$lib/types";

  // Mock data - replace with actual API call
  const rooms: Room[] = [
    {
      id: "1",
      name: "Casual Game Room",
      game: "Tic Tac Toe",
      players: { current: 1, max: 2 },
      status: "waiting",
      creator: "Player1",
    },
    {
      id: "2",
      name: "Pro Players Only",
      game: "Chess",
      players: { current: 2, max: 2 },
      status: "playing",
      creator: "Player2",
    },
    {
      id: "3",
      name: "Beginners Welcome",
      game: "Connect 4",
      players: { current: 0, max: 2 },
      status: "waiting",
      creator: "Player3",
    },
  ];

  function createRoom() {
    // TODO: Implement room creation
    console.log("Create room clicked");
  }
</script>

<div class="min-h-screen bg-gradient-to-br from-gray-800 to-black px-4 py-8 sm:px-6 lg:px-8">
  <div class="max-w-7xl mx-auto">
    <div class="flex justify-between items-center mb-8">
      <h1 class="text-3xl sm:text-4xl font-bold text-white">Game Rooms</h1>
      <button
        on:click={createRoom}
        class="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-md transition-colors duration-300 flex items-center"
      >
        <Icon icon="ph:plus-circle" class="w-5 h-5 mr-2" />
        Create Room
      </button>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each rooms as room}
        <RoomCard {room} />
      {/each}
    </div>

    {#if rooms.length === 0}
      <div class="text-center text-gray-400 py-12">
        <Icon icon="ph:game-controller" class="w-16 h-16 mx-auto mb-4" />
        <p class="text-xl">No rooms available</p>
        <p class="mt-2">Be the first to create a room!</p>
      </div>
    {/if}
  </div>
</div>
