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
        country: "GB",
        countryName: "United Kingdom",
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
  class="game-page h-[calc(100dvh-4rem)] bg-gradient-to-br from-gray-800 to-black p-4 sm:p-6 overflow-hidden"
>
  <div class="h-full max-w-7xl mx-auto flex flex-col">
    <!-- Game Header -->
    <div class="bg-gray-900 rounded-xl p-4 sm:p-6 mb-4 sm:mb-6 shrink-0 ">
      <div
        class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-3 sm:gap-4"
      >
        <div class="w-full">
          <div class="flex items-center gap-2 mb-1 w-full">
            <a
              href="/rooms/{gameId}"
              class="text-blue-400 hover:underline text-sm flex items-center"
            >
              <Icon icon="ph:arrow-left" class="w-4 h-4 mr-1" />
              Rooms
            </a>
            <div class="flex items-center ml-auto gap-x-2">
              <!-- Mobile sidebar toggle -->
              <button
                class="lg:hidden px-3 py-2 bg-gray-800 text-white rounded-lg"
                on:click={toggleSidebar}
              >
                <Icon icon="ph:sidebar" class="w-5 h-5" />
              </button>
              <!-- Voice chat controls -->
              <button
                class="px-3 py-2 {isSpeakerEnabled
                  ? 'bg-green-600 hover:bg-green-700'
                  : 'bg-gray-800 hover:bg-gray-700'} text-white rounded-lg transition-colors duration-300"
                on:click={toggleSpeaker}
                title={isSpeakerEnabled ? "Turn off microphone" : "Turn on microphone"}
              >
                <Icon
                  icon={isSpeakerEnabled ? "ph:speaker-high" : "ph:speaker-slash"}
                  class="size-6"
                />
              </button>
              <button
                class="px-3 py-2 {isMicEnabled
                  ? 'bg-green-600 hover:bg-green-700'
                  : 'bg-gray-800 hover:bg-gray-700'} text-white rounded-lg transition-colors duration-300"
                on:click={toggleMic}
                title={isMicEnabled ? "Turn off microphone" : "Turn on microphone"}
              >
                <Icon
                  icon={isMicEnabled ? "ph:microphone" : "ph:microphone-slash"}
                  class="size-6"
                />
              </button>
              <!-- Timer -->
              <div class="hidden sm:flex text-yellow-500 items-center">
                <Icon icon="ph:clock" class="w-6 h-6 mr-2" />
                <span class="text-2xl font-bold">{gameState.settings.time}s</span>
              </div>
              <!-- Leave button -->
              <button
                on:click={handleLeaveRoom}
                class="px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg transition-colors duration-300 flex items-center"
              >
                <Icon icon="ph:door-open" class="w-5 h-5 sm:mr-2" />
                <span class="hidden sm:inline">Leave Room</span>
              </button>
            </div>
          </div>
          <h1 class="text-lg sm:text-2xl font-bold text-white">{gameState.name}</h1>
        </div>
      </div>
    </div>

    <!-- Game Content -->
    <div class="flex-1 grid grid-cols-1 lg:grid-cols-4 gap-4 sm:gap-6 min-h-0">
      <!-- Player List - Hidden on mobile unless toggled -->
      <div
        class="lg:col-span-1 {showSidebar
          ? 'fixed inset-0 z-50 lg:relative lg:inset-auto'
          : 'hidden lg:block'}"
      >
        {#if showSidebar}
          <!-- Mobile overlay -->
          <div
            class="fixed inset-0 bg-black bg-opacity-50 lg:hidden"
            on:click={toggleSidebar}
          ></div>
        {/if}
        <div
          class="bg-gray-900 rounded-xl p-4 sm:p-6 h-full overflow-auto {showSidebar
            ? 'fixed right-0 top-0 bottom-0 w-80 lg:relative lg:w-auto'
            : ''}"
        >
          <!-- Close button for mobile -->
          {#if showSidebar}
            <button
              class="absolute top-4 right-4 text-gray-400 lg:hidden"
              on:click={toggleSidebar}
            >
              <Icon icon="ph:x" class="w-6 h-6" />
            </button>
          {/if}
          <h2 class="text-xl font-bold text-white mb-4">Players</h2>
          <div class="space-y-4">
            {#each gameState.players as player}
              <div class="flex items-center justify-between bg-gray-800 rounded-lg p-3">
                <div class="flex items-center gap-3">
                  <div class="relative">
                    <img
                      src={player.avatar}
                      alt={player.username}
                      class="w-10 h-10 rounded-full"
                    />
                    {#if player.isSpeaking}
                      <div
                        class="absolute -bottom-1 -right-1 w-4 h-4 bg-green-500 rounded-full border-2 border-gray-800"
                      ></div>
                    {:else if player.isMuted}
                      <div
                        class="absolute -bottom-1 -right-1 w-4 h-4 bg-red-500 rounded-full border-2 border-gray-800"
                      >
                        <Icon icon="ph:microphone-slash" class="w-3 h-3 text-white" />
                      </div>
                    {/if}
                  </div>
                  <div>
                    <div class="flex items-center gap-2">
                      <span class="text-white truncate max-w-[120px]" title={player.username}
                        >{player.username}</span
                      >
                      <img
                        src={`https://flagcdn.com/w20/${player.country.toLowerCase()}.png`}
                        alt={player.countryName}
                        class="w-4 h-3"
                        title={player.countryName}
                      />
                    </div>
                    <span class="text-sm text-gray-400 truncate max-w-[120px]"
                      >{player.countryName}</span
                    >
                  </div>
                </div>
                <div class="flex items-center gap-2">
                  <button
                    class="p-2 rounded-lg hover:bg-gray-700 transition-colors duration-200"
                    title={player.isMuted ? "Microphone muted" : "Microphone on"}
                  >
                    <Icon
                      icon={player.isMuted ? "ph:microphone-slash" : "ph:microphone"}
                      class="w-4 h-4 {player.isMuted ? 'text-red-400' : 'text-gray-400'}"
                    />
                  </button>
                  <button
                    class="p-2 rounded-lg hover:bg-gray-700 transition-colors duration-200"
                    title={player.soundMuted ? "Unmute sound" : "Mute sound"}
                  >
                    <Icon
                      icon={player.soundMuted ? "ph:speaker-slash" : "ph:speaker-high"}
                      class="w-4 h-4 {player.soundMuted ? 'text-red-400' : 'text-gray-400'}"
                    />
                  </button>
                  <Icon
                    icon={player.ready ? "ph:check-circle" : "ph:clock"}
                    class="w-5 h-5 {player.ready ? 'text-green-500' : 'text-yellow-500'}"
                  />
                </div>
              </div>
            {/each}
          </div>
        </div>
      </div>

      <!-- Game Area -->
      <div class="lg:col-span-3 h-full">
        <div class="bg-gray-900 rounded-xl p-4 sm:p-6 h-full flex flex-col">
          <!-- Mobile Player Info -->
          <div class="lg:hidden mb-4 sm:mb-6">
            <div class="flex items-center justify-between mb-4">
              <h3 class="text-lg font-semibold text-white">
                Players ({gameState.players.length}/4)
              </h3>
              <button
                class="px-3 py-1 text-sm bg-gray-800 text-gray-300 rounded-lg hover:bg-gray-700"
                on:click={toggleSidebar}
              >
                View Details
              </button>
            </div>
            <div class="flex flex-wrap gap-2">
              {#each gameState.players as player}
                <div class="flex items-center bg-gray-800/50 rounded-full pl-1 pr-3 py-1">
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
                  <div class="ml-2">
                    <div class="flex items-center gap-1">
                      <span
                        class="text-sm text-white truncate max-w-[80px]"
                        title={player.username}>{player.username}</span
                      >
                      <img
                        src={`https://flagcdn.com/w20/${player.country.toLowerCase()}.png`}
                        alt={player.countryName}
                        class="w-3 h-2"
                        title={player.countryName}
                      />
                    </div>
                  </div>
                  <div class="flex items-center gap-1 ml-2">
                    <button
                      class="p-1 rounded-lg hover:bg-gray-700/50 transition-colors duration-200"
                      title={player.isMuted ? "Microphone muted" : "Microphone on"}
                    >
                      <Icon
                        icon={player.isMuted ? "ph:microphone-slash" : "ph:microphone"}
                        class="w-3 h-3 {player.isMuted ? 'text-red-400' : 'text-gray-400'}"
                      />
                    </button>
                    <button
                      class="p-1 rounded-lg hover:bg-gray-700/50 transition-colors duration-200"
                      title={player.soundMuted ? "Unmute sound" : "Mute sound"}
                    >
                      <Icon
                        icon={player.soundMuted ? "ph:speaker-slash" : "ph:speaker-high"}
                        class="w-3 h-3 {player.soundMuted ? 'text-red-400' : 'text-gray-400'}"
                      />
                    </button>
                    <Icon
                      icon={player.ready ? "ph:check-circle" : "ph:clock"}
                      class="w-4 h-4 {player.ready ? 'text-green-500' : 'text-yellow-500'}"
                    />
                  </div>
                </div>
              {/each}
            </div>
            <!-- Game Settings Preview -->
            <div class="mt-4 flex flex-wrap gap-3 text-sm text-gray-400">
              <div class="flex items-center">
                <Icon icon="ph:sort-ascending" class="w-4 h-4 mr-1" />
                {gameState.settings.order === "asc" ? "Ascending" : "Descending"}
              </div>
              <div class="flex items-center">
                <Icon icon="ph:clock" class="w-4 h-4 mr-1" />
                {gameState.settings.time}s
              </div>
              <div class="flex items-center">
                <Icon icon="ph:hash" class="w-4 h-4 mr-1" />
                Range: {gameState.settings.range}
              </div>
            </div>
          </div>

          <!-- Game Content -->
          <div class="flex-1 flex items-center justify-center min-h-0">
            <div class="text-center">
              <Icon icon="ph:game-controller" class="w-16 h-16 text-gray-600 mx-auto mb-4" />
              <h2 class="text-2xl font-bold text-white mb-2">Game Starting Soon</h2>
              <p class="text-gray-400">Waiting for all players to be ready...</p>
              <!-- Ready Button -->
              <button
                class="mt-6 px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-xl transition-colors duration-300 flex items-center mx-auto"
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
</div>

<style>
  :global(.game-page) {
    @apply overflow-hidden;
  }
</style>
