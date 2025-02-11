<script lang="ts">
  import Icon from "@iconify/svelte";
  import { onMount } from "svelte";

  let email = "";
  let notificationSent = false;
  let mounted = false;

  const upcomingFeatures = [
    {
      icon: "ph:trophy",
      title: "Weekly Tournaments",
      description: "Compete in regular tournaments with players worldwide",
    },
    {
      icon: "ph:medal",
      title: "Prize Pools",
      description: "Win exclusive rewards and premium subscriptions",
    },
    {
      icon: "ph:users-three",
      title: "Team Competitions",
      description: "Form teams and compete in group tournaments",
    },
    {
      icon: "ph:chart-line-up",
      title: "Ranking System",
      description: "Climb the tournament ladder and earn prestigious titles",
    },
  ];

  const launchDate = new Date();
  launchDate.setMonth(launchDate.getMonth() + 1);

  function handleNotifyMe() {
    if (email) {
      // TODO: Implement email notification signup
      notificationSent = true;
      email = "";
    }
  }

  onMount(() => {
    mounted = true;
  });
</script>

<div class="min-h-screen bg-gradient-to-br from-gray-800 to-black px-4 py-8 sm:px-6 lg:px-8">
  <div class="max-w-7xl mx-auto">
    <!-- Hero Section -->
    <div class="text-center mb-16">
      <div class="relative inline-block">
        <h1 class="text-4xl sm:text-5xl lg:text-6xl font-bold text-white mb-6 relative">
          Tournaments
          <span
            class="absolute -top-4 -right-4 bg-blue-500 text-white text-sm px-3 py-1 rounded-full transform rotate-12"
          >
            Coming Soon
          </span>
        </h1>
      </div>
      <p class="text-xl text-gray-300 max-w-2xl mx-auto mb-8">
        Get ready for epic tournaments, amazing prizes, and unforgettable gaming moments!
      </p>

      <!-- Countdown Timer -->
      <div class="flex justify-center gap-4 mb-12">
        <div class="bg-gray-800 px-6 py-4 rounded-xl">
          <span class="block text-3xl font-bold text-blue-400"
            >{Math.floor((launchDate.getTime() - Date.now()) / (1000 * 60 * 60 * 24))}</span
          >
          <span class="text-gray-400">Days</span>
        </div>
        <div class="bg-gray-800 px-6 py-4 rounded-xl">
          <span class="block text-3xl font-bold text-blue-400"
            >{launchDate.toLocaleString("default", { month: "short" })}</span
          >
          <span class="text-gray-400">Month</span>
        </div>
        <div class="bg-gray-800 px-6 py-4 rounded-xl">
          <span class="block text-3xl font-bold text-blue-400">{launchDate.getFullYear()}</span
          >
          <span class="text-gray-400">Year</span>
        </div>
      </div>

      <!-- Notification Signup -->
      <div class="max-w-md mx-auto">
        {#if notificationSent}
          <div class="bg-green-500/20 text-green-400 p-4 rounded-lg mb-8 animate-fade-in">
            <Icon icon="ph:check-circle" class="w-6 h-6 inline-block mr-2" />
            Thanks! We'll notify you when tournaments are live.
          </div>
        {:else}
          <div class="flex gap-2">
            <input
              type="email"
              bind:value={email}
              placeholder="Enter your email"
              class="flex-1 px-4 py-3 bg-gray-800 border border-gray-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-white"
            />
            <button
              on:click={handleNotifyMe}
              class="px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors duration-300 flex items-center whitespace-nowrap"
            >
              <Icon icon="ph:bell" class="w-5 h-5 mr-2" />
              Notify Me
            </button>
          </div>
        {/if}
      </div>
    </div>

    <!-- Features Preview -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-8 mb-16">
      {#each upcomingFeatures as feature, index}
        <div
          class="bg-gray-800/50 rounded-xl p-6 border border-gray-700 backdrop-blur-sm transform hover:scale-105 transition-all duration-300 {mounted
            ? 'animate-fade-in'
            : ''}"
          style="animation-delay: {index * 100}ms"
        >
          <div class="flex items-center mb-4">
            <div class="w-12 h-12 bg-blue-500/20 rounded-lg flex items-center justify-center">
              <Icon icon={feature.icon} class="w-6 h-6 text-blue-400" />
            </div>
            <h3 class="text-xl font-semibold text-white ml-4">{feature.title}</h3>
          </div>
          <p class="text-gray-400">{feature.description}</p>
        </div>
      {/each}
    </div>

    <!-- Tournament Types Preview -->
    <div class="text-center mb-16">
      <h2 class="text-3xl font-bold text-white mb-8">Tournament Types</h2>
      <div class="flex flex-wrap justify-center gap-4">
        {#each ["Solo Ranked", "Team Battle", "Championship", "Weekly Challenge"] as type, index}
          <div
            class="px-6 py-3 bg-gray-800/30 rounded-full text-gray-300 border border-gray-700 backdrop-blur-sm transform hover:scale-105 transition-all duration-300 {mounted
              ? 'animate-fade-in'
              : ''}"
            style="animation-delay: {(index + upcomingFeatures.length) * 100}ms"
          >
            {type}
          </div>
        {/each}
      </div>
    </div>

    <!-- Early Access Banner -->
    <div
      class="bg-gradient-to-r from-blue-600 to-purple-600 rounded-xl p-8 text-center {mounted
        ? 'animate-fade-in'
        : ''}"
      style="animation-delay: {(upcomingFeatures.length + 5) * 100}ms"
    >
      <h3 class="text-2xl font-bold text-white mb-4">Want Early Access?</h3>
      <p class="text-white/90 mb-6">
        Premium members will get exclusive early access to tournaments before the public
        launch!
      </p>
      <a
        href="/premium"
        class="inline-flex items-center px-6 py-3 bg-white text-blue-600 rounded-lg font-semibold hover:bg-gray-100 transition-colors duration-300"
      >
        <Icon icon="ph:crown-simple" class="w-5 h-5 mr-2" />
        Get Premium Access
      </a>
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
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
