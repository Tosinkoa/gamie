<script lang="ts">
  import Icon from "@iconify/svelte";
  import { page } from "$app/stores";
  import { onMount } from "svelte";

  const gameId = $page.params.game;
  const roomId = $page.params.roomId;

  // Mock game state - replace with actual game logic and state management
  let gameState = {
    status: "waiting",
    name: "Quick Sort Challenge",
    players: [
      {
        username: "Player1",
        avatar: "https://api.dicebear.com/7.x/avataaars/svg?seed=Player1",
        ready: true,
        isSpeaking: false,
        isMuted: false,
        soundMuted: false,
        country: "US",
        countryName: "United States",
      },
      {
        username: "Player2",
        avatar: "https://api.dicebear.com/7.x/avataaars/svg?seed=Player2",
        ready: false,
        isSpeaking: true,
        isMuted: false,
        soundMuted: true,
      },
    ],
    settings: {
      order: "asc",
      time: 60,
      range: "1-100",
    },
  };

  let showSidebar = false;
  let isMicEnabled = false;
  let isSpeakerEnabled = false;
  let mounted = false;

  function toggleMic() {
    isMicEnabled = !isMicEnabled;
    // TODO: Implement actual mic logic
  }

  function toggleSidebar() {
    showSidebar = !showSidebar;
  }

  function handleLeaveRoom() {
    // TODO: Implement leave room logic
  }

  function toggleSpeaker() {
    isSpeakerEnabled = !isSpeakerEnabled;
    // TODO: Implement actual speaker logic
  }

  onMount(() => {
    mounted = true;
    // TODO: Connect to game websocket
    // TODO: Initialize game state
    // TODO: Setup voice chat
  });
</script>

<div
  class="game-page h-[100dvh] bg-gradient-to-br from-gray-800 to-black p-2 sm:p-6 overflow-hidden"
