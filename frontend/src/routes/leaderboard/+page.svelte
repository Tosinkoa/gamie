<script lang="ts">
  import Icon from "@iconify/svelte";
  import { onMount } from "svelte";

  // Mock data - replace with actual API data
  const leaderboardData = [
    {
      rank: 1,
      username: "ProGamer123",
      avatar: "https://api.dicebear.com/7.x/avataaars/svg?seed=Felix",
      rating: 2850,
      wins: 158,
      losses: 22,
      winRate: 87.8,
      streak: 12,
      badges: ["ph:crown-simple", "ph:trophy", "ph:lightning"],
      games: ["Chess", "Tic Tac Toe", "Connect 4"],
    },
    {
      rank: 2,
      username: "ChessQueen",
      avatar: "https://api.dicebear.com/7.x/avataaars/svg?seed=Luna",
      rating: 2720,
      wins: 142,
      losses: 28,
      winRate: 83.5,
      streak: 8,
      badges: ["ph:trophy", "ph:lightning"],
      games: ["Chess", "Connect 4"],
    },
    {
      rank: 3,
      username: "TacticalMaster",
      avatar: "https://api.dicebear.com/7.x/avataaars/svg?seed=Max",
      rating: 2695,
      wins: 136,
      losses: 31,
      winRate: 81.4,
      streak: 5,
      badges: ["ph:trophy"],
      games: ["Chess", "Tic Tac Toe"],
    },
    // Add more players...
  ];

  const timeFrames = ["All Time", "This Month", "This Week", "Today"];
  const gameFilters = ["All Games", "Chess", "Tic Tac Toe", "Connect 4"];

  let selectedTimeFrame = "All Time";
  let selectedGame = "All Games";
  let showStats = false;

  function getBadgeColor(badge: string): string {
    switch (badge) {
      case "ph:crown-simple":
        return "text-yellow-400";
      case "ph:trophy":
        return "text-blue-400";
      case "ph:lightning":
        return "text-purple-400";
      default:
        return "text-gray-400";
    }
  }

  function getRankStyle(rank: number): string {
    switch (rank) {
      case 1:
        return "bg-gradient-to-r from-yellow-500 to-yellow-600 text-white";
      case 2:
        return "bg-gradient-to-r from-gray-400 to-gray-500 text-white";
      case 3:
        return "bg-gradient-to-r from-amber-600 to-amber-700 text-white";
      default:
        return "bg-gray-800 text-gray-300";
    }
  }

  let mounted = false;
  onMount(() => {
    mounted = true;
  });
</script>

