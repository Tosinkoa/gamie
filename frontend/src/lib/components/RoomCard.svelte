<script lang="ts">
  import Icon from "@iconify/svelte";
  import { goto } from "$app/navigation";

  export let room: {
    id: string;
    name: string;
    game: string;
    players: {
      current: number;
      max: number;
    };
    status: "waiting" | "playing" | "finished";
    creator: string;
    avatars: string[]; // Array of user avatar URLs
  };

  function getStatusColor(status: string): string {
    switch (status) {
      case "waiting":
        return "text-yellow-500";
      case "playing":
        return "text-green-500";
      case "finished":
        return "text-red-500";
      default:
        return "text-gray-500";
    }
  }

  function handleJoinRoom() {
    goto(`/game/${room.game}/${room.id}`);
  }
</script>

<div class="bg-gray-900 rounded-lg p-6 shadow-xl hover:shadow-2xl transition-all duration-300">
  <div class="flex justify-between items-start mb-4">
    <h3 class="text-xl font-semibold text-white">{room.name}</h3>
    <span class={`flex items-center ${getStatusColor(room.status)}`}>
      <Icon
        icon={room.status === "waiting"
          ? "ph:clock"
          : room.status === "playing"
            ? "ph:play-circle"
            : "ph:flag"}
        class="w-5 h-5 mr-1"
      />
      {room.status}
    </span>
  </div>

  <div class="space-y-2 mb-4">
    <div class="flex items-center text-gray-400">
      <Icon icon="ph:game-controller" class="w-4 h-4 mr-2" />
      <span>{room.game}</span>
    </div>
    <div class="flex items-center text-gray-400">
      <Icon icon="ph:user" class="w-4 h-4 mr-2" />
      <span>{room.creator}</span>
    </div>
    <div class="flex items-center text-gray-400">
      <Icon icon="ph:users" class="w-4 h-4 mr-2" />
      <span>{room.players.current}/{room.players.max} players</span>
    </div>
    <!-- User Avatars -->
    <div class="flex -space-x-2 mt-2">
      {#each room.avatars.slice(0, 3) as avatar}
        <img
          src={avatar}
          alt="User Avatar"
          class="w-8 h-8 rounded-full border-2 border-gray-800 bg-gray-700 object-cover ring-2 ring-gray-900"
        />
      {/each}
      {#if room.avatars.length > 3}
        <div
          class="w-8 h-8 flex items-center justify-center bg-gray-700 text-white text-xs rounded-full border-2 border-gray-800 ring-2 ring-gray-900"
        >
          +{room.avatars.length - 3}
        </div>
      {/if}
    </div>
  </div>

  <button
    on:click={handleJoinRoom}
    class="w-full py-2 {room.status === 'waiting'
      ? 'bg-blue-600 hover:bg-blue-700'
      : room.status === 'playing'
        ? 'bg-gray-600 hover:bg-gray-700'
        : 'bg-red-600 hover:bg-red-700'} text-white rounded-md transition-colors duration-300 flex items-center justify-center"
    disabled={room.status !== "waiting" || room.players.current >= room.players.max}
  >
    <Icon
      icon={room.status === "waiting"
        ? "ph:sign-in"
        : room.status === "playing"
          ? "ph:hourglass"
          : "ph:flag"}
      class="w-5 h-5 mr-2"
    />
    {room.status === "waiting"
      ? "Join Room"
      : room.status === "playing"
        ? "Game in Progress"
        : "Game Ended"}
  </button>
</div>
