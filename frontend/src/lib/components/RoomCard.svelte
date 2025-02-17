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
    difficulty: string;
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

<div
  class="bg-gray-900 rounded-lg p-3 sm:p-4 shadow-xl hover:shadow-2xl transition-all duration-300"
>
  <div class="flex justify-between items-start mb-2 sm:mb-3">
    <h3 class="text-base sm:text-lg font-semibold text-white truncate pr-2">{room.name}</h3>
    <span class={`flex items-center text-xs sm:text-sm ${getStatusColor(room.status)}`}>
      <Icon
        icon={room.status === "waiting"
          ? "ph:clock"
          : room.status === "playing"
            ? "ph:play-circle"
            : "ph:flag"}
        class="w-4 h-4 mr-1"
      />
      {room.status}
    </span>
  </div>

  <div class="space-y-1.5 sm:space-y-2 mb-3">
    <div class="flex items-center text-gray-400 text-xs sm:text-sm">
      <Icon icon="ph:game-controller" class="w-3.5 h-3.5 sm:w-4 sm:h-4 mr-1.5" />
      <span class="truncate">{room.game}</span>
    </div>
    <div class="flex items-center text-gray-400 text-xs sm:text-sm">
      <Icon icon="ph:target" class="w-3.5 h-3.5 sm:w-4 sm:h-4 mr-1.5" />
      <span class="truncate">{room.difficulty}</span>
    </div>
    <div class="flex items-center justify-between">
      <div class="flex items-center text-gray-400 text-xs sm:text-sm">
        <Icon icon="ph:users" class="w-3.5 h-3.5 sm:w-4 sm:h-4 mr-1.5" />
        <span>{room.players.current}/{room.players.max}</span>
      </div>
      <!-- User Avatars -->
      <div class="flex -space-x-1.5 sm:-space-x-2">
        {#each room.avatars.slice(0, 3) as avatar}
          <img
            src={avatar}
            alt="User Avatar"
            class="w-6 h-6 sm:w-7 sm:h-7 rounded-full border-2 border-gray-800 bg-gray-700 object-cover"
          />
        {/each}
        {#if room.avatars.length > 3}
          <div
            class="w-6 h-6 sm:w-7 sm:h-7 flex items-center justify-center bg-gray-700 text-white text-[10px] sm:text-xs rounded-full border-2 border-gray-800"
          >
            +{room.avatars.length - 3}
          </div>
        {/if}
      </div>
    </div>
  </div>

  <button
    on:click={handleJoinRoom}
    class="w-full py-1.5 sm:py-2 text-sm {room.status === 'waiting'
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
      class="w-4 h-4 sm:w-5 sm:h-5 mr-1.5"
    />
    {room.status === "waiting"
      ? "Join Room"
      : room.status === "playing"
        ? "Game in Progress"
        : "Game Ended"}
  </button>
</div>