<div class="min-h-screen bg-gradient-to-br from-gray-800 to-black px-4 py-8 sm:px-6 lg:px-8">
  <div class="max-w-7xl mx-auto">
    <!-- Header Section -->
    <div class="text-center mb-12">
      <h1 class="text-4xl sm:text-5xl font-bold text-white mb-4">Global Leaderboard</h1>
      <p class="text-gray-400 text-lg max-w-2xl mx-auto">
        Compete with the best players worldwide and climb the ranks to glory!
      </p>
    </div>

    <!-- Filters Section -->
    <div
      class="flex flex-col sm:flex-row justify-between items-center mb-8 space-y-4 sm:space-y-0"
    >
      <div class="flex space-x-2">
        {#each timeFrames as timeFrame}
          <button
            class="px-4 py-2 rounded-md text-sm font-medium transition-colors duration-200 {selectedTimeFrame ===
            timeFrame
              ? 'bg-blue-600 text-white'
              : 'bg-gray-800 text-gray-300 hover:bg-gray-700'}"
            on:click={() => (selectedTimeFrame = timeFrame)}
          >
            {timeFrame}
          </button>
        {/each}
      </div>
      <select
        bind:value={selectedGame}
        class="bg-gray-800 text-gray-300 px-4 py-2 rounded-md text-sm font-medium focus:outline-none focus:ring-2 focus:ring-blue-500"
      >
        {#each gameFilters as game}
          <option value={game}>{game}</option>
        {/each}
      </select>
    </div>

    <!-- Leaderboard Table -->
    <div class="bg-gray-900 rounded-xl shadow-xl overflow-hidden">
      <div class="overflow-x-auto">
        <table class="w-full">
          <thead>
            <tr class="border-b border-gray-800">
              <th class="px-6 py-4 text-left text-sm font-semibold text-gray-300 w-20">Rank</th
              >
              <th class="px-6 py-4 text-left text-sm font-semibold text-gray-300">Player</th>
              <th class="px-6 py-4 text-center text-sm font-semibold text-gray-300 w-24"
                >Rating</th
              >
              <th class="px-6 py-4 text-center text-sm font-semibold text-gray-300 w-32"
                >Win Rate</th
              >
              <th
                class="px-6 py-4 text-center text-sm font-semibold text-gray-300 w-24 whitespace-nowrap"
                >Streak</th
              >
              <th class="px-6 py-4 text-center text-sm font-semibold text-gray-300 w-40"
                >Games</th
              >
            </tr>
          </thead>
          <tbody>
            {#each leaderboardData as player, index}
              <tr
                class="border-b border-gray-800 hover:bg-gray-800/50 transition-colors duration-200 {mounted
                  ? 'animate-fade-in'
                  : ''}"
                style="animation-delay: {index * 100}ms"
              >
                <td class="px-6 py-4 w-20">
                  <div class="flex items-center">
                    <span
                      class="{getRankStyle(
                        player.rank,
                      )} w-8 h-8 rounded-full flex items-center justify-center font-bold"
                    >
                      {player.rank}
                    </span>
                  </div>
                </td>
                <td class="px-6 py-4">
                  <div class="flex items-center space-x-3">
                    <img
                      src={player.avatar}
                      alt={player.username}
                      class="w-10 h-10 rounded-full bg-gray-700 flex-shrink-0"
                    />
                    <div class="min-w-0">
                      <div class="font-medium text-white flex items-center space-x-2">
                        <span class="truncate">{player.username}</span>
                        <div class="flex flex-shrink-0 space-x-1">
                          {#each player.badges as badge}
                            <Icon icon={badge} class="w-5 h-5 {getBadgeColor(badge)}" />
                          {/each}
                        </div>
                      </div>
                      <div class="text-sm text-gray-400 whitespace-nowrap">
                        {player.wins}W {player.losses}L
                      </div>
                    </div>
                  </div>
                </td>
                <td class="px-6 py-4 text-center w-24">
                  <span class="text-blue-400 font-semibold whitespace-nowrap"
                    >{player.rating}</span
                  >
                </td>
                <td class="px-6 py-4 w-32">
                  <div class="flex items-center justify-center">
                    <div
                      class="w-full max-w-[100px] h-2 bg-gray-700 rounded-full overflow-hidden flex-shrink-0"
                    >
                      <div
                        class="h-full bg-green-500 rounded-full transition-all duration-500"
                        style="width: {player.winRate}%"
                      />
                    </div>
                    <span class="ml-2 text-gray-300 whitespace-nowrap">{player.winRate}%</span>
                  </div>
                </td>
                <td class="px-6 py-4 text-center w-24">
                  <span
                    class="px-3 py-1 rounded-full text-sm whitespace-nowrap inline-flex items-center justify-center {player.streak >
                    5
                      ? 'bg-purple-500/20 text-purple-400'
                      : 'bg-gray-800 text-gray-400'}"
                  >
                    {player.streak} 🔥
                  </span>
                </td>
                <td class="px-6 py-4 w-40">
                  <div class="flex flex-wrap justify-center gap-1">
                    {#each player.games as game}
                      <span
                        class="px-2 py-1 rounded-md text-xs bg-gray-800 text-gray-300 whitespace-nowrap"
                      >
                        {game}
                      </span>
                    {/each}
                  </div>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  </div>
</div>

<style>
  .animate-fade-in {
    animation: fadeIn 0.5s ease-out forwards;
    opacity: 0;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
