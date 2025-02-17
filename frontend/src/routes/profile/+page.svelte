<script lang="ts">
  import Icon from "@iconify/svelte";
  import { onMount } from "svelte";
  import AuthGuard from "$lib/components/AuthGuard.svelte";
  import { userStore } from "$lib/stores/user";

  // Mock game data - will be replaced with API calls
  const gameData = {
    level: 42,
    xp: 8750,
    xpToNext: 10000,
    joinDate: "2023-09-15",
    status: "online",
    country: "US",
    countryName: "United States",
    badges: [
      { icon: "ph:crown-simple", label: "Premium Member", color: "text-yellow-400" },
      { icon: "ph:trophy", label: "Tournament Winner", color: "text-blue-400" },
      { icon: "ph:lightning", label: "Fast Player", color: "text-purple-400" },
      { icon: "ph:star", label: "Top Rated", color: "text-green-400" },
    ],
  };

  // Settings form data
  let username = $userStore.user?.username || "";
  let email = $userStore.user?.email || "";
  let oldPassword = "";
  let verificationToken = "";
  let newPassword = "";
  let confirmPassword = "";
  let selectedFile: File | null = null;
  let previewUrl = $userStore.user?.avatar || $userStore.user?.profile_picture || "";
  let passwordStage: "old" | "token" | "new" = "old";
  let showSuccessMessage = false;
  let successMessage = "";

  const stats = [
    { label: "Games Played", value: 1250 },
    { label: "Win Rate", value: "68%" },
    { label: "Tournament Wins", value: 15 },
    { label: "Current Streak", value: 8 },
  ];

  const recentGames = [
    {
      game: "Chess",
      result: "win",
      opponent: "ChessMaster99",
      rating: "+25",
      date: "2h ago",
      icon: "game-icons:chess-knight",
    },
    {
      game: "Tic Tac Toe",
      result: "win",
      opponent: "TicTacPro",
      rating: "+18",
      date: "5h ago",
      icon: "game-icons:tic-tac-toe",
    },
    {
      game: "Connect 4",
      result: "loss",
      opponent: "ConnectMaster",
      rating: "-15",
      date: "1d ago",
      icon: "game-icons:connect",
    },
  ];

  const achievements = [
    {
      icon: "ph:trophy",
      title: "Chess Champion",
      description: "Win 100 chess games",
      progress: 85,
      color: "from-yellow-500 to-amber-600",
    },
    {
      icon: "ph:lightning",
      title: "Speed Demon",
      description: "Win a game in under 1 minute",
      progress: 100,
      color: "from-purple-500 to-pink-600",
    },
    {
      icon: "ph:crown-simple",
      title: "Tournament King",
      description: "Win 5 tournaments",
      progress: 60,
      color: "from-blue-500 to-indigo-600",
    },
    {
      icon: "ph:medal",
      title: "Winning Streak",
      description: "Win 10 games in a row",
      progress: 40,
      color: "from-green-500 to-emerald-600",
    },
  ];

  let selectedTab = "overview";
  let mounted = false;

  onMount(() => {
    mounted = true;
  });

  $: currentUser = $userStore.user;

  function getResultColor(result: string): string {
    return result === "win" ? "text-green-400" : "text-red-400";
  }

  function handleFileSelect(event: Event) {
    const input = event.target as HTMLInputElement;
    if (input.files && input.files[0]) {
      selectedFile = input.files[0];
      previewUrl = URL.createObjectURL(input.files[0]);
    }
  }

  function handlePasswordSubmit() {
    if (passwordStage === "old") {
      // Simulate password verification
      if (oldPassword === "password123") {
        passwordStage = "token";
        // Simulate sending email
        showSuccessMessage = true;
        successMessage = "Verification code sent to your email!";
        setTimeout(() => (showSuccessMessage = false), 3000);
      }
    } else if (passwordStage === "token") {
      // Simulate token verification
      if (verificationToken === "123456") {
        passwordStage = "new";
      }
    } else if (passwordStage === "new") {
      // Simulate password update
      if (newPassword === confirmPassword) {
        passwordStage = "old";
        oldPassword = "";
        verificationToken = "";
        newPassword = "";
        confirmPassword = "";
        showSuccessMessage = true;
        successMessage = "Password updated successfully!";
        setTimeout(() => (showSuccessMessage = false), 3000);
      }
    }
  }

  function handleEmailUpdate() {
    // Simulate email update
    showSuccessMessage = true;
    successMessage = "Verification email sent to new address!";
    setTimeout(() => (showSuccessMessage = false), 3000);
  }

  function handleProfileUpdate() {
    // Simulate profile update
    showSuccessMessage = true;
    successMessage = "Profile updated successfully!";
    setTimeout(() => (showSuccessMessage = false), 3000);
  }