>
  <div class="h-full max-w-7xl mx-auto flex flex-col">
    <!-- Game Card - Single card for mobile -->
    <div class="bg-gray-900 rounded-xl h-full flex flex-col overflow-hidden">
      <!-- Header Section -->
      <div class="p-3 sm:p-6 border-b border-gray-800 shrink-0">
        <div class="flex items-center justify-between gap-2">
          <div class="flex items-center gap-2">
            <a
              href="/rooms/{gameId}"
              class="text-blue-400 hover:underline text-sm flex items-center"
            >
              <Icon icon="ph:arrow-left" class="w-4 h-4 mr-1" />
              <span class="hidden sm:inline">Rooms</span>
            </a>
            <h1 class="text-base sm:text-xl font-bold text-white truncate">
              {gameState.name}
            </h1>
          </div>
          <div class="flex items-center gap-2">
            <!-- Timer -->
            <div class="flex text-yellow-500 items-center bg-gray-800/50 px-2 py-1 rounded-lg">
              <Icon icon="ph:clock" class="w-4 h-4 sm:w-5 sm:h-5" />
              <span class="text-sm sm:text-base font-bold ml-1"
                >{gameState.settings.time}s</span
              >
            </div>
            <!-- Leave button -->
            <button
              on:click={handleLeaveRoom}
              class="p-2 sm:px-4 sm:py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg transition-colors duration-300 flex items-center"
            >
              <Icon icon="ph:door-open" class="w-4 h-4 sm:w-5 sm:h-5" />
              <span class="hidden sm:inline ml-2">Leave</span>
            </button>
          </div>
        </div>

        <!-- Game Controls & Settings -->
        <div class="mt-3 flex flex-wrap items-center gap-2 text-sm">
          <div class="flex items-center gap-2">
            <!-- Voice Controls -->
            <button
              class="p-2 {isSpeakerEnabled
                ? 'bg-green-600 hover:bg-green-700'
                : 'bg-gray-800 hover:bg-gray-700'} text-white rounded-lg transition-colors duration-300"
              on:click={toggleSpeaker}
            >
              <Icon
                icon={isSpeakerEnabled ? "ph:speaker-high" : "ph:speaker-slash"}
                class="w-4 h-4 sm:w-5 sm:h-5"
              />
            </button>
            <button
              class="p-2 {isMicEnabled
                ? 'bg-green-600 hover:bg-green-700'
                : 'bg-gray-800 hover:bg-gray-700'} text-white rounded-lg transition-colors duration-300"
              on:click={toggleMic}
            >
              <Icon
                icon={isMicEnabled ? "ph:microphone" : "ph:microphone-slash"}
                class="w-4 h-4 sm:w-5 sm:h-5"
              />
            </button>
            <!-- Players Toggle -->
            <button
              class="lg:hidden p-2 bg-gray-800 hover:bg-gray-700 text-white rounded-lg"
              on:click={toggleSidebar}
            >
              <Icon icon="ph:users" class="w-4 h-4 sm:w-5 sm:h-5" />
            </button>
          </div>
          <!-- Game Settings -->
          <div class="flex flex-wrap gap-2 text-xs sm:text-sm text-gray-400 ml-auto">
            <div class="flex items-center bg-gray-800/50 px-2 py-1 rounded-lg">
              <Icon icon="ph:sort-ascending" class="w-3 h-3 sm:w-4 sm:h-4 mr-1" />
              {gameState.settings.order === "asc" ? "Ascending" : "Descending"}
            </div>
            <div class="flex items-center bg-gray-800/50 px-2 py-1 rounded-lg">
              <Icon icon="ph:hash" class="w-3 h-3 sm:w-4 sm:h-4 mr-1" />
              {gameState.settings.range}
            </div>
          </div>
        </div>
      </div>

      <!-- Main Content Area -->
      <div class="flex-1 flex overflow-hidden">
        <!-- Player List - Hidden on mobile unless toggled -->
        <div
          class="lg:w-72 {showSidebar
            ? 'fixed inset-0 z-50 lg:relative lg:inset-auto'
            : 'hidden lg:block'} border-r border-gray-800"
        >
          {#if showSidebar}
            <div
              class="fixed inset-0 bg-black bg-opacity-50 lg:hidden"
              on:click={toggleSidebar}
            ></div>
          {/if}
          <div
            class="bg-gray-900 h-full overflow-auto {showSidebar
              ? 'fixed right-0 top-0 bottom-0 w-72 lg:relative lg:w-auto'
              : ''}"
          >
            <div class="p-4">
              <div class="flex items-center justify-between mb-4">
                <h2 class="text-lg font-bold text-white">Players</h2>
                {#if showSidebar}
                  <button
                    class="lg:hidden text-gray-400 hover:text-white"
                    on:click={toggleSidebar}
                  >
                    <Icon icon="ph:x" class="w-5 h-5" />
                  </button>
                {/if}
              </div>
              <div class="space-y-2">
                {#each gameState.players as player}
                  <div class="flex items-center justify-between bg-gray-800/50 rounded-lg p-2">
                    <div class="flex items-center gap-2">
                      <div class="relative">
                        <img
                          src={player.avatar}
                          alt={player.username}
                          class="w-8 h-8 rounded-full"
                        />
                        {#if player.isSpeaking}
                          <div
                            class="absolute -bottom-1 -right-1 w-3 h-3 bg-green-500 rounded-full border-2 border-gray-900"
                          ></div>
                        {/if}
                      </div>
                      <div>
                        <div class="flex items-center gap-1">
                          <span class="text-sm text-white truncate max-w-[80px]"
                            >{player.username}</span
                          >
                        </div>
                      </div>
                    </div>
                    <div class="flex items-center gap-1">
                      <Icon
                        icon={player.ready ? "ph:check-circle" : "ph:clock"}
                        class="w-4 h-4 {player.ready ? 'text-green-500' : 'text-yellow-500'}"
                      />
                    </div>
                  </div>
                {/each}
              </div>
            </div>
          </div>
        </div>

        <!-- Game Content -->
        <div class="flex-1 p-4 flex items-center justify-center">
          <div class="text-center max-w-md mx-auto">
            <Icon
              icon="ph:game-controller"
              class="w-12 h-12 sm:w-16 sm:h-16 text-gray-600 mx-auto mb-4"
            />
            <h2 class="text-xl sm:text-2xl font-bold text-white mb-2">Game Starting Soon</h2>
            <p class="text-sm sm:text-base text-gray-400 mb-6">
              Waiting for all players to be ready...
            </p>
            <!-- Ready Button -->
            <button
              class="px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-xl transition-colors duration-300 flex items-center mx-auto"
            >
              <Icon icon="ph:check-circle" class="w-5 h-5 mr-2" />
              Ready to Play
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  :global(.game-page) {
    @apply overflow-hidden;
  }
</style>