</script>

<AuthGuard>
  <div class="min-h-screen bg-gradient-to-br from-gray-800 to-black px-4 py-8 sm:px-6 lg:px-8">
    <div class="max-w-7xl mx-auto">
      <!-- Profile Header -->
      <div class="relative bg-gray-900 rounded-2xl p-8 mb-8 overflow-hidden">
        <!-- Background Pattern -->
        <div class="absolute inset-0 opacity-10">
          <div class="absolute inset-0 bg-gradient-to-r from-blue-500 to-purple-500"></div>
          {#each Array(50) as _, i}
            <div
              class="absolute w-8 h-8 border-2 border-white/20 rounded-full"
              style="left: {Math.random() * 100}%; top: {Math.random() *
                100}%; transform: scale({0.5 + Math.random()});"
            ></div>
          {/each}
        </div>

        <!-- Profile Content -->
        <div class="relative flex flex-col md:flex-row items-center md:items-start gap-8">
          <!-- Avatar Section -->
          <div class="relative">
            <div class="w-32 h-32 rounded-full overflow-hidden ring-4 ring-blue-500 shadow-xl">
              <img
                src={previewUrl}
                alt={currentUser?.username || "Profile"}
                class="w-full h-full object-cover"
              />
            </div>
            <div
              class="absolute bottom-0 right-0 w-6 h-6 rounded-full bg-green-500 ring-4 ring-gray-900"
            ></div>
          </div>

          <!-- User Info -->
          <div class="flex-grow text-center md:text-left">
            <div class="flex flex-col md:flex-row items-center gap-4 mb-4">
              <h1 class="text-3xl font-bold text-white">{currentUser?.username || "User"}</h1>
              <div class="flex items-center gap-2">
                <img
                  src={`https://flagcdn.com/w20/${gameData.country.toLowerCase()}.png`}
                  alt={gameData.countryName}
                  class="w-5 h-4 rounded-sm shadow-sm"
                  title={gameData.countryName}
                />
                <span class="text-gray-400">{gameData.countryName}</span>
              </div>
              <div class="flex flex-wrap justify-center gap-2">
                {#each gameData.badges as badge}
                  <div
                    class="px-3 py-1 rounded-full bg-gray-800/50 backdrop-blur-sm flex items-center gap-1"
                  >
                    <Icon icon={badge.icon} class="w-4 h-4 {badge.color}" />
                    <span class="text-sm text-gray-300">{badge.label}</span>
                  </div>
                {/each}
              </div>
            </div>

            <!-- Level Progress -->
            <div class="mb-6">
              <div class="flex items-center gap-2 mb-2">
                <span class="text-2xl font-bold text-white">Level {gameData.level}</span>
                <div class="text-sm text-gray-400">
                  {gameData.xp}/{gameData.xpToNext} XP
                </div>
              </div>
              <div class="w-full h-2 bg-gray-700 rounded-full overflow-hidden">
                <div
                  class="h-full bg-gradient-to-r from-blue-500 to-purple-500 transition-all duration-500"
                  style="width: {(gameData.xp / gameData.xpToNext) * 100}%"
                ></div>
              </div>
            </div>

            <!-- Quick Stats -->
            <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
              {#each stats as stat}
                <div class="bg-gray-800/50 backdrop-blur-sm rounded-lg p-4 text-center">
                  <div class="text-2xl font-bold text-white">{stat.value}</div>
                  <div class="text-sm text-gray-400">{stat.label}</div>
                </div>
              {/each}
            </div>
          </div>
        </div>
      </div>

      <!-- Tabs -->
      <div class="mb-8 overflow-hidden">
        <div class="overflow-x-auto scrollbar-none -mx-4 px-4">
          <div class="bg-gray-900 rounded-xl p-1 flex gap-1 whitespace-nowrap w-fit mx-auto">
            {#each ["overview", "achievements", "history", "settings"] as tab}
              <button
                class="min-w-[100px] px-4 py-2 rounded-lg text-sm font-medium transition-colors duration-200 {selectedTab ===
                tab
                  ? 'bg-blue-600 text-white'
                  : 'text-gray-400 hover:text-white hover:bg-gray-800'}"
                on:click={() => (selectedTab = tab)}
              >
                {tab.charAt(0).toUpperCase() + tab.slice(1)}
              </button>
            {/each}
          </div>
        </div>
      </div>

      <!-- Tab Content -->
      {#if selectedTab === "overview"}
        <!-- Recent Games -->
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
          <div class="bg-gray-900 rounded-xl p-6">
            <h2 class="text-xl font-bold text-white mb-6">Recent Games</h2>
            <div class="space-y-4">
              {#each recentGames as game}
                <div
                  class="bg-gray-800/50 backdrop-blur-sm rounded-lg p-4 flex items-center gap-4 transform hover:scale-102 transition-all duration-300"
                >
                  <div
                    class="w-12 h-12 bg-blue-500/20 rounded-lg flex items-center justify-center"
                  >
                    <Icon icon={game.icon} class="w-6 h-6 text-blue-400" />
                  </div>
                  <div class="flex-grow">
                    <div class="flex items-center gap-2">
                      <span class="font-medium text-white">{game.game}</span>
                      <span class="text-sm text-gray-400">vs {game.opponent}</span>
                    </div>
                    <div class="text-sm text-gray-400">{game.date}</div>
                  </div>
                  <div class="text-right">
                    <div class={getResultColor(game.result)}>
                      {game.result.toUpperCase()}
                    </div>
                    <div class="text-sm text-gray-400">{game.rating}</div>
                  </div>
                </div>
              {/each}
            </div>
          </div>

          <!-- Achievements Preview -->
          <div class="bg-gray-900 rounded-xl p-6">
            <h2 class="text-xl font-bold text-white mb-6">Recent Achievements</h2>
            <div class="space-y-4">
              {#each achievements.slice(0, 3) as achievement}
                <div class="bg-gray-800/50 backdrop-blur-sm rounded-lg p-4">
                  <div class="flex items-center gap-4 mb-3">
                    <div
                      class="w-12 h-12 bg-blue-500/20 rounded-lg flex items-center justify-center"
                    >
                      <Icon icon={achievement.icon} class="w-6 h-6 text-blue-400" />
                    </div>
                    <div>
                      <div class="font-medium text-white">{achievement.title}</div>
                      <div class="text-sm text-gray-400">{achievement.description}</div>
                    </div>
                  </div>
                  <div class="w-full h-2 bg-gray-700 rounded-full overflow-hidden">
                    <div
                      class="h-full bg-gradient-to-r {achievement.color} transition-all duration-500"
                      style="width: {achievement.progress}%"
                    ></div>
                  </div>
                  <div class="text-right text-sm text-gray-400 mt-1">
                    {achievement.progress}%
                  </div>
                </div>
              {/each}
            </div>
          </div>
        </div>
      {:else if selectedTab === "achievements"}
        <!-- Full Achievements List -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          {#each achievements as achievement}
            <div
              class="bg-gray-900 rounded-xl p-6 transform hover:scale-102 transition-all duration-300"
            >
              <div class="flex items-center gap-4 mb-4">
                <div
                  class="w-16 h-16 bg-blue-500/20 rounded-xl flex items-center justify-center"
                >
                  <Icon icon={achievement.icon} class="w-8 h-8 text-blue-400" />
                </div>
                <div>
                  <div class="text-xl font-medium text-white">{achievement.title}</div>
                  <div class="text-gray-400">{achievement.description}</div>
                </div>
              </div>
              <div class="w-full h-3 bg-gray-800 rounded-full overflow-hidden">
                <div
                  class="h-full bg-gradient-to-r {achievement.color} transition-all duration-500"
                  style="width: {achievement.progress}%"
                ></div>
              </div>
              <div class="flex justify-between mt-2">
                <span class="text-sm text-gray-400">Progress</span>
                <span class="text-sm text-white">{achievement.progress}%</span>
              </div>
            </div>
          {/each}
        </div>
      {:else if selectedTab === "history"}
        <!-- Full Game History -->
        <div class="bg-gray-900 rounded-xl p-6">
          <div class="space-y-4">
            {#each [...recentGames, ...recentGames] as game}
              <div
                class="bg-gray-800/50 backdrop-blur-sm rounded-lg p-4 flex items-center gap-4 transform hover:scale-102 transition-all duration-300"
              >
                <div
                  class="w-12 h-12 bg-blue-500/20 rounded-lg flex items-center justify-center"
                >
                  <Icon icon={game.icon} class="w-6 h-6 text-blue-400" />
                </div>
                <div class="flex-grow">
                  <div class="flex items-center gap-2">
                    <span class="font-medium text-white">{game.game}</span>
                    <span class="text-sm text-gray-400">vs {game.opponent}</span>
                  </div>
                  <div class="text-sm text-gray-400">{game.date}</div>
                </div>
                <div class="text-right">
                  <div class={getResultColor(game.result)}>
                    {game.result.toUpperCase()}
                  </div>
                  <div class="text-sm text-gray-400">{game.rating}</div>
                </div>
              </div>
            {/each}
          </div>
        </div>
      {:else if selectedTab === "settings"}
        <!-- Settings Form -->
        <div class="bg-gray-900 rounded-xl p-6 max-w-2xl mx-auto">
          {#if showSuccessMessage}
            <div
              class="bg-green-500/20 text-green-400 p-4 rounded-lg mb-6 animate-fade-in flex items-center gap-2"
            >
              <Icon icon="ph:check-circle" class="w-5 h-5" />
              {successMessage}
            </div>
          {/if}

          <div class="space-y-8">
            <!-- Avatar Upload -->
            <div class="flex flex-col items-center gap-4">
              <div class="relative">
                <img
                  src={previewUrl}
                  alt="Profile"
                  class="w-32 h-32 rounded-full object-cover ring-4 ring-blue-500"
                />
                <label
                  class="absolute bottom-0 right-0 w-8 h-8 bg-blue-600 rounded-full flex items-center justify-center cursor-pointer hover:bg-blue-700 transition-colors duration-200"
                >
                  <Icon icon="ph:pencil" class="w-4 h-4 text-white" />
                  <input
                    type="file"
                    accept="image/*"
                    class="hidden"
                    on:change={handleFileSelect}
                  />
                </label>
              </div>
              <span class="text-sm text-gray-400">Click the pencil to change avatar</span>
            </div>

            <!-- Profile Section -->
            <div class="space-y-4">
              <h3 class="text-lg font-semibold text-white">Profile Information</h3>
              <div>
                <label for="username" class="block text-sm font-medium text-gray-400 mb-2"
                  >Username</label
                >
                <input
                  id="username"
                  type="text"
                  bind:value={username}
                  class="w-full px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-white"
                />
              </div>
              <div class="flex justify-end">
                <button
                  on:click={handleProfileUpdate}
                  class="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors duration-300 flex items-center gap-2"
                >
                  <Icon icon="ph:check" class="w-5 h-5" />
                  Save Changes
                </button>
              </div>
            </div>

            <!-- Email Section -->
            <div class="space-y-4 pt-4 border-t border-gray-800">
              <h3 class="text-lg font-semibold text-white">Email Settings</h3>
              <div>
                <div class="block text-sm font-medium text-gray-400 mb-2">Current Email</div>
                <div
                  class="text-gray-300 px-4 py-2 bg-gray-800 rounded-lg border border-gray-700"
                >
                  {email}
                </div>
              </div>
              <div>
                <label for="newEmail" class="block text-sm font-medium text-gray-400 mb-2"
                  >New Email</label
                >
                <input
                  id="newEmail"
                  type="email"
                  placeholder="Enter new email address"
                  class="w-full px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-white"
                />
              </div>
              <div class="flex justify-end">
                <button
                  on:click={handleEmailUpdate}
                  class="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors duration-300 flex items-center gap-2"
                >
                  <Icon icon="ph:envelope" class="w-5 h-5" />
                  Send Verification
                </button>
              </div>
            </div>

            <!-- Password Section -->
            <div class="space-y-4 pt-4 border-t border-gray-800">
              <h3 class="text-lg font-semibold text-white">Change Password</h3>
              {#if passwordStage === "old"}
                <div>
                  <label
                    for="oldPassword"
                    class="block text-sm font-medium text-gray-400 mb-2"
                  >
                    Current Password
                  </label>
                  <input
                    id="oldPassword"
                    type="password"
                    bind:value={oldPassword}
                    class="w-full px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-white"
                    placeholder="Enter your current password"
                  />
                </div>
              {:else if passwordStage === "token"}
                <div>
                  <label
                    for="verificationToken"
                    class="block text-sm font-medium text-gray-400 mb-2"
                  >
                    Verification Code
                  </label>
                  <input
                    id="verificationToken"
                    type="text"
                    bind:value={verificationToken}
                    class="w-full px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-white"
                    placeholder="Enter the code sent to your email"
                  />
                </div>
              {:else}
                <div class="space-y-4">
                  <div>
                    <label
                      for="newPassword"
                      class="block text-sm font-medium text-gray-400 mb-2"
                    >
                      New Password
                    </label>
                    <input
                      id="newPassword"
                      type="password"
                      bind:value={newPassword}
                      class="w-full px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-white"
                      placeholder="Enter new password"
                    />
                  </div>
                  <div>
                    <label
                      for="confirmPassword"
                      class="block text-sm font-medium text-gray-400 mb-2"
                    >
                      Confirm New Password
                    </label>
                    <input
                      id="confirmPassword"
                      type="password"
                      bind:value={confirmPassword}
                      class="w-full px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-white"
                      placeholder="Confirm new password"
                    />
                  </div>
                </div>
              {/if}
              <div class="flex justify-end">
                <button
                  on:click={handlePasswordSubmit}
                  class="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors duration-300 flex items-center gap-2"
                >
                  <Icon icon="ph:arrow-right" class="w-5 h-5" />
                  {passwordStage === "new" ? "Update Password" : "Continue"}
                </button>
              </div>
            </div>
          </div>
        </div>
      {/if}
    </div>
  </div>
</AuthGuard>

<style>
  /* Hide scrollbar but keep functionality */
  .scrollbar-none {
    scrollbar-width: none;
    -ms-overflow-style: none;
  }
  .scrollbar-none::-webkit-scrollbar {
    display: none;
  }

  :global(.hover\:scale-102:hover) {
    transform: scale(1.02);
  }

  .animate-fade-in {
    animation: fadeIn 0.5s ease-out forwards;
    opacity: 0;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
